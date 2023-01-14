use codegen::{Field, Function, Scope, Struct};
use rust_format::{Formatter, RustFmt};

trait BevyCodegen {
    fn create_app(&mut self, inner_content: &str) -> &mut Function;

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function;

    fn create_query(&mut self, system: &crate::model::System) -> &mut Function;

    fn create_component(&mut self, name: &str, content: Vec<(String, String)>) -> &mut Struct;

    fn generate(&mut self) -> String;
}

impl BevyCodegen for Scope {
    fn create_app(&mut self, inner_content: &str) -> &mut Function {
        self.new_fn("main")
            .line(format!("App::new(){}.run();", inner_content))
    }

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function {
        self.new_struct(name).vis("pub");
        let plugin_impl = match is_group {
            false => self.new_impl(name).impl_trait("Plugin"),
            true => self.new_impl(name).impl_trait("Plugins"),
        };
        plugin_impl
            .new_fn("build")
            .arg_ref_self()
            .arg("app", "&mut App")
            .line("app")
            .line(content)
            .line(";")
    }

    fn create_query(&mut self, system: &crate::model::System) -> &mut Function {
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

    fn create_component(&mut self, name: &str, content: Vec<(String, String)>) -> &mut Struct {
        let a = self.new_struct(name);
        for (n, t) in content.iter() {
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
        scp.create_plugin("TestPlugin", false, "");
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
        scp.create_plugin("TestPlugins", true, "");
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
        scp.create_query(&System {
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
        scp.create_query(&System {
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
        scp.create_component("TestPlugin", vec![]);
        assert_eq!(
            scp.generate(),
r#"#[derive(Component)]
struct TestPlugin;
"#
        );
    }
}
