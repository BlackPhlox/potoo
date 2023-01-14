pub mod templates;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, KeyCode},
    winit::WinitSettings,
    DefaultPlugins,
};
use bevy_codegen::model::BevyModel;
use bevy_editor_pls::{controls, prelude::*};
use templates::default_game_template;

fn main() {
    /*App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .run();
    */

    let bm = default_game_template();
    write_src_folder();
    write_components_folder();
    write_systems_folder();
    write_outer_cargo();
}

fn write_outer_cargo() {}

fn write_src_folder() {}

fn write_components_folder() {}

fn write_systems_folder() {}
