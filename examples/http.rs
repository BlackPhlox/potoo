use bevy::prelude::*;

use bevy_codegen::{bevy_model_template::default_game_template, model::BevyModel};
use client::{PotooClient, PotooClientConfig};
use server::PotooServer;

fn main() {
    PotooServer::default().start();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PotooClient)
        .insert_resource(PotooClientConfig {
            ..Default::default()
        })
        .insert_resource(default_game_template())
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
