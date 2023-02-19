use bevy_codegen::parse::parse_file;

fn main() {
    let syntax = syn::parse_file(
        r#"fn main() {
App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(fade_transparency)
    .run();
}"#,
    )
    .expect("Unable to parse file");
    parse_file(syntax);
}
