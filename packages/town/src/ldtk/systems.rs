use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

pub fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
  let bundle = LdtkWorldBundle {
    ldtk_handle: asset_server.load("ldtk/town.ldtk"),
    ..Default::default()
  };
  commands.spawn(bundle);
}
