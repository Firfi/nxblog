use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::building_entrance_ref::{BuildingEntranceRef, UnresolvedBuildingEntranceRef};
use crate::cursor::{CursorPos, world_from_viewport};
use crate::level_positional::LevelPositional;
use crate::player::components::Player;
use wasm_bindgen::prelude::*;
use crate::alert;
use crate::OutsideWindowSize;
use crate::level_measurements::LevelMeasurements;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = window)]
  fn go_town(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn go_town(s: &str) {
  println!("go_town stub: {}", s);
}

pub struct BuildingAreaTriggered {
  pub entrance: Entity
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

pub fn building_area_touches_system(
  touches: Res<Touches>,
  mut event_writer: EventWriter<BuildingAreaTriggered>,
  transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>,
  camera_q: Query<(&GlobalTransform, &Camera)>,
  level_measurements: Res<LevelMeasurements>,
  outside_window_size: Res<OutsideWindowSize>
) {
  for finger in touches.iter_just_pressed() {
    let position_ = finger.position() - outside_window_size.0;
    // let (global_transform, camera) = camera_q.single();
    // let position = world_from_viewport(&position_, global_transform, camera, &level_measurements);
    //if let Some(pos) = position {
    let pos = position_;
      for entrance in transforms_query.iter().filter(|e| {
        inside_level_positional(&e.0, &pos.as_ivec2())
      }).map(|e| e.1) {
        event_writer.send(BuildingAreaTriggered {
          entrance: entrance.0
        });
      }
    //}

  }

}

fn inside_level_positional(level_positional: &LevelPositional, xy: &IVec2) -> bool {
  let gpxx_min = level_positional.px.x;
  let gpxx_max = gpxx_min + level_positional.width;
  let gpxy_min = level_positional.px.y;
  let gpxy_max = gpxy_min + level_positional.height;
  xy.x > gpxx_min && xy.x < gpxx_max
    && xy.y > gpxy_min && xy.y < gpxy_max
}

pub fn building_area_cursor_system(transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>,
                                   cursor_position_res: Res<CursorPos>,
                                   buttons: Res<Input<MouseButton>>,
  mut event_writer: EventWriter<BuildingAreaTriggered>
) {
  if !buttons.just_pressed(MouseButton::Left) {
    return;
  }
  for entrance in transforms_query.iter().filter(|e| {
    inside_level_positional(&e.0, &cursor_position_res.0)
  }).map(|e| e.1) {
    event_writer.send(BuildingAreaTriggered {
      entrance: entrance.0
    });
  }
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
pub struct InsideEntrance(pub bool);

pub fn building_entrance_trigger_system(
  entrance_query: Query<(&LevelPositional, &UrlPath), With<BuildingEntrance>>,
  mut player_query: Query<(&Transform, &mut InsideEntrance), With<Player>>,
  level_measurements: Res<LevelMeasurements>,
) {
  for (player_transform, mut inside_entrance) in player_query.iter_mut() {
    let mut some_inside = false;
    for (entrance_positional, url_path) in entrance_query.iter() {
      if inside_level_positional(&entrance_positional, &translation_to_ldtk_pixel_coords(player_transform.translation.truncate(), level_measurements.px_hei as i32)) {
        some_inside = true;
        if inside_entrance.0 {
          continue;
        }
        go_town(&url_path.0);
      }
    }
    inside_entrance.0 = some_inside;
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
