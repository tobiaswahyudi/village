use bevy::prelude::*;

use crate::fsm::components::*;
use crate::fsm::transitions::*;
use crate::villager::actions::*;
use crate::villager::villager::Villager;

pub fn fsm_update_bringing_to(
    mut commands: Commands,
    mut walker: Query<(Entity, &mut Transform, &Villager, &FSMBringingTo)>,
    transforms: Query<&Transform, Without<FSMBringingTo>>,
    time: Res<Time>,
) {
    for (entity, mut transform, villager, fsm_bringing_to) in &mut walker {
        if let Ok(target_transform) = transforms.get(fsm_bringing_to.target) {
            walk_to(&mut transform, target_transform, villager.movement_speed, &time);
            if transform.translation.distance(target_transform.translation) < fsm_bringing_to.proximity {
                if let Some(held_resource) = fsm_bringing_to.held_resource {
                    commands.entity(held_resource).despawn_recursive();
                }
                fsm_transition_to::<FSMBringingTo>(&mut commands, entity, FSMIdle);
            }
        }
    }
}