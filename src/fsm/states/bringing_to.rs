use bevy::prelude::*;

use crate::fsm::components::*;
use crate::fsm::transitions::*;
use crate::item_drop::*;
use crate::structure::wood_hut::WoodHut;
use crate::villager::actions::*;
use crate::villager::villager::Villager;

pub fn fsm_update_bringing_to(
    mut commands: Commands,
    mut walker: Query<(Entity, &mut Transform, &Villager, &FSMBringingTo)>,
    mut wood_pile: Query<&mut WoodPile>,
    transforms: Query<&Transform, Without<FSMBringingTo>>,
    children: Query<&Children, With<WoodHut>>,
    time: Res<Time>,
) {
    for (entity, mut transform, villager, fsm_bringing_to) in &mut walker {
        if let Ok(target_transform) = transforms.get(fsm_bringing_to.target) {
            walk_to(
                &mut transform,
                target_transform,
                villager.movement_speed,
                &time,
            );
            if transform.translation.distance(target_transform.translation)
                < fsm_bringing_to.proximity
            {
                if let Some(held_resource) = fsm_bringing_to.held_resource {
                    let held_wood_count = if let Ok(held_wood_pile) = wood_pile.get(held_resource) {
                        held_wood_pile.count
                    } else {
                        0
                    };

                    let wood_hut_first_child = *children.get(fsm_bringing_to.target).unwrap().first().unwrap();

                    println!("target wood pile: {:?} {:?}", wood_hut_first_child, wood_pile.get_mut(wood_hut_first_child));

                    if let Ok(mut wood_hut_pile) = wood_pile.get_mut(wood_hut_first_child) {
                        println!("wood_hut_pile: {:?}", wood_hut_pile);
                        wood_hut_pile.count += held_wood_count;
                    }

                    commands.entity(held_resource).despawn_recursive();
                }
                fsm_transition_to::<FSMBringingTo>(&mut commands, entity, FSMIdle);
            }
        }
    }
}
