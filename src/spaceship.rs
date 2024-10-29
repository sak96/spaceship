use crate::movement::{Acceleration, MovementObjectBundle, Velocity};
use bevy::prelude::*;

const START_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const START_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MovementObjectBundle {
        acceleration: Acceleration(Vec3::ZERO),
        velocity: Velocity(START_VELOCITY),
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(START_TRANSLATION),
            ..default()
        },
    });
}
