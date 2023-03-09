mod code_editor;
mod syntax_highlighting;

use std::thread;

use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::{App, Mut, Plugin, ResMut, Resource, World},
};
use bevy_codegen::model::BevyModel;
use bevy_editor_pls::{
    default_windows::hierarchy::HierarchyWindow,
    editor_window::{EditorWindow, EditorWindowContext},
    egui::{self, ScrollArea},
    AddEditorWindow, EditorPlugin,
};
use code_editor::{CodeEditor, View};
use mini_redis::blocking_client::BlockingClient;

use mini_redis::blocking_client::connect;

impl Plugin for PotooClient {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.init_resource::<PotooClientConfig>()
            .init_resource::<CodeEditor>()
            .init_resource::<BevyModel>()
            .add_startup_system(setup_client)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            .add_plugin(EditorPlugin)
            .add_editor_window::<PotooClientEditorWindow>();
    }
}
pub struct PotooClient;

#[derive(Resource)]
pub struct PotooClientConfig {
    pub client: Option<BlockingClient>,
    pub addr: String,
    pub channels: Vec<String>,
}

impl Default for PotooClientConfig {
    fn default() -> Self {
        Self {
            client: None,
            addr: String::from("127.0.0.1:7878"),
            channels: vec!["ch1".into()],
        }
    }
}

impl PotooClientConfig {
    pub fn start(&mut self) {
        let publishing_blocking_client = connect(self.addr.clone()).unwrap();
        self.client = Some(publishing_blocking_client);

        let subscripting_blocking_client = connect(self.addr.clone()).unwrap();
        let blocking_subscriber = subscripting_blocking_client
            .subscribe(self.channels.clone())
            .unwrap();

        println!(
            "Started listening to {} on channel(s): {}",
            self.addr,
            self.channels.join(", ")
        );

        let blocking_iter = blocking_subscriber.into_iter();
        thread::spawn(|| {
            for msg in blocking_iter.flatten() {
                println!("In: {msg:?}");
            }
        });
    }
}

fn setup_client(mut potoo_config: ResMut<PotooClientConfig>) {
    potoo_config.start();
}

pub struct PotooClientEditorWindow;
impl EditorWindow for PotooClientEditorWindow {
    type State = ();
    const NAME: &'static str = "Potoo";

    fn ui(world: &mut World, cx: EditorWindowContext, ui: &mut egui::Ui) {
        let _currently_inspected = &cx.state::<HierarchyWindow>().unwrap().selected;

        world.resource_scope(|world, mut ce: Mut<CodeEditor>| {
            ce.ui(ui);
            if ui.button("Save").clicked() {
                if let Some(mut client_config) = world.get_resource_mut::<PotooClientConfig>() {
                    if let Some(client) = &mut client_config.client {
                        let _ = client.publish("ch1", ce.code.clone().into());
                    }
                }
            }

            if let Some(mut bm) = world.get_resource_mut::<BevyModel>() {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.collapsing("Meta", |ui| {
                            ui.label(format!("      {:?}", bm.meta));
                            _ = ui.button("Edit");
                        });

                        ui.collapsing("Plugins", |ui| {
                            _ = ui.button("Add Plugin (+)");
                            ui.spacing();
                            for plugin in &mut bm.plugins {
                                ui.horizontal(|ui| {
                                    //ui.label(format!("      {system:?}"));
                                    if ui.button("Edit").clicked() {
                                        ce.code = plugin.name.clone();
                                    }
                                    _ = ui.label(plugin.name.as_str());
                                    _ = ui.button("X");
                                });
                            }
                        });

                        ui.collapsing("Components", |ui| {
                            _ = ui.button("Add Component (+)");
                            for component in &bm.components {
                                ui.horizontal(|ui| {
                                    //ui.label(format!("      {system:?}"));
                                    if ui.button("Edit").clicked() {
                                        ce.code = component.name.clone();
                                    }
                                    _ = ui.label(component.name.as_str());
                                    _ = ui.button("X");
                                });
                            }
                        });

                        ui.collapsing("Startup Systems", |ui| {
                            _ = ui.button("Add Startup Systems (+)");
                            for startup_system in &bm.startup_systems {
                                ui.horizontal(|ui| {
                                    //ui.label(format!("      {system:?}"));
                                    if ui.button("Edit").clicked() {
                                        ce.code = startup_system.content.clone();
                                    }
                                    _ = ui.label(startup_system.name.as_str());
                                    _ = ui.button("X");
                                });
                            }
                        });

                        ui.collapsing("Runtime Systems", |ui| {
                            _ = ui.button("Add Runtime Systems (+)");
                            for system in &bm.systems {
                                ui.horizontal(|ui| {
                                    //ui.label(format!("      {system:?}"));
                                    if ui.button("Edit").clicked() {
                                        ce.code = system.content.clone();
                                    }
                                    _ = ui.label(system.name.as_str());
                                    _ = ui.button("X");
                                });
                            }
                        });
                    });
            }
        });
    }
}
