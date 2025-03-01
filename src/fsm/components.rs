use bevy::prelude::*;

#[derive(Component, Default)]
pub struct HasFSM;

// FSM States

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMIdle;

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMWalkingTo {
    pub target: Entity,
    pub proximity: f32,
}

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMBuilding {
    pub target: Entity,
}

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMWalkingToHarvest {
    pub target: Entity,
    // Should be a property of the Harvestable
    pub proximity: f32,
}

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMHarvesting {
    pub target: Entity,
}

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMPickingUp {
    pub target: Entity,
    pub proximity: f32,
}

#[derive(Component)]
#[require(HasFSM)]
pub struct FSMBringingTo {
    pub target: Entity,
    pub held_resource: Option<Entity>,
    pub proximity: f32,
}
