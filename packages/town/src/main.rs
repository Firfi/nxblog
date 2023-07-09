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

use bevy::{prelude::*, winit::WinitSettings, input::touch::*};
use bevy::log::Level;
use bevy::prelude::CoreSet::Update;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::building_area::{building_area_cursor_system, building_area_touches_system, building_entrance_trigger_system, BuildingAreaBundle, BuildingAreaTriggered, BuildingEntranceBundle, UrlPath};
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
pub fn greet(canvas_id: &str) {
  init(Some(canvas_id));
}

static mut outside_pos: Option<Vec2> = None;
static mut outside_focus: Option<bool> = None;

#[wasm_bindgen]
pub fn report_canvas_screen_position(x: f32, y: f32) {
  let v = Vec2::new(x, y);
  unsafe {
    outside_pos = Some(v);
  }
}

#[wasm_bindgen]
pub fn report_window_focus(b: bool) {
  unsafe {
    outside_focus = Some(b);
  }
}

#[derive(Debug, Default, Resource)]
pub struct OutsideWindowSize(pub Vec2);

#[derive(Debug, Default, Resource)]
pub struct OutsideWindowFocus(pub bool);

// https://github.com/bevyengine/bevy/issues/9071
fn outside_window_size_system(mut event_writer: EventWriter<OutsideWindowResize>, mut size: ResMut<OutsideWindowSize>) {
  unsafe {
    if let Some(op) = outside_pos {
      if op != size.0 {
        size.0 = op;
        event_writer.send(OutsideWindowResize(op));
      }
    }
  }
}

// https://github.com/bevyengine/bevy/issues/2068
fn outside_window_focus_system(mut focus: ResMut<OutsideWindowFocus>) {
  unsafe {
    if let Some(b) = outside_focus {
      focus.0 = b;
    }
  }
}

fn init(canvas_id: Option<&str>) {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        canvas: canvas_id.map(|id| "#".to_string() + id),
        // can hardcode it as we know the size of the map beforehand
        resolution: WindowResolution::from((256.0, 256.0)),
        title: "Url Town".to_string(),
        ..default()
      }),
      ..default()
    }))
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

    .init_resource::<CursorPos>()
    .init_resource::<LevelMeasurements>()
    .init_resource::<OutsideWindowSize>()
    .init_resource::<OutsideWindowFocus>()
    .add_system(building_area_cursor_system)
    .add_system(building_area_touches_system)
    .add_system(building_entrance_trigger_system)
    .add_system(pathfinding_system)
    .add_system(unroll_path_system)
    .add_system(outside_window_size_system)
    .add_system(outside_window_focus_system)
    // not needed; for egui inspector
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .add_event::<StartingPointInitialized>()
    .add_event::<BuildingAreaTriggered>()
    .add_event::<OutsideWindowResize>().run();
  report_window_focus(true);
}

#[derive(Debug, Clone, Copy)]
pub struct OutsideWindowResize(pub Vec2);

fn main() {
  init(None);
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
  let window = window_query.single();
  // ui camera
  commands.spawn(Camera2dBundle {
    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 11.0),
    ..default()
  });

}


