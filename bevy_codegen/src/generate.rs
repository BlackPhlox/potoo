use codegen::{Field, Function, Scope, Struct};
use rust_format::{Formatter, RustFmt};

use crate::model::{Component, Plugin, System, BevyType, BevyModel};

impl BevyModel {
    pub fn generate(&self) -> Scope {
        let mut scope = Scope::new();

        if self.meta.bevy_type.eq(&BevyType::Example) {
            scope.import("bevy_test", "BevyTest");
        }

        let mut plugin_app_code: String = "".into();
        for plugin in &self.plugins {
            if plugin.is_group {
                plugin_app_code.push_str(format!(".add_plugins({})", &plugin.name).as_str());
            } else {
                plugin_app_code.push_str(format!(".add_plugin({})", &plugin.name).as_str());
            }
        }

        let mut startup_system_app_code: String = "".into();
        for system in &self.startup_systems {
            startup_system_app_code
                .push_str(format!(".add_startup_system({})", &system.name).as_str());
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
            BevyType::Plugin(name) => scope.create_plugin(Plugin{ name: name.to_string(), is_group: false, dependencies: vec![] }, &app_code_merge),
            BevyType::PluginGroup(name) => scope.create_plugin(Plugin { name: name.to_string(), is_group: true, dependencies: vec![] }, &app_code_merge),
            _ => scope.create_app(&app_code_merge),
        };

        for component in &self.components {
            scope.create_component(Component { name: component.name.clone(), content: component.content.clone() });
        }

        for system in &self.startup_systems {
            scope.create_query(system.clone());
        }
        for system in &self.systems {
            scope.create_query(system.clone());
        }
        scope
    }
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
            .line(format!("App::new(){}.run();", inner_content))
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
        if &system.visibility.len() > &0 {
            fun.vis(&system.visibility);
        }
        for att in &system.attributes {
            fun.attr(att);
        }
        fun.line(system.content.clone())
    }

    fn create_component(&mut self, component: Component) -> &mut Struct {
        let a = self.new_struct(&component.name);
        for (n, t) in component.content.iter() {
            a.push_field(Field::new(n, t));
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
struct TestPlugin;
"#
        );
    }
}
