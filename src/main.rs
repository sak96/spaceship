use bevy::prelude::*;
mod asset_loader;
mod asteroid;
mod camera;
mod collider;
mod movement;
mod schedule;
mod spaceship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }))
        // user plugins
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(asteroid::AsteroidPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(collider::CollisionPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .run();
}
