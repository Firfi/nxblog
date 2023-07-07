use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_ecs_ldtk::{LdtkSettings, LdtkWorldBundle};
use bevy_ecs_ldtk::LevelSpawnBehavior::UseWorldTranslation;

pub fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>, mut settings: ResMut<LdtkSettings>, mut window_query: Query<&mut Window, With<PrimaryWindow>>,) {
  let mut window = window_query.single_mut();
  let bundle = LdtkWorldBundle {
    ldtk_handle: asset_server.load("ldtk/town.ldtk"),
    // transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    // transform: Transform::from_xyz(-100.0, -100.0, 0.0),
    ..Default::default()
  };
  // settings.level_spawn_behavior = UseWorldTranslation { load_level_neighbors: false };

  // window.resolution.set(100.0, 100.0);
  commands.spawn(bundle);
}
