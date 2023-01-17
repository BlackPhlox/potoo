pub mod history;
pub mod templates;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::App, winit::WinitSettings, DefaultPlugins,
};
use bevy_codegen::{
    generate::GenerationType, model::Component, templates::default_cargo_src_template,
};
use bevy_editor_pls::prelude::*;
use codegen::Scope;
use history::{PotooEvents, ProjectModel};
use rust_format::{Formatter, RustFmt};
use templates::default_game_template;
use undo::History;

fn main() {
    let run_app = false;
    if run_app {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(EditorPlugin)
            .insert_resource(WinitSettings::desktop_app())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .run();
    }

    let bm = default_game_template();
    let mut pm = ProjectModel {
        model: bm,
        history: History::new(),
    };

    pm.apply(PotooEvents(history::PotooEvent::Component(Component {
        name: "OtherShip".to_string(),
        ..Default::default()
    })));

    pm.apply(PotooEvents(history::PotooEvent::Component(Component {
        name: "Bullet".to_string(),
        ..Default::default()
    })));

    let display_info = true;

    if display_info {
        println!("Raw:\n");
        println!("{:?}\n", pm.model);

        println!("Simplified Pretty:\n");
        println!("{}\n", pm.model);
    }

    //Write to file
    let bevy_folder = pm.model.meta.name.clone();
    let already_exists = Path::new(&bevy_folder).exists();
    if already_exists {
        remove_path(bevy_folder.to_string() + "/" + "Cargo.toml");
        remove_path(bevy_folder.to_string() + "/" + "src");
        remove_path(bevy_folder.to_string() + "/" + "components");
        remove_path(bevy_folder.to_string() + "/" + "systems");
    }

    //Remove whole project
    //let res = fs::remove_dir_all(bevy_folder.to_owned());

    let _ = pm.model.generate(GenerationType::Main);
    let _ = pm.model.generate(GenerationType::Components);
    let _ = pm.model.generate(GenerationType::Systems);

    if display_info {
        println!("Codegen format:\n");
        let cg = pm.model.generate_code(Scope::new(), GenerationType::All);
        println!("{cg:?}\n");

        println!("Codegen result:\n");
        let res = cg.to_string();
        println!("{res:?}\n");

        println!("Prettified Codegen result:\n");
        let pretty_res = RustFmt::default().format_str(res).unwrap();
        println!("{pretty_res:?}\n");

        println!("Cargo Toml:\n");
        let toml = default_cargo_src_template(&pm.model);
        println!("{toml:?}\n");

        let _ = fs::create_dir_all(bevy_folder.to_string());
        let po2_filename = format!("/{}.po2.json", bevy_folder.to_string());
        let mut cargo_file = File::create(bevy_folder.to_string() + &po2_filename).unwrap();
        let _ = cargo_file.write_all(serde_json::to_string(&pm.model).unwrap().as_bytes());
    }
}

fn remove_path(path: String) {
    let already_exists = Path::new(&path).exists();
    let is_dir = Path::new(&path).is_dir();
    if already_exists {
        if is_dir {
            let _ = fs::remove_dir_all(&path);
        } else {
            let _ = fs::remove_file(&path);
        }
    }
}
