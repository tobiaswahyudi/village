// On average, this many trees will spawn per second
const TREE_GROW_RATE: f32 = 0.6;

// This should be removed when we do chunking stuff anyway
const WORLD_RADIUS: f32 = 6.4;

////////////////////////////////////////////////////////////////

use bevy::prelude::*;
use rand::Rng;

use crate::assets::*;

use crate::harvestable::harvestable::*;
use crate::resource::spawn_wood;

#[derive(Component)]
#[require(Harvestable)]
pub struct Tree;

const TREE_TYPES: [SceneAssetType; 3] = [
    SceneAssetType::TreePine,
    SceneAssetType::TreeRound,
    SceneAssetType::TreeDead,
];

pub fn spawn_tree(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    let tree_type = &TREE_TYPES[rand::rng().random_range(0..TREE_TYPES.len())];
    commands.spawn((
        SceneRoot(scene_assets.handles.get(tree_type).unwrap().clone()),
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        Tree,
        Harvestable {
            health: 2.0,
            // max_health: 2.0,
        },
        Name::new("Tree"),
    ));
}

pub fn tick_grow_tree(mut commands: Commands, scene_assets: Res<SceneAssets>, time: Res<Time>) {
    if rand::rng().random::<f32>() > TREE_GROW_RATE * time.delta_secs() {
        return;
    }

    let r = WORLD_RADIUS * rand::rng().random::<f32>();
    let theta = rand::rng().random::<f32>() * 2.0 * std::f32::consts::PI;

    let x = r * theta.cos();
    let z = r * theta.sin();

    let random_position = Vec3::new(x, 0.0, z);
    spawn_tree(&mut commands, &scene_assets, random_position);
}

pub fn check_tree_should_be_destroyed(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    trees: Query<(Entity, &Transform, Option<&HarvestableDeathmark>), With<Tree>>,
) {
    for (entity, transform, harvestable_deathmark) in trees.iter() {
        if harvestable_deathmark.is_some() {
            commands.entity(entity).despawn_recursive();
            spawn_wood(
                &mut commands,
                &scene_assets,
                transform.translation + Vec3::new(0.0, 0.5, 0.0),
                rand::rng().random_range(1..=5),
            );
        }
    }
}