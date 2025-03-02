const WORLD_RADIUS: f32 = 7.0;

mod assets;
mod fsm;
mod harvestable;
mod item_drop;
mod structure;
mod villager;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use structure::wood_hut::spawn_wood_hut;

use crate::assets::*;
use crate::fsm::*;
use crate::harvestable::{harvestable::*, tree::*};
use crate::item_drop::*;
use crate::structure::house::*;
use crate::villager::villager::*;

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_event::<HarvestableDestroyed>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Village".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, exit_on_escape)
        .add_plugins(FSMPlugin)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, update_wood_stacks)
        .add_systems(
            Update,
            (
                tick_grow_tree,
                delete_underworld,
                check_harvestable_destroyed,
                check_tree_should_be_destroyed,
            ),
        )
        .add_systems(PostUpdate, update_wood_stacks)
        .run();
}

#[derive(Component)]
#[require(Sensor)]
struct UnderworldDeleter;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_assets: Res<SceneAssets>,
) {
    // circular base
    commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(WORLD_RADIUS))),
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
            MeshMaterial3d(materials.add(Color::srgb_u8(50, 200, 50))),
            RigidBody::Fixed,
        ))
        .with_children(|this| {
            this.spawn((
                Collider::cylinder(1.0, WORLD_RADIUS),
                Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                    .with_translation(Vec3::new(0.0, 0.0, -1.0)),
            ));
        });

    commands.spawn((
        Collider::cylinder(1.0, 2.0 * WORLD_RADIUS),
        Transform::from_translation(Vec3::new(0.0, -3.1, 0.0)),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::DYNAMIC_STATIC,
        UnderworldDeleter,
    ));

    // Houses
    spawn_house(&mut commands, &scene_assets, Vec3::new(3.0, 0.0, 1.0));
    spawn_wood_hut(
        &mut commands,
        &scene_assets,
        Vec3::new(0.633975, 0.0, 3.09808),
    );

    // Villagers
    spawn_villager(&mut commands, &scene_assets, Vec3::new(0.0, 0.0, 0.0));
    // spawn_villager(&mut commands, &scene_assets, Vec3::new(0.0, 0.0, 0.0));

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

fn delete_underworld(
    mut collision_events: EventReader<CollisionEvent>,
    underworld_deleter: Query<Entity, With<UnderworldDeleter>>,
    parent_query: Query<&Parent, With<Collider>>,
    mut commands: Commands,
) {
    if collision_events.is_empty() {
        return;
    }

    let underworld_deleter = underworld_deleter.get_single().unwrap();

    let collision_events = collision_events.read().collect::<Vec<_>>();
    for event in collision_events.iter() {
        let mut to_despawn = Entity::PLACEHOLDER;
        match **event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if entity1 == underworld_deleter {
                    to_despawn = entity2;
                }
                if entity2 == underworld_deleter {
                    to_despawn = entity1;
                }
            }
            _ => {}
        }
        if to_despawn != Entity::PLACEHOLDER {
            if let Ok(parent) = parent_query.get(to_despawn) {
                // Lesson: The entity is what has the collider. Sometimes this is a child of something else
                commands.entity(parent.get()).despawn_recursive();
            } else {
                commands.entity(to_despawn).despawn_recursive();
            }
        }
    }
}

fn exit_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut exit: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}