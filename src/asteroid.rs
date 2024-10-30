use crate::movement::{Acceleration, MovementObjectBundle, Velocity};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 3.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const VELOCITY_SCALAR: f32 = 5.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct AsteroidSpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroid);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let mut rng = rand::thread_rng();
        let translation = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.,
            rng.gen_range(SPAWN_RANGE_Z),
        );
        let mut random_unit_vector = || {
            Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero()
        };
        let velocity = random_unit_vector() * VELOCITY_SCALAR;
        let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

        commands.spawn((
            MovementObjectBundle {
                acceleration: Acceleration(acceleration),
                velocity: Velocity(velocity),
                model: SceneBundle {
                    scene: asset_server.load("Asteroid.glb#Scene0"),
                    transform: Transform::from_translation(translation),
                    ..default()
                },
            },
            Asteroid,
        ));
    }
}