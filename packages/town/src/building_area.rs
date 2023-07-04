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

pub fn building_area_cursor_system(transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>,
                                   cursor_position_res: Res<CursorPos>,

  mut event_writer: EventWriter<BuildingAreaTriggered>
) {
  for entrance in transforms_query.iter().filter(|e| {
    let gpxx_min = e.0.px.x;
    let gpxx_max = gpxx_min + e.0.width;
    let gpxy_min = e.0.px.y;
    let gpxy_max = gpxy_min + e.0.height;
    cursor_position_res.0.x > gpxx_min && cursor_position_res.0.x < gpxx_max
      && cursor_position_res.0.y > gpxy_min && cursor_position_res.0.y < gpxy_max
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
