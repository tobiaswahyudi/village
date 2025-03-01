use bevy::prelude::*;

use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::Rng;

use crate::fsm::components::*;
use crate::fsm::transitions::*;

use crate::harvestable::tree::*;
use crate::item_drop::*;
use crate::structure::house::*;

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

pub fn fsm_update_idle(
    mut commands: Commands,
    idlers: Query<(Entity, &Transform), With<FSMIdle>>,
    houses: Query<(Entity, &Transform), (With<House>, Without<FSMIdle>)>,
    trees: Query<(Entity, &Transform), (With<Tree>, Without<FSMIdle>)>,
    wood_resources: Query<(Entity, &Transform, &WoodPile), (Without<FSMIdle>, With<ItemDrop>)>,
) {
    let houses_iter = houses.iter().collect::<Vec<_>>();
    let trees_iter = trees.iter().collect::<Vec<_>>();
    let wood_resources_iter = wood_resources.iter().collect::<Vec<_>>();

    let dist = WeightedIndex::new(&WEIGHTS).unwrap();
    let mut rng = rand::rng();

    let decision = DECISIONS[dist.sample(&mut rng)].clone();

    for (entity, transform) in &idlers {
        match decision {
            Decision::WalkToHouse => {
                let (target_house, _target_house_transform) =
                    houses_iter[rand::rng().random_range(0..houses_iter.len())];

                fsm_transition_to::<FSMIdle>(
                    &mut commands,
                    entity,
                    FSMWalkingTo {
                        target: target_house,
                        proximity: 0.2,
                    },
                );
            }
            Decision::ChopTree => {
                if trees_iter.len() > 0 {
                    let (target_tree, _target_tree_transform) =
                        trees_iter[rand::rng().random_range(0..trees_iter.len())];

                    fsm_transition_to::<FSMIdle>(
                        &mut commands,
                        entity,
                        FSMWalkingToHarvest {
                            target: target_tree,
                            proximity: 0.2,
                        },
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

                    let (target_wood, _target_wood_transform, _target_wood_pile) = *target_wood;

                    fsm_transition_to::<FSMIdle>(
                        &mut commands,
                        entity,
                        FSMPickingUp {
                            target: target_wood,
                            proximity: 0.2,
                        },
                    );
                }
            }
        }
    }
}
