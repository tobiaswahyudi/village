//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// STRUCTS AND STUFF
// MAIN

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
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::srgb_u8(50, 200, 50)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.8, 0.6, 1.0)),
        material: materials.add(Color::srgb_u8(120, 80, 10)),
        transform: Transform::from_xyz(0.0, 0.3, 0.0),
        ..default()
    });
    // Triangular prism
    commands.spawn(PbrBundle {
        mesh: meshes.add(Extrusion::new(Triangle2d::new(Vec2::new(-0.4, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.0, 0.3)), 1.0)),
        material: materials.add(Color::srgb_u8(100, 70, 10)),
        transform: Transform::from_xyz(0.0, 0.6, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}