use bevy::prelude::*;

use crate::fsm::transitions::*;
use crate::fsm::components::*;
use crate::harvestable::HarvestableDestroyed;
use crate::villager::actions::*;

use crate::villager::villager::Villager;

pub fn fsm_update_walking_to_harvest(
    mut commands: Commands,
    mut walker: Query<(Entity, &mut Transform, &Villager, &FSMWalkingToHarvest)>,
    transforms: Query<&Transform, Without<FSMWalkingToHarvest>>,
    time: Res<Time>,
    mut harvestable_destroyed_events: EventReader<HarvestableDestroyed>,
) {
    let harvestable_destroyed_events = harvestable_destroyed_events.read().collect::<Vec<_>>();

    for (entity, mut transform, villager, fsm_walking) in &mut walker {
        if let Ok(target_transform) = transforms.get(fsm_walking.target) {
            walk_to(&mut transform, target_transform, villager.movement_speed, &time);
            if transform.translation.distance(target_transform.translation) < fsm_walking.proximity {
                fsm_transition_to::<FSMWalkingToHarvest>(&mut commands, entity, FSMHarvesting {
                    target: fsm_walking.target,
                });
            }
        }

        for event in harvestable_destroyed_events.iter() {
            if event.entity == fsm_walking.target {
                fsm_transition_to::<FSMWalkingToHarvest>(&mut commands, entity, FSMIdle);
            }
        }
    }
}