use bevy::prelude::*;

use crate::harvestable::harvestable::*;
use crate::villager::villager::Villager;

use crate::fsm::components::*;
use crate::fsm::transitions::*;

pub fn fsm_update_harvesting(
    mut commands: Commands,
    gatherer: Query<(Entity, &FSMHarvesting, &Villager)>,
    mut harvestables: Query<&mut Harvestable>,
    time: Res<Time>,
    mut harvestable_destroyed_events: EventReader<HarvestableDestroyed>,

) {
    let harvestable_destroyed_events = harvestable_destroyed_events.read().collect::<Vec<_>>();

    for (entity, fsm_gathering, villager) in &gatherer {
        if let Ok(mut harvestable) = harvestables.get_mut(fsm_gathering.target) {
            harvestable.health -= time.delta_secs() * villager.harvesting_speed;
        }

        for event in harvestable_destroyed_events.iter() {
            if event.entity == fsm_gathering.target {
                fsm_transition_to::<FSMHarvesting>(&mut commands, entity, FSMIdle);
            }
        }
    }
}