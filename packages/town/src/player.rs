use std::prelude::*;
use std::default::Default;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, int_grid_index_to_grid_coords, ldtk_pixel_coords_to_grid_coords, translation_to_grid_coords};
use crate::collisions::{CollisionIndex, LevelCollisionsSet};
use crate::level_measurements::LevelMeasurements;
use crate::pathfinding::{get_direction, MoveCompulsion, PathCompulsion};
use crate::starting_point::{StartingPoint, StartingPointInitialized};

pub const PLAYER_SPEED: f32 = 50.0;
pub const COMPULSED_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player;

pub fn respawn_player_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut event_reader: EventReader<StartingPointInitialized>,
  level_measurements: Res<LevelMeasurements>
) {
  let texture_handle: Handle<Image> = asset_server.load(PLAYER_ANIMATION_SPRITE_PATH);
  let tilemap_metadata = PLAYER_ANIMATION_SPRITE_METADATA;

  let tilemap_entity = commands.spawn_empty().id();
  for start_point in event_reader.iter() {
    let transform = Transform::from_translation(Vec3::from((
      grid_coords_to_translation(
        start_point.grid_coords.clone(),
        IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32)),
      10.0/*TODO relative to world/level/layers*/)));
    let tile_pos = TilePos::new(0, 0);
    let tile_entity = commands.spawn((
      TileBundle {
        position: tile_pos,
        tilemap_id: TilemapId(tilemap_entity),
        texture_index: TileTextureIndex(0), // TODO
        ..Default::default()
      },
      AnimatedTile {
        start: 0,
        end: 2,
        speed: 0.95,
      },
    ));
    let mut tile_storage = TileStorage::empty(tilemap_metadata.size);
    tile_storage.set(&tile_pos, tile_entity.id());
    let map_type = TilemapType::Square;
    commands.entity(tilemap_entity).insert(
      (
        TilemapBundle {
          spacing: TilemapSpacing {x: 32.0, y: 32.0},
          size: tilemap_metadata.size,
          grid_size: tilemap_metadata.grid_size,
          map_type,
          tile_size: tilemap_metadata.tile_size,
          storage: tile_storage,
          texture: TilemapTexture::Single(texture_handle.clone()),
          transform,
          ..Default::default()
        },
        // SpriteBundle {
        //   transform: Transform::from_translation(Vec3::from((
        //     grid_coords_to_translation(
        //       start_point.grid_coords.clone(),
        //       IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32)),
        //     10.0/*TODO relative to world/level/layers*/))),
        //   texture: asset_server.load("sprites/star.png"),
        //   sprite: Sprite {
        //     custom_size: Some(Vec2::new(16.0, 16.0)),
        //     ..default()
        //   },
        //   ..default()
        // },
        AnimationStateBundle::default(),
        Player,
        start_point.grid_coords.clone(),
      )
    );
    // e.set_parent()
  }
}

#[derive(Component)]
enum MovingState {
  Idle,
  Moving
}

#[derive(Component)]
enum LookDirection {
  Up,
  Down,
  Left,
  Right
}

#[derive(Bundle)]
pub struct AnimationStateBundle {
  moving_state: MovingState,
  look_direction: LookDirection
}

impl Default for AnimationStateBundle {
  fn default() -> Self {
    Self {
      moving_state: MovingState::Idle,
      look_direction: LookDirection::Up
    }
  }
}

struct TilemapMetadata {
  size: TilemapSize,
  tile_size: TilemapTileSize,
  grid_size: TilemapGridSize,
  gap: usize
}

const PLAYER_ANIMATION_SPRITE_PATH: &str = "atlas/sprout-lands-basic-character-sheet.png";
const PLAYER_ANIMATION_SPRITE_METADATA: TilemapMetadata = TilemapMetadata {
  size: TilemapSize { x: 16, y: 16 },
  tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
  grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
  gap: 16
};

pub fn player_movement(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform, &mut GridCoords, Option<&MoveCompulsion>, Entity), With<Player>>,
  time: Res<Time>,
  level_measurements: Res<LevelMeasurements>,
  mut commands: Commands
) {
  for (mut transform, mut grid_coords, move_compulsion, entity) in query.iter_mut() {
    let translation = transform.translation.truncate();
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
      direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
      direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
      direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
      direction.y -= 1.0;
    }

    fn get_translation_diff(direction: Vec3, time: &Time, speed: f32) -> Vec3 {
      direction * speed * time.delta_seconds()
    }

    fn signum(vec: Vec3) -> Vec3 {
      Vec3::from((vec.x.signum(), vec.y.signum(), vec.z.signum()))
    }
    let mut speed = PLAYER_SPEED;
    match move_compulsion {
      Some(compulsion) => {
        if direction != Vec3::ZERO {
          // don't thread on me
          commands.entity(entity).remove::<MoveCompulsion>();
          commands.entity(entity).remove::<PathCompulsion>();
        } else {
          // compulsed movement is faster
          speed = COMPULSED_SPEED;
          direction = Vec3::from((get_direction(&translation, &compulsion.0.as_vec2()), 0.0));
          let translation_to_be = transform.translation + get_translation_diff(direction, &time, speed);
          let direction_to_be = Vec3::from((get_direction(&translation_to_be.truncate(), &compulsion.0.as_vec2()), 0.0));
          if signum(direction_to_be) != signum(direction) { // overshoot; we DID pass the
            let sticky = Vec3::from((compulsion.0.as_vec2(), transform.translation.z));
            // forcefully stick the player to the coords
            transform.translation = sticky;
            commands.entity(entity).remove::<MoveCompulsion>();
            return;
          }
        }

      },
      None => {}
    };

    direction = direction.normalize_or_zero();

    transform.translation += get_translation_diff(direction, &time, speed);
    update_player_grid_coords(&mut grid_coords, &transform, &level_measurements);

  }
}

pub fn translation_from_collision_int(level_measurements: &Res<LevelMeasurements>, collision_index: &CollisionIndex) -> Vec2 {

  let grid_coords = int_grid_index_to_grid_coords(collision_index.0,
                                level_measurements.px_wid / level_measurements.grid_size,
                                level_measurements.px_hei / level_measurements.grid_size,
  ).expect("grid coords not expected over boards");
  grid_coords_to_translation(grid_coords, IVec2::new(level_measurements.grid_size as i32, level_measurements.grid_size as i32))
}

pub fn confine_player_movement(
  mut player_query: Query<(&mut Transform, &mut GridCoords), With<Player>>,
  level_measurements: Res<LevelMeasurements>,
  level_collisions: Res<LevelCollisionsSet>,
) {
  let player_size = level_measurements.c_wid; // assume = cell height
  for (mut transform, mut grid_coords) in player_query.iter_mut() {
    let half_player_size = player_size / 2;
    let x_min = 0 + half_player_size;
    let x_max = (level_measurements.grid_size * level_measurements.c_wid) - half_player_size;
    let y_min = 0 + half_player_size;
    let y_max = (level_measurements.grid_size * level_measurements.c_hei)  - half_player_size;

    let translation = &mut transform.translation;

    if translation.x < x_min as f32 {
      translation.x = x_min as f32;
    }
    if translation.x > x_max as f32 {
      translation.x = x_max as f32;
    }
    if translation.y < y_min as f32 {
      translation.y = y_min as f32;
    }
    if translation.y > y_max as f32 {
      translation.y = y_max as f32;
    }

    for collision in level_collisions.0.iter() {

      let collision_translation = translation_from_collision_int(&level_measurements, &collision);
      let collision_x_min = collision_translation.x - (level_measurements.grid_size / 2) as f32;
      let collision_x_max = collision_translation.x + (level_measurements.grid_size / 2) as f32;
      let collision_y_min = collision_translation.y - (level_measurements.grid_size / 2) as f32;
      let collision_y_max = collision_translation.y + (level_measurements.grid_size / 2) as f32;

      let player_x_min = translation.x - half_player_size as f32;
      let player_x_max = translation.x + half_player_size as f32;
      let player_y_min = translation.y - half_player_size as f32;
      let player_y_max = translation.y + half_player_size as f32;

      // squares intersect // TODO what when > 1 collision? - take largest intersection, only consider it
      if player_x_min < collision_x_max && player_x_max > collision_x_min && player_y_min < collision_y_max && player_y_max > collision_y_min {
        // calculate diff of how deep the player is in and roll back exactly that much
        // i.e. player got from the top = roll exactly the top diff, etc.
        let mut x_diff = 0.0;
        let mut y_diff = 0.0;
        // from the left
        if player_x_max < collision_x_max {
          x_diff = player_x_max - collision_x_min;
        // from the right
        } else if player_x_max > collision_x_max {
          x_diff = player_x_min - collision_x_max;
        } else {
          // corner case when player can slide into the tile perfectly...
          x_diff = player_size as f32; // max
        }
        // from the top
        if player_y_max < collision_y_max {
          y_diff = player_y_max - collision_y_min;
        // from the bottom
        } else if player_y_max > collision_y_max {
          y_diff = player_y_min - collision_y_max;
        } else {
          // corner case when player can slide into the tile perfectly...
          y_diff = player_size as f32; // max
        }

        if x_diff.abs() < y_diff.abs() {
          translation.x -= x_diff;
        } else {
          translation.y -= y_diff;
        }


      }
    }

    update_player_grid_coords(&mut grid_coords, &transform, &level_measurements);
  }
}

pub fn update_player_grid_coords(grid_coords: &mut GridCoords, transform: &Transform, level_measurements: &Res<LevelMeasurements>) {
  *grid_coords = translation_to_grid_coords(transform.translation.truncate(), IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32));
}
