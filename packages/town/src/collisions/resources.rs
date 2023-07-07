use bevy::prelude::{Deref, DerefMut, Resource};
use std::collections::HashSet;

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone, PartialEq, Eq, Hash)]
pub struct CollisionIndex(pub usize);

#[derive(Debug, Default, Resource, Deref, DerefMut, Clone, PartialEq, Eq)]
pub struct LevelCollisionsSet(pub HashSet<CollisionIndex>);
