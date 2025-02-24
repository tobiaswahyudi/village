const CLOSE_ENOUGH_DISTANCE: f32 = 0.1;

////////////////////////////////////////////////////////////////

use crate::resource::PickupItem;
use bevy::math::*;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FSM {
    pub state: FSMState,
}

impl FSM {
    pub fn new_idle() -> Self {
        Self {
            state: FSMState::Idle,
        }
    }
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
    Walking(Entity, Vec3),
    Building(Entity, TaskProgress),
    WalkingToGather(Entity, Vec3),
    Gathering(Entity, TaskProgress),
    PickingUp(Entity, Vec3, Option<PickupItem>),
    BringingTo(Entity, Vec3, Option<PickupItem>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FSMDecision {
    Continue,
    Finished,
    WalkTo(Entity, Vec3),
    Build(Entity, f32),
    WalkToGather(Entity, Vec3),
    Gather(Entity, f32),
    PickUp(Entity, Vec3, Option<PickupItem>),
    BringTo(Entity, Vec3, Option<PickupItem>),
}

impl FSM {
    pub fn new() -> Self {
        Self {
            state: FSMState::Idle,
        }
    }

    pub fn update(&mut self, decision: FSMDecision, time_delta: f32) -> FSMState {
        match (self.state, decision) {
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
            (FSMState::Idle, FSMDecision::PickUp(item, position, held_item)) => {
                FSMState::PickingUp(item, position, held_item)
            }
            (FSMState::Idle, FSMDecision::BringTo(target, position, held_item)) => {
                FSMState::BringingTo(target, position, held_item)
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
            // Picking up stuff
            (FSMState::PickingUp(item, position, held_item), _) => {
                FSMState::PickingUp(item, position, held_item)
            }
            // Bringing stuff to a target
            (FSMState::BringingTo(target, position, held_item), _) => {
                FSMState::BringingTo(target, position, held_item)
            }
            // Continue, or other invalid transitions
            (_, _) => self.state,
        }
    }

    pub fn is_finished(&self, position: Vec3) -> bool {
        match self.state {
            FSMState::Idle => false,
            FSMState::Building(_, task_progress) | FSMState::Gathering(_, task_progress) => {
                task_progress.time_elapsed >= task_progress.time_needed
            }
            FSMState::BringingTo(_, target_position, _)
            | FSMState::Walking(_, target_position)
            | FSMState::WalkingToGather(_, target_position)
            | FSMState::PickingUp(_, target_position, _) => {
                position.distance(target_position) < CLOSE_ENOUGH_DISTANCE
            }
        }
    }
}