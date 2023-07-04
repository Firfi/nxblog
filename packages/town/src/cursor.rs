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
