const CLOSE_ENOUGH_DISTANCE: f32 = 0.1;

////////////////////////////////////////////////////////////////

use crate::resource::WoodPile;
use bevy::math::*;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FSM {
    pub state: FSMState,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TaskProgress {
    time_needed: f32,
    time_elapsed: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
// Later on, these states should be higher level actions, like "Building a house" or "Gathering wood".
// Each action should have a Sub-FSM, this allows 'emergency' actions to be performed, like "run away from fire"
// or "assist in fighting a monster". Also makes the transitions less tedious.
pub enum FSMState {
    Idle,
    Freeze,
    Walking(Entity, Vec3),
    Building(Entity, TaskProgress),
    WalkingToGather(Entity, Vec3),
    Gathering(Entity, TaskProgress),
    PickingUp(Entity, WoodPile, Vec3, Option<WoodPile>),
    BringingTo(Entity, Vec3, WoodPile, Entity),
}

// What's decision even used for? It's translated directly to a state, so maybe just use the state directly?
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FSMDecision {
    Freeze,
    Continue,
    Finished,
    WalkTo(Entity, Vec3),
    Build(Entity, f32),
    WalkToGather(Entity, Vec3),
    Gather(Entity, f32),
    PickUp(Entity, WoodPile, Vec3, Option<WoodPile>),
    BringTo(Entity, Vec3, WoodPile, Entity),
}

impl FSM {
    pub fn new_idle() -> Self {
        Self {
            state: FSMState::Idle,
        }
    }

    pub fn update(&mut self, decision: FSMDecision, time_delta: f32) -> FSMState {
        println!("FSMState: {:?}", self.state);
        match (self.state, decision) {
            (_, FSMDecision::Freeze) => FSMState::Freeze,
            (_, FSMDecision::Finished) => FSMState::Idle,
            // From Idle: Start doing the thing
            (FSMState::Idle, FSMDecision::WalkTo(target, position)) => {
                FSMState::Walking(target, position)
            }
            (FSMState::Idle, FSMDecision::Build(target, time_needed)) => {
                FSMState::Building(
                    target,
                    TaskProgress {
                        time_needed: time_needed,
                        time_elapsed: 0.0,
                    },
                )
            }
            (FSMState::Idle, FSMDecision::WalkToGather(target, position)) => {
                FSMState::WalkingToGather(target, position)
            }
            (FSMState::Idle | FSMState::WalkingToGather(_, _), FSMDecision::Gather(target, time_needed)) => {
                FSMState::Gathering(
                    target,
                    TaskProgress {
                        time_needed: time_needed,
                        time_elapsed: 0.0,
                    },
                )
            }
            (FSMState::Idle, FSMDecision::PickUp(item, wood_pile, position, held_item)) => {
                FSMState::PickingUp(item, wood_pile, position, held_item)
            }
            (FSMState::Idle, FSMDecision::BringTo(target, position, wood_pile, wood_entity)) => {
                FSMState::BringingTo(target, position, wood_pile, wood_entity)
            }
            // Walking
            (FSMState::Walking(_, _), FSMDecision::WalkTo(target, position)) => {
                FSMState::Walking(target, position)
            }
            // Building
            (FSMState::Building(target, task_progress), _) => FSMState::Building(
                target,
                TaskProgress {
                    time_needed: task_progress.time_needed,
                    time_elapsed: task_progress.time_elapsed + time_delta,
                },
            ),
            // Gathering
            (FSMState::Gathering(target, task_progress), _) => FSMState::Gathering(
                target,
                TaskProgress {
                    time_needed: task_progress.time_needed,
                    time_elapsed: task_progress.time_elapsed + time_delta,
                },
            ),
            (FSMState::PickingUp(_, _, _, _), FSMDecision::BringTo(target_entity, target_position, wood_pile, wood_entity)) => {
                FSMState::BringingTo(target_entity, target_position, wood_pile, wood_entity)
            }
            // Continue, or other invalid transitions
            (_, _) => self.state,
        }
    }

    pub fn is_finished(&self, position: Vec3) -> bool {
        match self.state {
            FSMState::Freeze => false,
            FSMState::Idle => false,
            FSMState::Building(_, task_progress) | FSMState::Gathering(_, task_progress) => {
                // TODO: This should be handled by the Gatherable itself, so multiple villagers can gather at once.
                task_progress.time_elapsed >= task_progress.time_needed
            }
            FSMState::BringingTo(_, target_position, _, _)
            | FSMState::Walking(_, target_position)
            | FSMState::WalkingToGather(_, target_position)
            | FSMState::PickingUp(_, _, target_position, _) => {
                println!("Distance: {}", position.distance(target_position));
                position.distance(target_position) < CLOSE_ENOUGH_DISTANCE
            }
        }
    }
}