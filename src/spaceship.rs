use crate::asset_loader::SceneAssets;
use crate::movement::{Acceleration, MovementObjectBundle, Velocity};
use bevy::prelude::*;

const START_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSLE_FORWARD_SPAWN_SCALAR: f32 = 8.0;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct SpaceShipMissile;

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovementObjectBundle {
            acceleration: Acceleration(Vec3::ZERO),
            velocity: Velocity(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(START_TRANSLATION),
                ..default()
            },
        },
        SpaceShip,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let movement = if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        1.0
    } else if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        -1.0
    } else {
        0.0
    };

    let rotation = if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        -1.0
    } else if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        1.0
    } else {
        0.0
    };

    let roll = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        -1.0
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        1.0
    } else {
        0.0
    };

    // minus is added due to model co-ordinates
    *velocity = Velocity(-transform.forward() * movement * SPACESHIP_SPEED);

    // rotate reset the existing rotate; rotate local rotates on top of existing rotate
    transform.rotate_y(rotation * time.delta_seconds() * SPACESHIP_ROTATION_SPEED);
    transform.rotate_local_z(roll * time.delta_seconds() * SPACESHIP_ROLL_SPEED);
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<SpaceShip>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovementObjectBundle {
                acceleration: Acceleration(Vec3::ZERO),
                velocity: Velocity(-transform.forward() * MISSILE_SPEED),
                model: SceneBundle {
                    scene: scene_assets.missile.clone(),
                    transform: Transform::from_translation(
                        transform.translation - transform.forward() * MISSLE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceShipMissile,
        ));
    }
}
