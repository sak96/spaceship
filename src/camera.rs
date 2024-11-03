use bevy::prelude::*;

use crate::spaceship::SpaceShip;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)));
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, sync_with_spaceship);
    }
}

#[derive(Component, Debug)]
pub struct MyCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCamera,
    ));
}

fn sync_with_spaceship(
    space_ship: Query<&Transform, With<SpaceShip>>,
    mut camera: Query<&mut Transform, (With<MyCamera>, Without<SpaceShip>)>,
) {
    let Ok(spaceship) = space_ship.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };

    camera_transform.translation.x = spaceship.translation.x;
    camera_transform.translation.y = spaceship.translation.y;
}
