use std::collections::{BTreeSet, HashSet};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_ldtk_grid_coords, grid_coords_to_translation, int_grid_index_to_grid_coords, ldtk_grid_coords_to_grid_coords};
use pathfinding::num_traits::Signed;
use crate::building_area::{BuildingAreaTriggered, BuildingEntrance};
extern crate pathfinding;
use pathfinding::prelude::astar;
use crate::collisions::resources::{CollisionIndex, LevelCollisionsSet};
use crate::level_measurements::LevelMeasurements;
use crate::player::components::Player;

fn int_cell_1d_to_2d_index(cell_index: usize, width_in_cells: usize) -> (usize, usize) {
  let x = cell_index % width_in_cells;
  let y = (cell_index - x) / width_in_cells;
  (x, y)
}

#[test]
fn test_int_1d_to_2d() {
  assert_eq!(int_cell_1d_to_2d_index(0, 3), (0, 0));
  assert_eq!(int_cell_1d_to_2d_index(1, 3), (1, 0));
  assert_eq!(int_cell_1d_to_2d_index(2, 3), (2, 0));
  assert_eq!(int_cell_1d_to_2d_index(3, 3), (0, 1));
  assert_eq!(int_cell_1d_to_2d_index(4, 3), (1, 1));
  assert_eq!(int_cell_1d_to_2d_index(5, 3), (2, 1));
  assert_eq!(int_cell_1d_to_2d_index(6, 3), (0, 2));
  assert_eq!(int_cell_1d_to_2d_index(7, 3), (1, 2));
  assert_eq!(int_cell_1d_to_2d_index(8, 3), (2, 2));
}

fn get_neighbour_indices(index: usize, width_in_cells: usize, height_in_cells: usize) -> BTreeSet<usize> {
  let mut neighbours = BTreeSet::new();
  let width = width_in_cells;
  let height = height_in_cells;
  match int_cell_1d_to_2d_index(index, width_in_cells) {
    (x, y) => {
      if x > 0 {
        neighbours.insert(index - 1);
      }
      if x < width - 1 {
        neighbours.insert(index + 1);
      }
      if y > 0 {
        neighbours.insert(index - width);
      }
      if y < height - 1 {
        neighbours.insert(index + width);
      }
    }
  }
  neighbours
}

#[test]
fn test_get_neighbour_indices() {
  assert_eq!(get_neighbour_indices(0, 3, 3), BTreeSet::from([1, 3]));
  assert_eq!(get_neighbour_indices(1, 3, 3), BTreeSet::from([0, 2, 4]));
  assert_eq!(get_neighbour_indices(2, 3, 3), BTreeSet::from([1, 5]));
  assert_eq!(get_neighbour_indices(3, 3, 3), BTreeSet::from([0, 4, 6]));
  assert_eq!(get_neighbour_indices(4, 3, 3), BTreeSet::from([1, 3, 5, 7]));
  assert_eq!(get_neighbour_indices(5, 3, 3), BTreeSet::from([2, 4, 8]));
  assert_eq!(get_neighbour_indices(6, 3, 3), BTreeSet::from([3, 7]));
  assert_eq!(get_neighbour_indices(7, 3, 3), BTreeSet::from([4, 6, 8]));
  assert_eq!(get_neighbour_indices(8, 3, 3), BTreeSet::from([5, 7]));
}


fn int_cell_distance(i1: usize, i2: usize, width_in_cells: usize) -> usize {
  let (x1, y1) = int_cell_1d_to_2d_index(i1, width_in_cells);
  let (x2, y2) = int_cell_1d_to_2d_index(i2, width_in_cells);
  ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

#[test]
fn test_int_cell_distance() {
  // check distance
  assert_eq!(int_cell_distance(0, 0, 3), 0);
  assert_eq!(int_cell_distance(0, 1, 3), 1);
  assert_eq!(int_cell_distance(0, 2, 3), 2);
  assert_eq!(int_cell_distance(0, 3, 3), 1);
  assert_eq!(int_cell_distance(0, 4, 3), 2);
  assert_eq!(int_cell_distance(0, 5, 3), 3);
}

// pub fn int_grid_index_to_grid_coords(
//   index: usize,
//   layer_width_in_tiles: u32,
//   layer_height_in_tiles: u32,
// ) -> Option<GridCoords> {
//   if layer_width_in_tiles * layer_height_in_tiles == 0 {
//     // Checking for potential n mod 0 and n / 0 issues
//     // Also it just doesn't make sense for either of these to be 0.
//     return None;
//   }
//
//   let tile_x = index as u32 % layer_width_in_tiles;
//
//   let inverted_y = (index as u32 - tile_x) / layer_width_in_tiles;
//
//   if layer_height_in_tiles > inverted_y {
//     // Checking for potential subtraction issues.
//     // We don't need to check index >= tile_x because tile_x is defined as index mod n where n
//     // is a natural number.
//     // This means tile_x == index where index < n, and tile_x < index where index >= n.
//
//     Some(ldtk_grid_coords_to_grid_coords(
//       IVec2::new(tile_x as i32, inverted_y as i32),
//       layer_height_in_tiles as i32,
//     ))
//   } else {
//     None
//   }
// }

fn grid_coords_to_int_grid_index(grid_coords: &GridCoords, layer_width_in_tiles: u32,
                                 layer_height_in_tiles: u32,) -> Option<usize> {
  if layer_width_in_tiles * layer_height_in_tiles == 0 {
    // Checking for potential n mod 0 and n / 0 issues
    // Also it just doesn't make sense for either of these to be 0.
    return None;
  }

  match grid_coords_to_ldtk_grid_coords(grid_coords.clone(), layer_height_in_tiles as i32) {
    IVec2 {x: tile_x, y: inverted_y} => {
      if layer_height_in_tiles as i32 <= inverted_y { return None; };
      let index = inverted_y * layer_width_in_tiles as i32 + tile_x;
      Some(usize::try_from(index).expect("Index is too large"))
    }
  }


}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct MoveCompulsion(pub IVec2);

#[test]
fn test_grid_coords_to_int_grid_index() {
  // this one is simple: make sure that int_grid_index_to_grid_coords(grid_coords_to_int_grid_index(x)) == x
  let width: u32 = 3;
  let height: u32 = 3;
  for i in 0..width * height {
    let i = usize::try_from(i).unwrap();
    assert_eq!(int_grid_index_to_grid_coords(i, width, height).and_then(|cs| grid_coords_to_int_grid_index(&cs, width, height)), Some(i));
  }
}

fn pathfind(start: &usize, goal: &usize, collisions: &HashSet<CollisionIndex>, layer_width_in_tiles: &u32, layer_height_in_tiles: &u32) -> Option<(Vec<usize>, usize)> {
  astar(start, |ii|
    get_neighbour_indices(*ii,
                          usize::try_from(*layer_width_in_tiles)
                            .expect("level_measurements.c_wid too large"),
                          usize::try_from(*layer_height_in_tiles)
                            .expect("level_measurements.c_hei too large"))
      .into_iter()
      .filter(|ii| collisions.get(&CollisionIndex(ii.clone())).is_none())
      .map(|v| (v, 1)).collect::<Vec<_>>(),
        |ii| {
          int_cell_distance(*ii, *goal, usize::try_from(*layer_width_in_tiles).expect("grid coords assumed to be valid at this point")) * 3
        },
        |ii| *ii == *goal
  )
}

#[test]
fn test_pathfind() {
  // test pathfind
  let width: u32 = 3;
  let height: u32 = 3;
  let collisions = HashSet::new();
  assert_eq!(pathfind(&0, &8, &collisions, &width, &height), Some((vec![0, 1, 2, 5, 8], 4)));
  // now with some collisions on the way
  let mut collisions = HashSet::new();
  collisions.insert(CollisionIndex(3));
  collisions.insert(CollisionIndex(4));
  // only one valid path
  assert_eq!(pathfind(&0, &6, &collisions, &width, &height), Some((vec![0, 1, 2, 5, 8, 7, 6], 6)));
  // now with collisions fully blocking the way
  let mut collisions = HashSet::new();
  collisions.insert(CollisionIndex(3));
  collisions.insert(CollisionIndex(4));
  collisions.insert(CollisionIndex(5));
  assert_eq!(pathfind(&1, &7, &collisions, &width, &height), None);
  // now we try to patfind INTO a collision
  let mut collisions = HashSet::new();
  collisions.insert(CollisionIndex(3));
  assert_eq!(pathfind(&0, &3, &collisions, &width, &height), None);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct PathCompulsion(pub Vec<usize>);

// returns normalized f32, f32
pub fn get_direction(from: &Vec2, to: &Vec2) -> Vec2 {
  (*to - *from).normalize_or_zero()
}

pub fn pathfinding_system(
  mut event_reader: EventReader<BuildingAreaTriggered>,
  entrance_positional_query: Query<(&GridCoords, &BuildingEntrance)>,
  player_positional_query: Query<(Entity, &GridCoords), With<Player>>,
  collisions: Res<LevelCollisionsSet>,
  level_measurements: Res<LevelMeasurements>,
  mut commands: Commands,
) {
  for e in event_reader.iter() {
    let (player_entity, player_coords) = player_positional_query.iter().next().expect("Player is required at this point");
    let player_i = grid_coords_to_int_grid_index(&player_coords, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
    let entrance = entrance_positional_query.get(e.entrance).expect("Entrance is required at this point");
    let i = grid_coords_to_int_grid_index(entrance.0, level_measurements.c_wid/*TODO probably not right*/, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
    let path = pathfind(&player_i, &i, &collisions.0, &level_measurements.c_wid, &level_measurements.c_hei);
    match path {
      Some((path, _)) => {
        let mut entity_commands = commands.get_entity(player_entity).expect("player entity certainly exists");
        entity_commands.insert(PathCompulsion(path));
      }
      None => {
        println!("No path found");
      }
    }
  }
}

pub fn unroll_path_system(mut commands: Commands, mut compulsion_query: Query<(Entity, &mut PathCompulsion, &GridCoords, &Transform, Option<&MoveCompulsion>)>, level_measurements: Res<LevelMeasurements>) {
  for (e, mut compulsion, coords, transform, move_compulsion) in compulsion_query.iter_mut() {
    let translation = transform.translation.truncate();
    if move_compulsion.is_some() {
      // we're already moving, so we don't need to do anything
      continue;
    }
    match compulsion.0[..] {
      [] => {
        // no path, no compulsion
        commands.entity(e).remove::<PathCompulsion>();
        println!("path unrolled");
        return;
      }
      [i, ..] => {
        let next_coords = int_grid_index_to_grid_coords(i, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
        let next_translation = grid_coords_to_translation(next_coords, IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32));
        if translation == next_translation {
          // we're already on this tile; cut down compulsion
          println!("cutting compulsion");
          *compulsion = PathCompulsion(compulsion.0[1..].to_vec());
          continue;
        }
        commands.entity(e).insert(MoveCompulsion(next_translation.as_ivec2()));
      }
    }
    // cut the compulsion until we reach the next tile; it means we iterate the path removing things until we found the tile we're standing on
    // let mut i = 0;
    // while i < compulsion.0.len() {
    //   if compulsion.0[i] == grid_coords_to_int_grid_index(coords, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point") {
    //     break;
    //   }
    //   i += 1;
    // }
    // if i == compulsion.0.len() {
    //   // we didn't find the tile we're standing on in the path, so we're not compelled to move anywhere
    //   commands.entity(e).remove::<PathCompulsion>();
    //   println!("path unrolled");
    //   return;
    // } else {
    //   // we found the tile we're standing on in the path; cut all the previous one
    //   compulsion.0 = compulsion.0.split_off(i); // the tile we're standing on is kept
    // }
    // let mut entity_commands = commands.get_entity(e).expect("entity certainly exists");
    // // now if it's the last tile, we must complete our compulsion until the middle of the tile
    // // and if we're in the middle of the last tile finally, we can remove the compulsion entirely
    // match compulsion.0[..] {
    //   [] => panic!("compulsion isn't empty at this point"),
    //   [last_tile] => {
    //     let last_tile_coords = int_grid_index_to_grid_coords(last_tile, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
    //     let last_tile_translation = grid_coords_to_translation(last_tile_coords, IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32));
    //     let direction = get_direction(&translation, &last_tile_translation);
    //     if direction == Vec2::ZERO {
    //       // goal reached
    //       commands.entity(e).remove::<PathCompulsion>();
    //     } else {
    //       // we're in the middle of the last tile, so we must complete our compulsion until the middle of the tile
    //       entity_commands.insert(MoveCompulsion(direction));
    //     }
    //   }
    //   // at least two tiles left
    //   [current_tile, next_tile, ..] => {
    //     let current_tile_coords = int_grid_index_to_grid_coords(current_tile, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
    //     println!("current_tile_coords: {:?}", current_tile_coords);
    //     let current_tile_translation = grid_coords_to_translation(current_tile_coords, IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32));
    //     println!("current_tile_translation: {:?}", current_tile_translation);
    //     println!("my translation: {:?}", translation);
    //     if current_tile_translation != translation {
    //       // we're not in the middle of the tile, so we must complete our compulsion until the middle of the tile
    //       let direction = get_direction(&translation, &current_tile_translation);
    //       println!("direction: {:?}", direction);
    //       entity_commands.insert(MoveCompulsion(direction));
    //       return;
    //     }
    //     println!("in the middle!");
    //     let next_tile_coords = int_grid_index_to_grid_coords(next_tile, level_measurements.c_wid, level_measurements.c_hei).expect("grid coords assumed to be valid at this point");
    //     let next_tile_translation = grid_coords_to_translation(next_tile_coords, IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32));
    //     let direction = get_direction(&translation, &next_tile_translation);
    //     entity_commands.insert(MoveCompulsion(direction));
    //   }

  }
}
