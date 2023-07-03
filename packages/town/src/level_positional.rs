use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Debug, Default, Component)]
pub struct LevelPositional {
  pub px: IVec2,
  pub width: i32,
  pub height: i32,
}

impl LevelPositional {
  pub fn from_entity_field(entity_instance: &EntityInstance) -> LevelPositional {
    return LevelPositional {
      px: entity_instance.px,
      width: entity_instance.width,
      height: entity_instance.height,
    }
  }
}
