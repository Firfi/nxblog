use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, int_grid_index_to_grid_coords, ldtk_pixel_coords_to_grid_coords, translation_to_grid_coords};
use crate::collisions::{CollisionIndex, LevelCollisionsSet};
use crate::level_measurements::LevelMeasurements;
use crate::starting_point::{StartingPoint, StartingPointInitialized};

pub const PLAYER_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Player;

pub fn respawn_player_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut event_reader: EventReader<StartingPointInitialized>,
  level_measurements: Res<LevelMeasurements>
) {
  for start_point in event_reader.iter() {
    let mut e = commands.spawn(
      (
        SpriteBundle {
          transform: Transform::from_translation(Vec3::from((
            grid_coords_to_translation(
              start_point.grid_coords.clone(),
              IVec2::new(level_measurements.c_wid as i32, level_measurements.c_hei as i32)),
            10.0/*TODO relative to world/level/layers*/))),
          texture: asset_server.load("sprites/star.png"),
          sprite: Sprite {
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
          },
          ..default()
        },
        Player,
        start_point.grid_coords.clone(),
      )
    );
    // e.set_parent()
  }
}

pub fn player_movement(
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&mut Transform, &mut GridCoords), With<Player>>,
  time: Res<Time>,
  level_measurements: Res<LevelMeasurements>
) {
  for (mut transform, mut grid_coords) in query.iter_mut() {
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

    direction = direction.try_normalize().unwrap_or_else(|| direction);

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
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
