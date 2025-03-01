use bevy::prelude::*;

pub fn fsm_transition_to<FSMFrom: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    fsm_to: impl Bundle
) {
    commands.entity(entity).remove::<FSMFrom>();
    commands.entity(entity).insert(fsm_to);
}