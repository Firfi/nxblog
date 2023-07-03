use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use itertools::*;

pub(crate) fn with_level_asset(
  level_handles: &Query<&Handle<LdtkLevel>>,
  level_assets: &Res<Assets<LdtkLevel>>,
  level_events: &mut EventReader<LevelEvent>,
  mut cb: impl FnMut(&LdtkLevel),
) {
  for _ in level_events.iter().filter(|e| matches!(e, LevelEvent::Transformed(_)) ) {
    let level_handle = level_handles
      .get_single()
      .expect("only one level should be spawned at a time in this example");
    let level_asset = level_assets
      .get(level_handle)
      .expect("level asset should be loaded before LevelEvent::Transformed");
    cb(&level_asset);
  }
}
