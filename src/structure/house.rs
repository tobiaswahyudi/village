use bevy::prelude::*;

use crate::assets::*;

#[derive(Component)]
pub struct House;

pub fn spawn_house(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::StructureHouse)
                .unwrap()
                .clone(),
        ),
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        House,
        Name::new("House"),
    ));
}