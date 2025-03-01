use bevy::prelude::*;

pub fn walk_to(
    transform: &mut Transform,
    target_transform: &Transform,
    movement_speed: f32,
    time: &Res<Time>,
) {
    // Maybe later we can use Rapier RigidBodies and set the velocity
    let direction = target_transform.translation - transform.translation;
    transform.translation += direction.with_y(0.0).normalize() * movement_speed * time.delta_secs();
    transform.look_at(transform.translation + direction, Vec3::Y);
}
