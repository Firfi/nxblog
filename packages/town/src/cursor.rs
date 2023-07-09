use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_ldtk_pixel_coords;
use crate::level_measurements::LevelMeasurements;

#[derive(Resource, Deref, DerefMut)]
pub struct CursorPos(pub IVec2);
impl Default for CursorPos {
  fn default() -> Self {
    // Initialize the cursor pos at some far away place. It will get updated
    // correctly when the cursor moves.
    Self(IVec2::new(-1000, -1000))
  }
}

pub fn world_from_viewport(
  pos: &Vec2,
  camera_transform: &GlobalTransform,
  camera: &Camera,
  level_measurements: &LevelMeasurements
) -> Option<IVec2> {
  camera.viewport_to_world_2d(camera_transform, pos.clone()).map(|pos| {
    translation_to_ldtk_pixel_coords(pos, level_measurements.px_hei as i32)
  })
}

pub fn update_cursor_pos(
  mut cursor_moved_events: EventReader<CursorMoved>,
  mut cursor_pos: ResMut<CursorPos>,
  camera_q: Query<(&GlobalTransform, &Camera)>,
  level_measurements: Res<LevelMeasurements>
) {
  for cursor_moved in cursor_moved_events.iter() {
    for (cam_t, cam) in camera_q.iter() {
      if let Some(pos) = world_from_viewport(
        &cursor_moved.position,
        cam_t,
        cam,
        &level_measurements
      ) {
        **cursor_pos = pos;
      }
    }
  }
}
