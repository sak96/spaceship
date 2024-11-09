use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;
const VIEWPORT_MAX_DISTANCE: f32 = 100.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)));
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, despawn_entities_out_of_viewport);
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

fn despawn_entities_out_of_viewport(mut commands: Commands, query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation.distance(Vec3::ZERO) > VIEWPORT_MAX_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
