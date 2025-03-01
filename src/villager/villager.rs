const MOVEMENT_SPEED: f32 = 3.0;
const HARVESTING_SPEED: f32 = 1.0;

////////////////////////////////////////////////////////////////

use bevy::prelude::*;

use crate::assets::*;
use crate::fsm::*;

#[derive(Component)]
pub struct Villager {
    pub movement_speed: f32,
    pub harvesting_speed: f32,
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
            movement_speed: MOVEMENT_SPEED,
            harvesting_speed: HARVESTING_SPEED,
        },
        FSMIdle,
        Name::new("Villager"),
    ));
}