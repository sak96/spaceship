use bevy::prelude::*;

#[derive(Component, Debug)]
struct Velocity {
    value: Vec3,
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, update_position)
        .add_systems(Update, log_position)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::new(1.0, 1.0, 0.0),
        },
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn log_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?}",
            entity, transform.translation
        );
    }
}
