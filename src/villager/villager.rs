use bevy::prelude::*;

use crate::assets::*;
use crate::fsm::*;

#[derive(Component)]
pub struct Villager {
    pub fsm: FSM,
}

pub fn spawn_villager(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::Villager)
                .unwrap()
                .clone(),
        ),
        Transform::from_translation(position).with_scale(GLOBAL_SCALE_VEC),
        Villager {
            fsm: FSM::new_idle(),
        },
        Name::new("Villager"),
    ));
}