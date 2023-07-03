use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use itertools::*;

#[derive(Debug, Default, Resource, Reflect, Clone, PartialEq, Eq, Hash)]
#[reflect(Resource)]
pub struct LevelMeasurements {
  pub px_wid: u32,
  pub px_hei: u32,
  pub c_wid: u32,
  pub c_hei: u32,
  pub grid_size: u32,
}

pub fn set_level_measurements_to_current_level(
  mut level_events: EventReader<LevelEvent>,
  level_handles: Query<&Handle<LdtkLevel>>,
  mut current_level_measurements: ResMut<LevelMeasurements>,
  level_assets: Res<Assets<LdtkLevel>>,
) {
  for _ in level_events.iter().filter(|e| matches!(e, LevelEvent::Transformed(_)) ) {
    let level_handle = level_handles
      .get_single()
      .expect("only one level should be spawned at a time in this example");

    let level_asset = level_assets
      .get(level_handle)
      .expect("level asset should be loaded before LevelEvent::Transformed");
    // for mouse position shenanigans
    let px_wid = u32::try_from(level_asset
      .level.px_wid).expect("negative level width?");
    let px_hei = u32::try_from(level_asset
      .level.px_hei).expect("negative level height?");
    *current_level_measurements = level_asset.level.layer_instances.clone().expect("First of all, layer instances are expected here (not 'separate layers' setting)").iter().map(|li| LevelMeasurements {
      c_wid: u32::try_from(li.c_wid).expect("expected integer level wid"),
      c_hei: u32::try_from(li.c_hei).expect("expected integer level hei"),
      grid_size: u32::try_from(li.grid_size).expect("expected integer level grid_size"),
      px_wid, // not the best place to introduce it but let's roll for now
      px_hei // not the best place to introduce it but let's roll for now
    }).unique().exactly_one().expect("All the layers expected to have the same measurement right now");

  }
}
