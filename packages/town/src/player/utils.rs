use bevy::math::Vec2;
use crate::player::components::LookDirection;

pub fn look_direction_from_direction(v: Vec2) -> Option<LookDirection> {
  if v.x > 0.0 {
    Some(LookDirection::Right)
  } else if v.x < 0.0 {
    Some(LookDirection::Left)
  } else if v.y > 0.0 {
    Some(LookDirection::Up)
  } else if v.y < 0.0 {
    Some(LookDirection::Down)
  } else {
    None
  }
}
