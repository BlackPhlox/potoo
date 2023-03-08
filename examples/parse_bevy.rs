use bevy_codegen::{model::BevyModel, parse::parse_file};

fn main() {
    let syntax = syn::parse_file(
        r#"
use bevy::prelude::*;

fn main() {

App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(fade_transparency)
    .run();
}"#,
    )
    .expect("Unable to parse file");
    let pbm = parse_file(syntax);
    println!("{pbm:?}");
    if let Some(bevy_model) = pbm {
        let bevy_model: BevyModel = bevy_model.into();
        println!("{bevy_model:?}");
    }
}
