use bevy::prelude::*;
use client::PotooClient;
use server::PotooServer;

fn main() {
    let _potoo_server = PotooServer::new();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PotooClient)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ship_C.png"),
        ..default()
    });
}
