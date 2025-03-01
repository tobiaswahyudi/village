use bevy::prelude::*;

use crate::{assets::*, item_drop::WoodPileModel};

#[derive(Component)]
pub struct WoodHut;

pub fn spawn_wood_hut(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::StructureWoodHut)
                .unwrap()
                .clone(),
        ),
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        WoodHut,
        Name::new("Wood Hut"),
    )).with_children(|this| {
        this.spawn((
            WoodPileModel::new(scene_assets, 0),
            Transform::from_translation(Vec3::new(0.0, 0.075, 0.0)),
        ));
    });
}
