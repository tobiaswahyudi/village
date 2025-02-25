use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::assets::*;

#[derive(Clone, Copy, Component, PartialEq, Debug)]
pub struct WoodPile {
    pub count: u32,
    pub dropped: bool,
}

pub fn spawn_wood(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3, count: u32) {
    // Four stacks of wood don't look good, so on spawning, 4 is converted to 3.
    let count = if count == 4 { 3 } else { count };
    commands
        .spawn((
            SceneRoot(
                scene_assets
                    .handles
                    .get(&SceneAssetType::ResourceWood)
                    .unwrap()
                    .clone(),
            ),
            WoodPile {
                count: count,
                dropped: true,
            },
            Transform::from_translation(position).with_scale(GLOBAL_SCALE_VEC),
            Velocity {
                linvel: Vec3::new(
                    rand::rng().random_range(-1.0..=1.0),
                    2.0,
                    rand::rng().random_range(-1.0..=1.0),
                ),
                angvel: Vec3::new(
                    rand::rng().random_range(-6.0..=6.0),
                    rand::rng().random_range(-6.0..=6.0),
                    rand::rng().random_range(-6.0..=6.0),
                )
            },
            RigidBody::Dynamic,
            Restitution::coefficient(0.5),
            Name::new("Wood"),
        ))
        .with_children(|this| {
            if count == 1 {
                this.spawn((
                    Collider::cuboid(0.085, 0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                ));
            }
            if count >= 2 {
                this.spawn((
                    Collider::cylinder(0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                ));
                this.spawn((
                    Collider::cylinder(0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                        .with_translation(Vec3::new(-0.16, 0.0, 0.0)),
                ));
            }
            if count >= 3 {
                this.spawn((
                    Collider::cylinder(0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                        .with_translation(Vec3::new(-0.08, 0.12, 0.0)),
                ));
            }
            if count >= 5 {
                this.spawn((
                    Collider::cylinder(0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                        .with_translation(Vec3::new(0.16, 0.0, 0.0)),
                ));
                this.spawn((
                    Collider::cylinder(0.34, 0.085),
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                        .with_translation(Vec3::new(0.08, 0.12, 0.0)),
                ));
            }
        });
}

pub fn update_wood_stacks(
    wood_stacks: Query<(Entity, &WoodPile), Changed<WoodPile>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for (wood_scene, wood_pile) in wood_stacks.iter() {
        // This print message verifies that the stuff only updates when the pickup item spawns.
        // println!("Update Wood scene: {:?} {:?}", wood_scene, pickup_item);
        if let Ok(wood_children) = children.get(wood_scene) {
            if wood_children.len() > 0 {
                let wood_scene_root = wood_children[wood_children.len() - 1];
                if let Ok(children) = children.get(wood_scene_root) {
                    for (i, child) in children.iter().enumerate() {
                        if i as u32 >= wood_pile.count {
                            commands.entity(*child).insert(Visibility::Hidden);
                        } else {
                            commands.entity(*child).remove::<Visibility>();
                        }
                    }
                }
            }
        }
    }
}
