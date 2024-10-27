use bevy::prelude::*;
mod debug;
mod movement;
mod spaceship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // user plugins
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
