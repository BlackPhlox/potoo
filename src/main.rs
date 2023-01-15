pub mod history;
pub mod templates;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, KeyCode},
    winit::WinitSettings,
    DefaultPlugins,
};
use bevy_codegen::{
    generate::GenerationType,
    model::{BevyModel, Component},
    templates::default_cargo_src_template,
};
use bevy_editor_pls::{controls, prelude::*};
use codegen::Scope;
use history::{PotooEvents, ProjectModel};
use rust_format::{Formatter, RustFmt};
use templates::default_game_template;
use undo::History;

fn main() {
    /*App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .run();
    */

    let bm = default_game_template();
    let mut pm = ProjectModel {
        model: bm,
        history: History::new(),
    };
    pm.apply(PotooEvents(history::PotooEvent::Component(Component {
        name: "Hello".to_string(),
        content: vec![],
    })));

    println!("Raw:\n");
    println!("{:?}\n", pm.model);

    println!("Simplified Pretty:\n");
    println!("{}\n", pm.model);

    println!("Codegen format:\n");
    let cg = pm
        .model
        .generate_code(Scope::new(), GenerationType::All);
    println!("{:?}\n", cg);

    //Write to file
    let _ = pm.model.generate(GenerationType::Main);
    let _ = pm.model.generate(GenerationType::Components);
    let _ = pm.model.generate(GenerationType::Systems);

    println!("Codegen result:\n");
    let res = cg.to_string();
    println!("{:?}\n", res);

    println!("Prettified Codegen result:\n");
    let pretty_res = RustFmt::default().format_str(res).unwrap();
    println!("{:?}\n", pretty_res);

    println!("Cargo Toml:\n");
    let toml = default_cargo_src_template(&pm.model);
    println!("{:?}\n", toml);
}
