pub mod systems;
pub mod components;
pub mod utils;

use std::prelude::*;
use std::default::Default;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, int_grid_index_to_grid_coords, ldtk_pixel_coords_to_grid_coords, translation_to_grid_coords};
use components::{LookDirection, MovingState};
use crate::collisions;
use crate::collisions::resources::{CollisionIndex, LevelCollisionsSet};
use crate::collisions::systems::set_collisions_to_current_level;
use crate::level_measurements::{LevelMeasurements, set_level_measurements_to_current_level};
use crate::pathfinding::{get_direction, MoveCompulsion, PathCompulsion};
use crate::player::systems::{confine_player_movement, player_animation_system, player_movement_system, respawn_player_system};
use crate::starting_point::{StartingPoint, StartingPointInitialized};

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(respawn_player_system.after(set_level_measurements_to_current_level))
      .add_system(player_movement_system)
      .add_system(player_animation_system)
      .add_system(confine_player_movement); // .after(player_movement_system).after(set_collisions_to_current_level)
  }
}
