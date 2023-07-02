use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct LevelHeight(pub u32);

pub fn set_level_height_to_current_level(
  mut level_events: EventReader<LevelEvent>,
  level_handles: Query<&Handle<LdtkLevel>>,
  mut current_level_height: ResMut<LevelHeight>,
  level_assets: Res<Assets<LdtkLevel>>,
) {
  for level_event in level_events.iter() {
    if matches!(level_event, LevelEvent::Transformed(_)) {
      let level_handle = level_handles
        .get_single()
        .expect("only one level should be spawned at a time in this example");

      let level_asset = level_assets
        .get(level_handle)
        .expect("level asset should be loaded before LevelEvent::Transformed");

      let h = level_asset
        .level.px_hei;

      **current_level_height = u32::try_from(h).expect("negative level height?"); // because, why i32?
    }
  }
}
