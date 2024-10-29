use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug)]
pub struct Acceleration(pub Vec3);

#[derive(Bundle)]
pub struct MovementObjectBundle {
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_acceleration, update_position));
    }
}

fn update_acceleration(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
