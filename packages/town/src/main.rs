mod building_entrance_ref;
mod level_measurements;
mod level_positional;
mod starting_point;
mod level;
mod cursor;
mod pathfinding;
mod building_area;
mod player;
mod collisions;
mod ldtk;

use wasm_bindgen::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use building_entrance_ref::*;
use level_positional::*;
use starting_point::*;

use bevy::{prelude::*, winit::WinitSettings};
use bevy::log::Level;
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::building_area::{building_area_cursor_system, building_area_touches_system, BuildingAreaBundle, BuildingAreaTriggered, BuildingEntranceBundle, UrlPath};
use crate::collisions::CollisionsPlugin;
use crate::cursor::{CursorPos, update_cursor_pos};
use crate::ldtk::TownLdtkPlugin;
use crate::level_measurements::{LevelMeasurements, set_level_measurements_to_current_level};
use crate::pathfinding::{pathfinding_system, unroll_path_system};
use crate::player::PlayerPlugin;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = window)]
  fn go_town(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, wasm-game-of-life!");
  main();
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .insert_resource(WinitSettings::game())
    .add_startup_system(setup)
    .add_plugin(PlayerPlugin)
    .add_plugin(CollisionsPlugin)
    .add_plugin(TownLdtkPlugin)
    .add_system(starting_point_system)
    .add_system(resolve_building_entrance_references)
    .add_system(update_cursor_pos)
    .add_system(set_level_measurements_to_current_level)
    // .add_startup_system(button_setup)
    // .add_system(button_interaction_system)
    .insert_resource(LevelSelection::Index(0))
    .init_resource::<CursorPos>()
    .init_resource::<LevelMeasurements>()
    .add_system(building_area_cursor_system)
    .add_system(building_area_touches_system)
    .add_system(pathfinding_system)
    .add_system(unroll_path_system)
    // not needed; for egui inspector
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .add_event::<StartingPointInitialized>()
    .add_event::<BuildingAreaTriggered>()
    .run();
}

fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2dBundle::default());

}


