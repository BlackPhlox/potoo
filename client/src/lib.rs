mod code_editor;
mod syntax_highlighting;

use std::thread;

use bevy::prelude::{App, Plugin, ResMut, Resource, World};
use bevy_editor_pls::{
    default_windows::hierarchy::HierarchyWindow,
    editor_window::{EditorWindow, EditorWindowContext},
    egui, AddEditorWindow, EditorPlugin,
};
use code_editor::{CodeEditor, View};
use mini_redis::blocking_client::BlockingClient;

use mini_redis::blocking_client::connect;

impl Plugin for PotooClient {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        app.init_resource::<PotooClientConfig>()
            .init_resource::<CodeEditor>()
            .add_startup_system(setup_client)
            .add_plugin(EditorPlugin)
            .add_editor_window::<PotooClientEditorWindow>();
    }
}
pub struct PotooClient;

#[derive(Resource)]
pub struct PotooClientConfig {
    client: Option<BlockingClient>,
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
    pub fn start(&mut self) -> () {
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
            for sub_message in blocking_iter {
                if let Ok(msg) = sub_message {
                    println!("In: {msg:?}");
                }
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
        let currently_inspected = &cx.state::<HierarchyWindow>().unwrap().selected;

        if let Some(mut ce) = world.get_resource_mut::<CodeEditor>() {
            ce.ui(ui);
            let code = ce.code.clone();
            ui.label("Anything can go here");
            if ui.button("Add System").clicked() {
                if let Some(mut client_config) = world.get_resource_mut::<PotooClientConfig>() {
                    if let Some(client) = &mut client_config.client {
                        let _ = client.publish("ch1", code.into());
                    }
                }
            }
        };
    }
}
