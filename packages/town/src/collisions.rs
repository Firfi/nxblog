use std::collections::HashSet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use itertools::*;
use crate::level::level_utils::with_level_asset;
use crate::level_measurements::LevelMeasurements;
use crate::player::translation_from_collision_int;

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone, PartialEq, Eq, Hash)]
pub struct CollisionIndex(pub usize);

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone, PartialEq, Eq)]
pub struct LevelCollisionsSet(pub HashSet<CollisionIndex>);

pub struct CollisionsInitialized;

pub fn set_collisions_to_current_level(
  mut level_events: EventReader<LevelEvent>,
  level_handles: Query<&Handle<LdtkLevel>>,
  mut current_level_collisions_set: ResMut<LevelCollisionsSet>,
  level_assets: Res<Assets<LdtkLevel>>,
  mut event_writer: EventWriter<CollisionsInitialized>,
) {
  let current_level_collisions_ref = &mut *current_level_collisions_set;
  with_level_asset(&level_handles, &level_assets, &mut level_events, |asset| {
    *current_level_collisions_ref = LevelCollisionsSet(asset.level.layer_instances
      .clone()/*??*/
      .expect("expected layer_instances here").iter()
      .filter(|li| li.identifier.eq("Collisions"))
      .exactly_one().expect("'Collisions' layer expected")
      .int_grid_csv
      .iter()
      .enumerate()
      .map(|(i, v)| (u32::try_from(v.clone()).expect("expect int map values to be integers"), i as u32))
      .filter(|(v, coord_id)| v.clone()/*??*/ == 1/*collisions encoded into 1s*/)
      .map(|(_, coord_id)| CollisionIndex(usize::try_from(coord_id).expect("Collision index is too big")))
      .collect::<HashSet<CollisionIndex>>());
    event_writer.send(CollisionsInitialized);
  });

}

pub fn draw_debug_collisions(
  mut commands: Commands,
  level_measurements: Res<LevelMeasurements>,
  mut event_reader: EventReader<CollisionsInitialized>,
  level_collisions: Res<LevelCollisionsSet>,
) {
  for _ in event_reader.iter() {
    for collision in level_collisions.0.iter() {
      let collision_grid_coords = translation_from_collision_int(&level_measurements, &collision);
      // draw a square on z 10
      commands.spawn(SpriteBundle {
        // material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(collision_grid_coords.x as f32, collision_grid_coords.y as f32, 10.0),
        sprite: Sprite {
          color: Color::rgb(1.0, 0.0, 0.0),
          custom_size: Some(Vec2::new(level_measurements.grid_size as f32, level_measurements.grid_size as f32)),
          ..default()
        },
        ..Default::default()
      });
      // if translation.x > collision_grid_coords.x as f32 &&
      //   translation.x < (collision_grid_coords.x + level_measurements.c_wid as i32) as f32 &&
      //   translation.y > collision_grid_coords.y as f32 &&
      //   translation.y < (collision_grid_coords.y + level_measurements.c_hei as i32) as f32 {
      //   translation.x = collision_grid_coords.x as f32;
      //   translation.y = collision_grid_coords.y as f32;
      // }
    }
  }
}
