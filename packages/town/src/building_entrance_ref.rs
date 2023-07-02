//! Contains the [BuildingEntranceRef] component and the ECS logic supporting it.
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Component that eventually transforms into the [BuildingEntranceRef] component.
///
/// This just stores the entity iid of the building_entrance entity.
/// The initial value of this is sourced from the entity's "building_entrance" field in LDtk.
/// In [resolve_building_entrance_references], this gets resolved to the actual bevy Entity of the building_entrance.
#[derive(Debug, Default, Deref, DerefMut, Component)]
pub struct UnresolvedBuildingEntranceRef(Option<EntityIid>);

impl UnresolvedBuildingEntranceRef {
  pub fn from_building_entrance_field(entity_instance: &EntityInstance) -> UnresolvedBuildingEntranceRef {
    UnresolvedBuildingEntranceRef(
      entity_instance
        .get_maybe_entity_ref_field("target")
        .expect("expected entity to have target entity ref field")
        .as_ref()
        .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
    )
  }
}

/// Component defining a relation - the "building_entrance" of this entity.
#[derive(Debug, Deref, DerefMut, Component, Reflect)]
pub struct BuildingEntranceRef(pub Entity);

pub fn resolve_building_entrance_references(
  mut commands: Commands,
  unresolved_building_entrances: Query<(Entity, &UnresolvedBuildingEntranceRef), Added<UnresolvedBuildingEntranceRef>>,
  ldtk_entities: Query<(Entity, &EntityIid)>,
) {
  for (child_entity, unresolved_building_entrance_ref) in unresolved_building_entrances.iter() {
    if let Some(building_entrance_iid) = unresolved_building_entrance_ref.0.as_ref() {
      let (building_entrance_entity, _) = ldtk_entities
        .iter()
        .find(|(_, iid)| *iid == building_entrance_iid)
        .expect("building_entrance entity should exist");
      commands
        .entity(child_entity)
        .remove::<UnresolvedBuildingEntranceRef>()
        .insert(BuildingEntranceRef(building_entrance_entity));
    } else {
      commands
        .entity(child_entity)
        .remove::<UnresolvedBuildingEntranceRef>();
    }
  }
}
