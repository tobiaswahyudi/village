const WORLD_RADIUS: f32 = 7.0;

mod assets;
mod fsm;
mod harvestable;
mod resource;
mod structure;
mod villager;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::assets::*;
use crate::harvestable::tree::*;
use crate::structure::house::*;
use crate::villager::{villager::*, actions::*};

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
        .add_systems(Update, (villager_update, grow_tree))
        .add_systems(PostUpdate, villager_cancel_if_entity_deleted)
        .run();
}

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

    // Villagers
    spawn_villager(&mut commands, &scene_assets, Vec3::new(0.0, 0.0, 0.0));
    spawn_villager(&mut commands, &scene_assets, Vec3::new(0.0, 0.0, 0.0));

    // Light
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
