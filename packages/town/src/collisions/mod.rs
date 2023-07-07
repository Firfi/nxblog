use std::collections::HashSet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use itertools::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, int_grid_index_to_grid_coords};
use crate::collisions::events::CollisionsInitialized;
use crate::collisions::resources::LevelCollisionsSet;
use crate::collisions::systems::set_collisions_to_current_level;
use crate::level::level_utils::with_level_asset;
use crate::level_measurements::LevelMeasurements;

pub mod utils;
pub mod systems;
pub mod events;
pub mod resources;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
  fn build(&self, app: &mut App) {
    // .add_system(draw_debug_collisions)
    app.init_resource::<LevelCollisionsSet>()
      .add_system(set_collisions_to_current_level)
      .add_event::<CollisionsInitialized>()
    ;
  }
}
