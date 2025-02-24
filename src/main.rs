//! A simple 3D scene with light shining over a cube sitting on a plane.

mod fsm;
mod resource;

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;

use crate::fsm::*;

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Village".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(Update, (villager_movement, grow_tree))
        .run();
}

// Asset Loading

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SceneAssetType {
    House,
    TreePine,
    TreeRound,
    TreeDead,
    Villager,
}

#[derive(Resource)]
pub struct SceneAssets {
    handles: HashMap<SceneAssetType, Handle<Scene>>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut asset_handles = HashMap::new();

    asset_handles.insert(
        SceneAssetType::House,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("house.glb")),
    );

    asset_handles.insert(
        SceneAssetType::TreePine,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.glb")),
    );
    asset_handles.insert(
        SceneAssetType::TreeRound,
        asset_server.load(GltfAssetLabel::Scene(1).from_asset("tree.glb")),
    );
    asset_handles.insert(
        SceneAssetType::TreeDead,
        asset_server.load(GltfAssetLabel::Scene(2).from_asset("tree.glb")),
    );

    asset_handles.insert(
        SceneAssetType::Villager,
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("villager_man.glb")),
    );

    commands.insert_resource(SceneAssets {
        handles: asset_handles,
    });
}

const GLOBAL_SCALE: f32 = 0.3;
const GLOBAL_SCALE_VEC: Vec3 = Vec3::new(GLOBAL_SCALE, GLOBAL_SCALE, GLOBAL_SCALE);

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Villager {
    pub fsm: FSM,
}

fn spawn_house(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::House)
                .unwrap()
                .clone(),
        ),
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        House,
        Name::new("House"),
    ));
}

#[derive(Component)]
pub struct Tree;

const TREE_TYPES: [SceneAssetType; 3] = [
    SceneAssetType::TreePine,
    SceneAssetType::TreeRound,
    SceneAssetType::TreeDead,
];

fn spawn_tree(commands: &mut Commands, scene_assets: &SceneAssets, position: Vec3) {
    let tree_type = &TREE_TYPES[rand::rng().random_range(0..TREE_TYPES.len())];
    commands.spawn((
        SceneRoot(scene_assets.handles.get(tree_type).unwrap().clone()),
        Transform::from_xyz(position.x, position.y, position.z).with_scale(GLOBAL_SCALE_VEC),
        Tree,
        Name::new("Tree"),
    ));
}

const WORLD_RADIUS: f32 = 5.0;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_assets: Res<SceneAssets>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(WORLD_RADIUS))),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
        MeshMaterial3d(materials.add(Color::srgb_u8(50, 200, 50))),
    ));
    // Houses
    // spawn_house(&mut commands, &scene_assets, Vec3::new(0.0, 0.0, 0.0));
    spawn_house(&mut commands, &scene_assets, Vec3::new(3.0, 0.0, 1.0));
    spawn_house(
        &mut commands,
        &scene_assets,
        Vec3::new(0.633975, 0.0, 3.09808),
    );
    spawn_house(
        &mut commands,
        &scene_assets,
        Vec3::new(-2.36603, 0.0, 2.09808),
    );
    spawn_house(&mut commands, &scene_assets, Vec3::new(-3.0, 0.0, -1.0));
    spawn_house(
        &mut commands,
        &scene_assets,
        Vec3::new(-0.633975, 0.0, -3.09808),
    );
    spawn_house(
        &mut commands,
        &scene_assets,
        Vec3::new(2.36603, 0.0, -2.09808),
    );

    // Villager
    commands.spawn((
        SceneRoot(
            scene_assets
                .handles
                .get(&SceneAssetType::Villager)
                .unwrap()
                .clone(),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(GLOBAL_SCALE_VEC),
        Villager {
            fsm: FSM::new_idle(),
        },
        Name::new("Villager"),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-2.5, 4.5, 9.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ),
    ));
}

const MOVEMENT_SPEED: f32 = 3.0;

fn villager_movement(
    mut villagers: Query<
        (&mut Villager, &mut Transform),
        (With<Villager>, Without<House>, Without<Tree>),
    >,
    houses: Query<(Entity, &Transform), (With<House>, Without<Villager>, Without<Tree>)>,
    trees: Query<(Entity, &Transform), (With<Tree>, Without<Villager>, Without<House>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let houses_iter = houses.iter().collect::<Vec<_>>();
    let trees_iter = trees.iter().collect::<Vec<_>>();

    for (mut villager, mut transform) in &mut villagers {
        let mut action = FSMDecision::Continue;
        // if villager.moving_to.distance(transform.translation) < 0.1 {
        //     // choose a random house to move to
        //     let house_transforms = houses.iter().collect::<Vec<_>>();
        //     let random_house =
        //         house_transforms[rand::rng().random_range(0..house_transforms.len())];
        //     villager.moving_to = random_house.translation;
        // } else {
        //     // move towards the house
        //     let direction = villager.moving_to - transform.translation;
        //     transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
        //     transform.look_at(villager.moving_to, Vec3::Y);
        // }
        match villager.fsm.state {
            FSMState::Idle => {
                // choose a random house to move to or a random tree to gather from
                if rand::rng().random_bool(0.1) {
                    let (target_house, target_house_transform) =
                        houses_iter[rand::rng().random_range(0..houses_iter.len())];
                    action = FSMDecision::WalkTo(target_house, target_house_transform.translation);
                } else {
                    if trees_iter.len() > 0 {
                        let (target_tree, target_tree_transform) =
                            trees_iter[rand::rng().random_range(0..trees_iter.len())];
                        action = FSMDecision::WalkToGather(target_tree, target_tree_transform.translation);
                    }
                }
            }
            FSMState::Walking(_, target)
            | FSMState::PickingUp(_, target, _)
            | FSMState::BringingTo(_, target, _)
            | FSMState::WalkingToGather(_, target) => {
                let direction = target - transform.translation;
                transform.translation += direction.normalize() * MOVEMENT_SPEED * time.delta_secs();
                transform.look_at(target, Vec3::Y);
                if villager.fsm.is_finished(transform.translation) {
                    action = match villager.fsm.state {
                        FSMState::WalkingToGather(target, _) => FSMDecision::Gather(target, 1.0),
                        _ => FSMDecision::Finished,
                    }
                }
            }
            FSMState::Gathering(entity, _) => {
                if villager.fsm.is_finished(transform.translation) {
                    action = FSMDecision::Finished;
                    commands.entity(entity).despawn_recursive();
                }
            }
            _ => {}
        }

        println!("state: {:?}, action: {:?}", villager.fsm.state, action);

        villager.fsm.state = villager.fsm.update(action, time.delta_secs());
    }
}

const TREE_GROW_RATE: f32 = 0.5;

fn grow_tree(mut commands: Commands, scene_assets: Res<SceneAssets>, time: Res<Time>) {
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
