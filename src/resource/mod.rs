use bevy::prelude::*;

use crate::assets::*;

#[derive(Clone, Copy, Component, PartialEq, Debug)]
pub struct PickupItem {
    pub item_type: PickupItemType,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupItemType {
    Wood,
}

pub fn spawn_wood(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3, count: u32) {
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::ResourceWood)
                .unwrap()
                .clone(),
        ),
        PickupItem {
            item_type: PickupItemType::Wood,
            count: count,
        },
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        Name::new("Wood"),
    ));
}

pub fn update_wood_stacks(
    wood_stacks: Query<(Entity, &PickupItem), Changed<PickupItem>>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for (wood_scene, pickup_item) in wood_stacks.iter() {
        // println!("Update Wood scene: {:?} {:?}", wood_scene, pickup_item);
        if let Ok(wood_scene_root) = children.get(wood_scene) {
            if let Ok(children) = children.get(wood_scene_root[0]) {
                for (i, child) in children.iter().enumerate() {
                    if i as u32 > pickup_item.count {
                        commands.entity(*child).insert(Visibility::Hidden);
                    } else {
                        commands.entity(*child).remove::<Visibility>();
                    }
                }
            }
        }
    }
}
