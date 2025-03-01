use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Harvestable {
    pub health: f32,
    // pub max_health: f32,
}

#[derive(Event)]
pub struct HarvestableDestroyed {
    pub entity: Entity,
}

#[derive(Component)]
pub struct HarvestableDeathmark;

pub fn check_harvestable_destroyed(
    mut commands: Commands,
    mut harvestable_destroyed_events: EventWriter<HarvestableDestroyed>,
    harvestables: Query<(Entity, &Harvestable, Option<&HarvestableDeathmark>)>,
) {
    for (entity, harvestable, harvestable_to_destroy) in harvestables.iter() {
        if harvestable_to_destroy.is_none() && harvestable.health <= 0.0 {
            harvestable_destroyed_events.send(HarvestableDestroyed {
                entity,
            });
            commands.entity(entity).insert(HarvestableDeathmark);
        }
    }
}