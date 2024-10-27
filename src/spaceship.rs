use bevy::prelude::*;
pub struct SpaceshipPlugin;

use crate::movement::Velocity;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::new(1.0, 1.0, 0.0),
        },
    ));
}
