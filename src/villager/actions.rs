const MOVEMENT_SPEED: f32 = 3.0;

////////////////////////////////////////////////////////////////

use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::Rng;

use crate::assets::*;
use crate::fsm::*;
use crate::resource::WoodPile;

use crate::harvestable::tree::*;
use crate::resource::spawn_wood;
use crate::structure::house::*;
use crate::villager::villager::*;

#[derive(Clone)]
enum Decision {
    WalkToHouse,
    ChopTree,
    PickUpWood,
}

const DECISIONS: [Decision; 3] = [
    Decision::WalkToHouse,
    Decision::ChopTree,
    Decision::PickUpWood,
];
const WEIGHTS: [u32; 3] = [1, 2, 3];

pub fn villager_update(
    mut villagers: Query<
        (Entity, &mut Villager, &mut Transform),
        (With<Villager>, Without<House>, Without<Tree>),
    >,
    houses: Query<(Entity, &Transform), (With<House>, Without<Villager>)>,
    trees: Query<(Entity, &Transform), (With<Tree>, Without<Villager>)>,
    wood_resources: Query<(Entity, &Transform, &WoodPile), (Without<Villager>)>,
    time: Res<Time>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    let houses_iter = houses.iter().collect::<Vec<_>>();
    let trees_iter = trees.iter().collect::<Vec<_>>();
    let wood_resources_iter = wood_resources
        .iter()
        .filter(|(_, _, wood_pile)| wood_pile.dropped)
        .collect::<Vec<_>>();

    let dist = WeightedIndex::new(&WEIGHTS).unwrap();
    let mut rng = rand::rng();

    for (villager_entity, mut villager, mut transform) in &mut villagers {
        let mut action = FSMDecision::Continue;
        match villager.fsm.state {
            FSMState::Freeze => {
                action = FSMDecision::Continue;
            }
            FSMState::Idle => {
                let decision = DECISIONS[dist.sample(&mut rng)].clone();

                match decision {
                    Decision::WalkToHouse => {
                        let (target_house, target_house_transform) =
                            houses_iter[rand::rng().random_range(0..houses_iter.len())];
                        action =
                            FSMDecision::WalkTo(target_house, target_house_transform.translation);
                    }
                    Decision::ChopTree => {
                        if trees_iter.len() > 0 {
                            let (target_tree, target_tree_transform) =
                                trees_iter[rand::rng().random_range(0..trees_iter.len())];
                            action = FSMDecision::WalkToGather(
                                target_tree,
                                target_tree_transform.translation,
                            );
                        }
                    }
                    Decision::PickUpWood => {
                        if wood_resources_iter.len() > 0 {
                            // find closest wood pile
                            let target_wood = wood_resources_iter
                                .iter()
                                .min_by(|a, b| {
                                    let a_dist = a.1.translation.distance(transform.translation);
                                    let b_dist = b.1.translation.distance(transform.translation);
                                    a_dist.partial_cmp(&b_dist).unwrap()
                                })
                                .unwrap();

                            let (target_wood, target_wood_transform, target_wood_pile) =
                                *target_wood;

                            action = FSMDecision::PickUp(
                                target_wood,
                                *target_wood_pile,
                                target_wood_transform.translation,
                                None,
                            );
                        }
                    }
                }
            }
            FSMState::Walking(_, target) => {
                let direction = target - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    action = FSMDecision::Finished;
                }
            }
            FSMState::BringingTo(_, target, _, wood_entity) => {
                let direction = target - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    commands.entity(wood_entity).despawn_recursive();
                    action = FSMDecision::Finished;
                }
            }
            FSMState::WalkingToGather(target_entity, target_transform) => {
                let direction = target_transform - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target_transform, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    action = FSMDecision::Gather(target_entity, 0.7);
                }
            }
            // TODO: can stack carried wood piles
            FSMState::PickingUp(
                target_entity,
                target_wood_pile,
                target_transform,
                _carried_wood_pile,
            ) => {
                let direction = target_transform - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target_transform, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    let target_wood_entity = commands.get_entity(target_entity);

                    if target_wood_entity.is_some() {
                        // Bring to random house
                        let (target_house, target_house_transform) =
                            houses_iter[rand::rng().random_range(0..houses_iter.len())];

                        // Make villager hold the wood pile
                        let mut held_wood_entity = Entity::PLACEHOLDER;

                        commands.entity(villager_entity).with_children(|children| {
                            held_wood_entity = children
                                .spawn((
                                    SceneRoot(
                                        scene_assets
                                            .handles
                                            .get(&SceneAssetType::ResourceWood)
                                            .unwrap()
                                            .clone(),
                                    ),
                                    WoodPile {
                                        count: target_wood_pile.count,
                                        dropped: false,
                                    },
                                    Transform::from_translation(Vec3::new(0.0, 0.95, 0.0)),
                                ))
                                .id()
                        });

                        action = FSMDecision::BringTo(
                            target_house,
                            target_house_transform.translation,
                            target_wood_pile.clone(),
                            held_wood_entity,
                        );

                        commands.entity(target_entity).despawn_recursive();
                        // action = FSMDecision::Freeze;
                    }
                }
            }
            FSMState::Gathering(entity, _) => {
                if villager.fsm.is_finished(transform.translation) {
                    action = FSMDecision::Finished;
                    // TODO: This should be handled by the Gatherable itself, so if multiple villagers gather together,
                    // despawn_recursive and spawn_wood will still be called just once.
                    if let Some(entity) = commands.get_entity(entity) {
                        entity.despawn_recursive();
                    }
                    spawn_wood(
                        &mut commands,
                        &scene_assets,
                        transform.translation + Vec3::new(0.0, 0.1, 0.0),
                        rand::rng().random_range(1..=5),
                    );
                }
            }
            _ => {}
        }

        villager.fsm.state = villager.fsm.update(action, time.delta_secs());
    }
}

pub fn villager_cancel_if_entity_deleted(
    mut villagers: Query<&mut Villager>,
    mut deleted_entities: RemovedComponents<Tree>,
) {
    let deleted_entities = deleted_entities.read().collect::<HashSet<_>>();

    for mut villager in &mut villagers {
        match villager.fsm.state {
            FSMState::WalkingToGather(target, _)
            | FSMState::Gathering(target, _)
            | FSMState::PickingUp(target, _, _, _) => {
                if deleted_entities.contains(&target) {
                    villager.fsm.state = FSMState::Idle;
                }
            }
            _ => {}
        }
    }
}
