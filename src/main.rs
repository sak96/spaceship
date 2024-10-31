use bevy::prelude::*;
mod asset_loader;
mod asteroid;
mod camera;
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
        .add_plugins(camera::CameraPlugin)
        .add_plugins(asteroid::AsteroidPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .run();
}
