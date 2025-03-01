use bevy::prelude::*;

use std::any::type_name;
use std::fmt::Debug;

pub fn fsm_transition_to<FSMFrom: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    fsm_to: impl Bundle + Debug
) {
    commands.entity(entity).remove::<FSMFrom>();
    println!("Transitioned from {:?} to {:?}", type_name::<FSMFrom>(), &fsm_to);
    commands.entity(entity).insert(fsm_to);
}