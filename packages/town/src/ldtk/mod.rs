use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy_ecs_ldtk::prelude::*;
use crate::building_area::{BuildingAreaBundle, BuildingEntranceBundle};
use crate::ldtk::systems::setup_ldtk;
use crate::starting_point::StartingPointBundle;

mod systems;

pub struct TownLdtkPlugin;

impl Plugin for TownLdtkPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup_ldtk)
      .insert_resource(LevelSelection::Index(0))
      .register_ldtk_entity::<BuildingAreaBundle>("BuildingArea")
      .register_ldtk_entity::<BuildingEntranceBundle>("BuildingEntrance")
      .register_ldtk_entity::<StartingPointBundle>("StartingPoint");
  }
}
