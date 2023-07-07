use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::building_entrance_ref::{BuildingEntranceRef, UnresolvedBuildingEntranceRef};
use crate::cursor::CursorPos;
use crate::level_positional::LevelPositional;

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
  transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>
) {
  for finger in touches.iter() {
    for entrance in transforms_query.iter().filter(|e| {
      let position = finger.position();
      inside_level_positional(&e.0, &position.as_ivec2())
    }).map(|e| e.1) {
      event_writer.send(BuildingAreaTriggered {
        entrance: entrance.0
      });
    }
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
