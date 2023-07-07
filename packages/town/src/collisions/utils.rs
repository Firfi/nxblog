use bevy::prelude::Res;
use bevy::math::{IVec2, Vec2};
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, int_grid_index_to_grid_coords};
use crate::collisions::resources::CollisionIndex;
use crate::level_measurements::LevelMeasurements;

pub fn translation_from_collision_int(level_measurements: &Res<LevelMeasurements>, collision_index: &CollisionIndex) -> Vec2 {

  let grid_coords = int_grid_index_to_grid_coords(collision_index.0,
                                                  level_measurements.px_wid / level_measurements.grid_size,
                                                  level_measurements.px_hei / level_measurements.grid_size,
  ).expect("grid coords not expected over boards");
  grid_coords_to_translation(grid_coords, IVec2::new(level_measurements.grid_size as i32, level_measurements.grid_size as i32))
}
