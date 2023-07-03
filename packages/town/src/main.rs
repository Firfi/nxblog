mod building_entrance_ref;
mod level_measurements;
mod level_positional;
mod starting_point;
mod player;
mod collisions;
mod level;

use wasm_bindgen::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use building_entrance_ref::*;
use level_positional::*;
use starting_point::*;

use bevy::{prelude::*, winit::WinitSettings};
use bevy::log::Level;
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::collisions::{LevelCollisions, set_collisions_to_current_level, CollisionsInitialized, draw_debug_collisions};
use crate::level_measurements::{LevelMeasurements, set_level_measurements_to_current_level};
use crate::player::{confine_player_movement, player_movement, respawn_player_system};

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
    .init_resource::<LevelCollisions>()
    .register_ldtk_entity::<BuildingAreaBundle>("BuildingArea")
    .register_ldtk_entity::<BuildingEntranceBundle>("BuildingEntrance")
    .register_ldtk_entity::<StartingPointBundle>("StartingPoint")
    .add_system(ls)
    .add_system(respawn_player_system.after(set_level_measurements_to_current_level))
    .add_system(player_movement)
    .add_system(confine_player_movement.after(player_movement).after(set_collisions_to_current_level))
    .add_system(set_collisions_to_current_level)
    .add_system(draw_debug_collisions)
    // not needed; for egui inspector
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .add_event::<StartingPointInitialized>()
    .add_event::<CollisionsInitialized>()
    .run();
}

fn ls (transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>,
  cursor_position_res: Res<CursorPos>,
  entrance_positional_query: Query<(&GridCoords, &BuildingEntrance)>
) {
  for entrance in transforms_query.iter().filter(|e| {
    let gpxx_min = e.0.px.x;
    let gpxx_max = gpxx_min + e.0.width;
    let gpxy_min = e.0.px.y;
    let gpxy_max = gpxy_min + e.0.height;
    cursor_position_res.0.x > gpxx_min && cursor_position_res.0.x < gpxx_max
      && cursor_position_res.0.y > gpxy_min && cursor_position_res.0.y < gpxy_max
  }).map(|e| e.1) {
    let entrance = entrance_positional_query.get(entrance.0).expect("Entrance is required at this point");
    println!("tile to pathfind: {:?}", entrance.0);
  }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CursorPos(pub IVec2);
impl Default for CursorPos {
  fn default() -> Self {
    // Initialize the cursor pos at some far away place. It will get updated
    // correctly when the cursor moves.
    Self(IVec2::new(-1000, -1000))
  }
}

pub fn update_cursor_pos(
  camera_q: Query<(&GlobalTransform, &Camera)>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mut cursor_pos: ResMut<CursorPos>,
  level_measurements: Res<LevelMeasurements>
) {
  for cursor_moved in cursor_moved_events.iter() {
    for (cam_t, cam) in camera_q.iter() {
      if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
        **cursor_pos = translation_to_ldtk_pixel_coords(pos, level_measurements.px_hei as i32);
      }
    }
  }
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

#[derive(Debug, Default, Component)]
pub struct BuildingArea;

#[derive(Bundle, LdtkEntity)]
pub struct BuildingAreaBundle {
  #[with(UnresolvedBuildingEntranceRef::from_building_entrance_field)]
  unresolved_building_entrance: UnresolvedBuildingEntranceRef,
  #[with(LevelPositional::from_entity_field)]
  positional: LevelPositional,
  #[grid_coords]
  grid_coords: GridCoords,
  building_area: BuildingArea,
  // #[sprite_sheet_bundle]
  // #[bundle]
  // sprite_bundle: SpriteSheetBundle,
}



#[derive(Debug, Default, Component, Reflect)]
pub struct UrlPath(String);

impl UrlPath {
  pub fn from_field(entity_instance: &EntityInstance) -> UrlPath {
    UrlPath(
      entity_instance
        .get_string_field("path")
        .expect("expected entity to have non-nullable path str field").to_string(),
    )
  }
}

#[derive(Debug, Default, Component)]
pub struct BuildingEntrance;

#[derive(Bundle, LdtkEntity)]
pub struct BuildingEntranceBundle {
  #[with(UrlPath::from_field)]
  path: UrlPath,
  #[with(LevelPositional::from_entity_field)]
  positional: LevelPositional,
  #[grid_coords]
  grid_coords: GridCoords,
  building_entrance: BuildingEntrance,
}
