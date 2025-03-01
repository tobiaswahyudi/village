use bevy::prelude::*;

use crate::fsm::transitions::*;
use crate::fsm::components::*;
use crate::villager::actions::*;

use crate::villager::villager::Villager;

pub fn fsm_update_walking_to(
    mut commands: Commands,
    mut walker: Query<(Entity, &mut Transform, &Villager, &FSMWalkingTo)>,
    transforms: Query<&Transform, Without<FSMWalkingTo>>,
    time: Res<Time>,
) {
    for (entity, mut transform, villager, fsm_walking) in &mut walker {
        if let Ok(target_transform) = transforms.get(fsm_walking.target) {
            walk_to(&mut transform, target_transform, villager.movement_speed, &time);
            if transform.translation.distance(target_transform.translation) < fsm_walking.proximity {
                fsm_transition_to::<FSMWalkingTo>(&mut commands, entity, FSMIdle  );
            }
        }
    }
}