use bevy::prelude::*;

use rand::Rng;

use crate::fsm::components::*;
use crate::fsm::transitions::*;

use crate::item_drop::*;

use crate::structure::wood_hut::WoodHut;
use crate::villager::{actions::*, villager::Villager};

use crate::assets::*;

pub fn fsm_update_picking_up(
    mut commands: Commands,
    mut walker: Query<(Entity, &mut Transform, &Villager, &FSMPickingUp)>,
    wood_huts: Query<(Entity, &Transform), (With<WoodHut>, Without<FSMPickingUp>)>,
    wood_resources: Query<(Entity, &Transform, &WoodPile), (Without<FSMPickingUp>, With<ItemDrop>)>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    let wood_huts_iter = wood_huts.iter().collect::<Vec<_>>();

    for (entity, mut transform, villager, fsm_picking_up) in &mut walker {
        if let Ok((target_entity, target_transform, target_wood_pile)) = wood_resources.get(fsm_picking_up.target) {
            walk_to(&mut transform, target_transform, villager.movement_speed, &time);

            println!("distance vs proximity: {:?} {:?}", transform.translation.distance(target_transform.translation), fsm_picking_up.proximity);
            if transform.translation.distance(target_transform.translation) < fsm_picking_up.proximity {
                // Bring to random wood_hut
                let (target_wood_hut, _target_wood_hut_transform) =
                    wood_huts_iter[rand::rng().random_range(0..wood_huts_iter.len())];

                // Make villager hold the wood pile
                let mut held_wood_entity = Entity::PLACEHOLDER;

                commands.entity(entity).with_children(|children| {
                    held_wood_entity = children
                        .spawn((
                            WoodPileModel::new(&scene_assets, target_wood_pile.count),
                            Transform::from_translation(Vec3::new(0.0, 0.95, 0.0)),
                        ))
                        .id()
                });

                fsm_transition_to::<FSMPickingUp>(
                    &mut commands,
                    entity,
                    FSMBringingTo {
                        target: target_wood_hut,
                        held_resource: Some(held_wood_entity),
                        proximity: 0.2,
                    },
                );

                commands.entity(target_entity).despawn_recursive();
            }
        } else {
            fsm_transition_to::<FSMPickingUp>(&mut commands, entity, FSMIdle);
        }
    }
}
