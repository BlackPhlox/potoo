use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use codegen::{Field, Function, Scope, Struct};
use rust_format::{Formatter, RustFmt};

use crate::{
    model::{BevyModel, BevyType, Component, Custom, Import, Plugin, System, Used},
    templates::{
        default_cargo_components_template, default_cargo_src_template,
        default_cargo_systems_template,
    },
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GenerationType {
    All,
    Main,
    Components,
    Systems,
}

impl BevyModel {
    pub fn generate_code(&self, mut scope: Scope, gen_type: GenerationType) -> Scope {
        match gen_type {
            GenerationType::Main => {
                let mut plugin_app_code: String = "".into();
                for plugin in &self.plugins {
                    if plugin.is_group {
                        plugin_app_code
                            .push_str(format!(".add_plugins({})", &plugin.name).as_str());
                    } else {
                        plugin_app_code.push_str(format!(".add_plugin({})", &plugin.name).as_str());
                    }
                }

                let mut startup_system_app_code: String = "".into();
                for system in &self.startup_systems {
                    startup_system_app_code.push_str(
                        format!(".add_startup_system(systems::{})", &system.name).as_str(),
                    );
                }

                let mut system_app_code: String = "".into();
                for system in &self.systems {
                    system_app_code.push_str(format!(".add_system({})", &system.name).as_str());
                }

                let mut app_code_merge: String = "".to_owned();
                app_code_merge.push_str(&plugin_app_code);
                app_code_merge.push_str(&startup_system_app_code);
                app_code_merge.push_str(&system_app_code);

                match &self.meta.bevy_type {
                    BevyType::Plugin(name) => scope.create_plugin(
                        Plugin {
                            name: name.to_string(),
                            is_group: false,
                            dependencies: vec![],
                        },
                        &app_code_merge,
                    ),
                    BevyType::PluginGroup(name) => scope.create_plugin(
                        Plugin {
                            name: name.to_string(),
                            is_group: true,
                            dependencies: vec![],
                        },
                        &app_code_merge,
                    ),
                    _ => scope.create_app(&app_code_merge),
                };
            }
            GenerationType::Components => {
                for component in &self.components {
                    scope.create_component(Component {
                        name: component.name.clone(),
                        content: component.content.clone(),
                    });
                }
            }
            GenerationType::Systems => {
                for system in &self.startup_systems {
                    scope.create_query(system.clone());
                }

                for system in &self.systems {
                    scope.create_query(system.clone());
                }
            }
            GenerationType::All => {
                let mut main_scope = self.generate_code(scope, GenerationType::Main);
                main_scope = self.generate_code(main_scope, GenerationType::Components);
                main_scope = self.generate_code(main_scope, GenerationType::Systems);
                return main_scope;
            }
        }

        /*if self.meta.bevy_type.eq(&BevyType::Example) {
            scope.import("bevy_test", "BevyTest");
        }*/

        scope
    }

    pub fn generate(&self, gen_type: GenerationType) -> std::io::Result<()> {
        let res = generate_structure(self.clone(), gen_type);
        if let Ok(mut bevy_lib_file) = res {
            //println!("Structure done");
            let r = RustFmt::default()
                .format_str(self.generate_code(Scope::new(), gen_type).to_string())
                .unwrap();
            //println!("Test: {:?}", r);
            bevy_lib_file.write_all(r.as_bytes())?;
        } else {
            println!("Generate.rs 119: {res:?}");
        }

        Ok(())
    }
}

fn import_format(import: Import) -> String {
    let name = import.dependency.crate_name;
    let ps = import.dependency.crate_paths;
    let a = "{".to_string() + &ps.join(", ") + "}";
    let s = format!("use {}::{};\n", &name, a);
    s
}

fn generate_structure(bm: BevyModel, gen_type: GenerationType) -> std::io::Result<File> {
    let folder = match gen_type {
        GenerationType::Components => "components/",
        GenerationType::Systems => "systems/",
        _ => "",
    };
    let bevy_folder = bm.meta.name.clone();
    let _ = fs::create_dir(&bevy_folder);
    if !folder.is_empty() {
        fs::create_dir_all(bevy_folder.to_owned() + "/" + folder)?
    };
    //Write cargo toml
    let cargo_path = match gen_type {
        GenerationType::Components => bevy_folder.to_owned() + "/" + folder,
        GenerationType::Systems => bevy_folder.to_owned() + "/" + folder,
        _ => bevy_folder.to_string(),
    };

    let mut cargo_file = File::create(cargo_path + "/Cargo.toml")?;
    let buf = match gen_type {
        GenerationType::All => todo!(),
        GenerationType::Main => default_cargo_src_template(&bm),
        GenerationType::Components => default_cargo_components_template(),
        GenerationType::Systems => default_cargo_systems_template(),
    };
    cargo_file.write_all(buf.as_bytes())?;

    //Write plugin or main/game
    let bevy_type_filename = match (bm.meta.clone().bevy_type, gen_type) {
        (BevyType::App, GenerationType::Main) => "/main.rs",
        (BevyType::App, GenerationType::All) => "/main.rs",
        (_, _) => "/lib.rs",
    };

    let path = bevy_folder.to_owned() + "/" + folder + "src";
    fs::create_dir_all(path.clone())?;
    let mut bevy_lib_file = File::create(path + bevy_type_filename)?;

    for cc in &bm.custom {
        match cc {
            Custom::Main(x) if gen_type.eq(&GenerationType::Main) => {
                let _ = bevy_lib_file
                    .write((format!("mod {};\n", x.name.replace(".rs", ""))).as_bytes());
            }
            Custom::Component(x) if gen_type.eq(&GenerationType::Components) => {
                let _ = bevy_lib_file
                    .write((format!("mod {};\n", x.name.replace(".rs", ""))).as_bytes());
            }
            Custom::System(x) if gen_type.eq(&GenerationType::Systems) => {
                let _ = bevy_lib_file
                    .write((format!("mod {};\n", x.name.replace(".rs", ""))).as_bytes());
            }
            _ => (),
        }
    }
    let _ = bevy_lib_file.write("\n".as_bytes());

    //Add bevy prelude
    let _ = bevy_lib_file.write(("use bevy::prelude::*;\n").as_bytes());

    for imp in bm.imports {
        match imp.used {
            Used::Main if gen_type.eq(&GenerationType::Main) => {
                let _ = bevy_lib_file.write((import_format(imp)).as_bytes());
            }
            Used::Components if gen_type.eq(&GenerationType::Components) => {
                let _ = bevy_lib_file.write((import_format(imp)).as_bytes());
            }
            Used::Systems if gen_type.eq(&GenerationType::Systems) => {
                let _ = bevy_lib_file.write((import_format(imp)).as_bytes());
            }
            _ => (),
        }
    }

    if gen_type.eq(&GenerationType::Systems) {
        let _ = bevy_lib_file.write(("use components::*;\n\n").as_bytes());
    } else {
        let _ = bevy_lib_file.write(("\n").as_bytes());
    }

    //Custom code
    for cc in bm.custom {
        match cc {
            Custom::Main(x) if gen_type.eq(&GenerationType::Main) => {
                let path = bevy_folder.to_owned() + "/src";
                let full_path = path.to_string() + &x.name;
                fs::create_dir_all(path)?;
                let mut cc_file = File::create(full_path)?;
                let _ = cc_file.write((x.content).as_bytes());
            }
            Custom::Component(x) if gen_type.eq(&GenerationType::Components) => {
                let path = bevy_folder.to_owned() + "/components/src";
                let full_path = path.to_string() + "/" + &x.name;
                fs::create_dir_all(path)?;
                let mut cc_file = File::create(full_path)?;
                let _ = cc_file.write((x.content).as_bytes());
            }
            Custom::System(x) if gen_type.eq(&GenerationType::Systems) => {
                let path = bevy_folder.to_owned() + "/systems/src";
                let full_path = path.to_string() + "/" + &x.name;
                fs::create_dir_all(path)?;
                let mut cc_file = File::create(full_path)?;
                let _ = cc_file.write((x.content).as_bytes());
            }
            _ => (),
        }
    }

    if gen_type.eq(&GenerationType::Main) {
        let _ = bevy_lib_file.write(
            (r#"#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;
        
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("systems/src/lib.rs");
}

"#)
            .as_bytes(),
        );

        //Assets
        let _ = copy_dir_all(bm.meta.asset_path, bevy_folder + "/assets");
    }

    if bm.meta.bevy_type.eq(&BevyType::App)
        && (gen_type.eq(&GenerationType::Main) || gen_type.eq(&GenerationType::All))
    {
        let _ = bevy_lib_file.write(("#[bevy_main]\n").as_bytes());
    }
    Ok(bevy_lib_file)
}

// https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

trait BevyCodegen {
    fn create_app(&mut self, inner_content: &str) -> &mut Function;

    fn create_plugin(&mut self, plugin: Plugin, content: &str) -> &mut Function;

    fn create_query(&mut self, system: System) -> &mut Function;

    fn create_component(&mut self, component: Component) -> &mut Struct;

    fn generate(&mut self) -> String;
}

impl BevyCodegen for Scope {
    fn create_app(&mut self, inner_content: &str) -> &mut Function {
        self.new_fn("main")
            .line(format!("App::new(){inner_content}.run();"))
    }

    fn create_plugin(&mut self, plugin: Plugin, content: &str) -> &mut Function {
        self.new_struct(&plugin.name).vis("pub");
        let plugin_impl = match &plugin.is_group {
            false => self.new_impl(&plugin.name).impl_trait("Plugin"),
            true => self.new_impl(&plugin.name).impl_trait("Plugins"),
        };
        plugin_impl
            .new_fn("build")
            .arg_ref_self()
            .arg("app", "&mut App")
            .line("app")
            .line(content)
            .line(";")
    }

    fn create_query(&mut self, system: System) -> &mut Function {
        let mut fun = self.new_fn(system.name.as_str());
        for (name, ty) in &system.param {
            fun = fun.arg(name, ty);
        }
        if !system.visibility.is_empty() {
            fun.vis(&system.visibility);
        }
        for att in &system.attributes {
            fun.attr(att);
        }
        fun.line(system.content.clone())
    }

    fn create_component(&mut self, component: Component) -> &mut Struct {
        let a = self.new_struct(&component.name);
        a.vis("pub");
        for (n, t) in component.content.iter() {
            let mut f = Field::new(n, t);
            f.vis("pub");
            a.push_field(f);
        }
        a.derive("Component")
    }

    fn generate(&mut self) -> String {
        RustFmt::default().format_str(self.to_string()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::System;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn create_app_works() {
        let mut scp = Scope::new();
        scp.create_app(".add_plugins(DefaultPlugins)");
        assert_eq!(
            scp.generate(),
r#"fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
"#
        );
    }

    #[test]
    #[rustfmt::skip]
    fn create_plugin_works() {
        let mut scp = Scope::new();
        scp.create_plugin(Plugin { name: "TestPlugin".to_string(), is_group: false, dependencies: vec![] }, "");
        assert_eq!(
            scp.generate(),
r#"pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
"#
        );
    }

    #[test]
    #[rustfmt::skip]
    fn create_plugin_group_works() {
        let mut scp = Scope::new();
        scp.create_plugin(Plugin{ name: "TestPlugins".to_string(), is_group: true, dependencies: vec![] }, "");
        assert_eq!(
            scp.generate(),
r#"pub struct TestPlugins;

impl Plugins for TestPlugins {
    fn build(&self, app: &mut App) {
        app;
    }
}
"#
        );
    }

    #[test]
    fn create_simple_query_works() {
        let mut scp = Scope::new();
        scp.create_query(System {
            name: "test".to_string(),
            param: vec![],
            content: "".to_string(),
            visibility: "".to_string(),
            attributes: vec![],
        });
        assert_eq!(scp.generate(), "fn test() {}\n");
    }

    #[test]
    #[rustfmt::skip]
    fn create_query_with_params_and_attribute_works() {
        let mut scp = Scope::new();
        scp.create_query(System {
            name: "test2".to_string(),
            param: vec![("field".to_string(), "Type".to_string())],
            content: "".to_string(),
            visibility: "pub".to_string(),
            attributes: vec!["no_mangle".to_string()],
        });
        assert_eq!(
            scp.generate(),
r#"#[no_mangle]
pub fn test2(field: Type) {}
"#
        );
    }

    #[test]
    #[rustfmt::skip]
    fn create_component_works() {
        let mut scp = Scope::new();
        scp.create_component(Component { name: "TestPlugin".to_string(), content: vec![] });
        assert_eq!(
            scp.generate(),
r#"#[derive(Component)]
pub struct TestPlugin;
"#
        );
    }
}
