const MOVEMENT_SPEED: f32 = 3.0;

////////////////////////////////////////////////////////////////

use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use rand::Rng;

use crate::assets::SceneAssets;
use crate::fsm::*;

use crate::harvestable::tree::*;
use crate::resource::spawn_wood;
use crate::structure::house::*;
use crate::villager::villager::*;

pub fn villager_update(
    mut villagers: Query<
        (&mut Villager, &mut Transform),
        (With<Villager>, Without<House>, Without<Tree>),
    >,
    houses: Query<(Entity, &Transform), (With<House>, Without<Villager>, Without<Tree>)>,
    trees: Query<(Entity, &Transform), (With<Tree>, Without<Villager>, Without<House>)>,
    time: Res<Time>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    let houses_iter = houses.iter().collect::<Vec<_>>();
    let trees_iter = trees.iter().collect::<Vec<_>>();

    for (mut villager, mut transform) in &mut villagers {
        let mut action = FSMDecision::Continue;
        match villager.fsm.state {
            FSMState::Idle => {
                // choose a random house to move to or a random tree to gather from
                if rand::rng().random_bool(0.1) {
                    let (target_house, target_house_transform) =
                        houses_iter[rand::rng().random_range(0..houses_iter.len())];
                    action = FSMDecision::WalkTo(target_house, target_house_transform.translation);
                } else {
                    if trees_iter.len() > 0 {
                        let (target_tree, target_tree_transform) =
                            trees_iter[rand::rng().random_range(0..trees_iter.len())];
                        action = FSMDecision::WalkToGather(
                            target_tree,
                            target_tree_transform.translation,
                        );
                    }
                }
            }
            FSMState::Walking(_, target)
            | FSMState::PickingUp(_, target, _)
            | FSMState::BringingTo(_, target, _)
            | FSMState::WalkingToGather(_, target) => {
                let direction = target - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    action = match villager.fsm.state {
                        FSMState::WalkingToGather(target, _) => FSMDecision::Gather(target, 0.7),
                        _ => FSMDecision::Finished,
                    }
                }
            }
            FSMState::Gathering(entity, _) => {
                if villager.fsm.is_finished(transform.translation) {
                    action = FSMDecision::Finished;
                    if let Some(entity) = commands.get_entity(entity) {
                        entity.despawn_recursive();
                    }
                    spawn_wood(&mut commands, &scene_assets, transform.translation, rand::rng().random_range(1..10));
                }
            }
            _ => {}
        }

        // println!("state: {:?}, action: {:?}", villager.fsm.state, action);

        villager.fsm.state = villager.fsm.update(action, time.delta_secs());
    }
}

pub fn villager_cancel_if_entity_deleted(
    mut villagers: Query<&mut Villager>,
    mut deleted_entities: RemovedComponents<Tree>,
) {
    // convert to a set
    let deleted_entities = deleted_entities.read().collect::<HashSet<_>>();

    for mut villager in &mut villagers {
        match villager.fsm.state {
            FSMState::WalkingToGather(target, _) | FSMState::Gathering(target, _) => {
                if deleted_entities.contains(&target) {
                    villager.fsm.state = FSMState::Idle;
                }
            }
            _ => {}
        }
    }
}
