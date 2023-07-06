mod building_entrance_ref;
mod level_measurements;
mod level_positional;
mod starting_point;
mod player;
mod collisions;
mod level;
mod cursor;
mod pathfinding;
mod building_area;

use wasm_bindgen::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use building_entrance_ref::*;
use level_positional::*;
use starting_point::*;

use bevy::{prelude::*, winit::WinitSettings};
use bevy::log::Level;
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::building_area::{building_area_cursor_system, BuildingAreaBundle, BuildingAreaTriggered, BuildingEntranceBundle, UrlPath};
use crate::collisions::{LevelCollisionsSet, set_collisions_to_current_level, CollisionsInitialized, draw_debug_collisions};
use crate::cursor::{CursorPos, update_cursor_pos};
use crate::level_measurements::{LevelMeasurements, set_level_measurements_to_current_level};
use crate::pathfinding::{pathfinding_system, unroll_path_system};
use crate::player::{confine_player_movement, player_animation_system, player_movement_system, respawn_player_system};

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
    .add_startup_system(setup_ldtk)
    .add_system(starting_point_system)
    .add_system(resolve_building_entrance_references)
    .add_system(update_cursor_pos)
    .add_system(set_level_measurements_to_current_level)
    // .add_startup_system(button_setup)
    // .add_system(button_interaction_system)
    .insert_resource(LevelSelection::Index(0))
    .init_resource::<CursorPos>()
    .init_resource::<LevelMeasurements>()
    .init_resource::<LevelCollisionsSet>()
    .register_ldtk_entity::<BuildingAreaBundle>("BuildingArea")
    .register_ldtk_entity::<BuildingEntranceBundle>("BuildingEntrance")
    .register_ldtk_entity::<StartingPointBundle>("StartingPoint")
    .add_system(building_area_cursor_system)
    .add_system(respawn_player_system.after(set_level_measurements_to_current_level))
    .add_system(player_movement_system)
    .add_system(player_animation_system)
    .add_system(confine_player_movement.after(player_movement_system).after(set_collisions_to_current_level))
    .add_system(set_collisions_to_current_level)
    // .add_system(draw_debug_collisions)
    .add_system(pathfinding_system)
    .add_system(unroll_path_system)
    // not needed; for egui inspector
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .add_event::<StartingPointInitialized>()
    .add_event::<CollisionsInitialized>()
    .add_event::<BuildingAreaTriggered>()
    .run();
}

// fn ls2 (q: Query<&UrlPath>) {
//   for e in q.iter() {
//     println!("path.. {}", e.0);
//   }
// }



fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2dBundle::default());

}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
  let bundle = LdtkWorldBundle {
    ldtk_handle: asset_server.load("ldtk/town.ldtk"),
    ..Default::default()
  };
  commands.spawn(bundle);
}


