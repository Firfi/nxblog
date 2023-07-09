use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::level_positional::LevelPositional;

#[derive(Debug, Default, Component)]
pub struct StartingPoint;

pub struct StartingPointInitialized {
  pub grid_coords: GridCoords,
}

#[derive(Bundle, LdtkEntity)]
pub struct StartingPointBundle {
  #[with(LevelPositional::from_entity_field)]
  positional: LevelPositional,
  #[grid_coords]
  grid_coords: GridCoords,
  starting_point: StartingPoint,
}

pub fn starting_point_system(
  mut level_events: EventReader<LevelEvent>,
  starting_point_query: Query<&GridCoords, With<StartingPoint>>,
  mut event_writer: EventWriter<StartingPointInitialized>) {
  for level_event in level_events.iter().filter(|e| matches!(e, LevelEvent::Transformed(_)) ) {
    event_writer.send(StartingPointInitialized {
      grid_coords: starting_point_query.get_single().expect("Only one starting point expected").clone()
    })
  }
}
