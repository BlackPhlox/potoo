use bevy::{DefaultPlugins, prelude::{App, KeyCode}, diagnostic::FrameTimeDiagnosticsPlugin, winit::WinitSettings};
use bevy_editor_pls::{prelude::*, controls};

fn main() {
    /*App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .run();
    */
    write_src_folder();
    write_components_folder();
    write_systems_folder();
    write_outer_cargo();
}

fn write_outer_cargo() {

}

fn write_src_folder(){

}

fn write_components_folder(){

}

fn write_systems_folder(){

}

