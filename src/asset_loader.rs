use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SceneAssets {
        asteroid: asset_server.load("Asteroid.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
    });
}
