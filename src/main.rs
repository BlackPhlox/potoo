pub mod history;
pub mod templates;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

/*
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::App, winit::WinitSettings, DefaultPlugins,
};
*/
use bevy_codegen::{
    generate::GenerationType,
    model::{BevyModel, Component, ConfirmPo2Version, Po2Version, ReadPo2Version, System},
    templates::default_cargo_src_template,
};
//use bevy_editor_pls::prelude::*;
use codegen::Scope;
use history::PotooEvent::*;
use history::{PotooEvents, ProjectModel};
use rust_format::{Formatter, RustFmt};
use templates::default_game_template;
use undo::History;

fn main() {
    /*
    let run_app = false;
    if run_app {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(EditorPlugin)
            .insert_resource(WinitSettings::desktop_app())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .run();
    }
    */
    let bm = default_game_template();
    let mut pm = ProjectModel {
        model: bm,
        history: History::new(),
    };

    pm.apply(PotooEvents(AddComponent(Component {
        name: "OtherShip".to_string(),
        ..Default::default()
    })));

    pm.apply(PotooEvents(AddComponent(Component {
        name: "Bullet".to_string(),
        ..Default::default()
    })));

    /*
    pm.apply(PotooEvents(AddRunTimeSystem(System{
        name: "hello_world_system".to_string(),
        param: vec![],
        content: "println!(\"Hello World\")".to_string(),
        visibility: "pub".to_string(),
        attributes: vec!["no_mangle".to_string()],
    })));
    */
    
    println!("History:");
    println!("{:?}", pm.history);

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

        let _ = fs::create_dir_all(&bevy_folder);
        let po2_filename = format!("/{bevy_folder}.po2.json");
        let mut cargo_file = File::create(bevy_folder.clone() + &po2_filename).unwrap();
        let ser_prep = ReadPo2Version {
            po2_version: Po2Version::default(),
            model: pm.model,
        };
        let _ = cargo_file.write_all(serde_json::to_string(&ser_prep).unwrap().as_bytes());

        let a = read_po2_file(format!("{bevy_folder}{po2_filename}"));
        println!("Res {a:?}");

        /*
        cargo watch -w systems -x "build -p systems --features dynamic" --ignore-nothing
        cargo run --features reload --target-dir "target-bin"
        */

        let _b1 = Command::new("cargo")
            .arg("build")
            .args(["-p", "systems"])
            .args(["--features", " dynamic"])
            .current_dir(bevy_folder.clone())
            .status();

        let _b2 = Command::new("cargo")
            .arg("build")
            .args(["--features", "reload"])
            .args(["--target-dir", "target-bin"])
            .current_dir(bevy_folder.clone())
            .status();

        let _run_watch = Command::new("cargo")
            .arg("watch")
            .args(["-w", "systems"])
            .args(["-w", "components"])
            .args(["-x", "build -p systems --features dynamic"])
            .arg("--ignore-nothing")
            .current_dir(bevy_folder.clone())
            .spawn();

        let _run_reload = Command::new("cargo")
            .arg("run")
            .args(["--features", "reload"])
            .args(["--target-dir", "target-bin"])
            .current_dir(bevy_folder)
            .status();
    }
}

#[must_use]
fn read_po2_file(path: String) -> BevyModel {
    let file_as_string = fs::read_to_string(path).unwrap();
    let parsed_version_file = serde_json::from_str::<ReadPo2Version>(&file_as_string).unwrap();
    let compatible = match parsed_version_file.po2_version {
        Po2Version::V0_0_1 => true,
        parsed_po2_version => {
            println!(
                "Warning: File is not compatible ({parsed_po2_version}) with current version {}",
                Po2Version::default()
            );
            false
        }
    };
    if !compatible {
        //TODO: Do something
    }
    let parsed_file = serde_json::from_str::<ConfirmPo2Version>(&file_as_string).unwrap();
    parsed_file.model
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
