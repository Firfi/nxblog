use bevy::prelude::Component;
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapTileSize};

#[derive(Component)]
pub struct Player;


#[derive(Component, Debug, Eq, PartialEq)]
pub enum MovingState {
  Idle,
  Moving
}

#[derive(Component, Debug, Eq, PartialEq)]
pub enum LookDirection {
  Up,
  Down,
  Left,
  Right
}

pub struct TilemapMetadata {
  pub size: TilemapSize,
  pub tile_size: TilemapTileSize,
  pub grid_size: TilemapGridSize,
  pub gap: usize
}
