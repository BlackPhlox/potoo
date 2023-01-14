use codegen::{Field, Function, Scope, Struct};

trait BevyCodegen {
    fn create_app(&mut self, inner_content: &str) -> &mut Function;

    fn create_plugin(&mut self, name: &str, is_group: bool, content: &str) -> &mut Function;

    fn create_query(&mut self, system: &crate::model::System) -> &mut Function;

    fn create_component(&mut self, name: &str, content: Vec<(String, String)>) -> &mut Struct;
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
        fun.vis(&system.visibility);
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
}


#[cfg(test)]
mod tests {
    use crate::model::System;

    use super::*;

    #[test]
    fn create_app_works() {
        let mut scp = Scope::new();
        scp.create_app(".add_plugins(DefaultPlugins)");
        assert_eq!(scp.to_string(), "fn main() {\n    App::new().add_plugins(DefaultPlugins).run();\n}");
    }

    #[test]
    fn create_plugin_works() {
        let mut scp = Scope::new();
        scp.create_plugin("TestPlugin", false, "");
        assert_eq!(scp.to_string(), "pub struct TestPlugin;\n\nimpl Plugin for TestPlugin {\n    fn build(&self, app: &mut App) {\n        app\n\n        ;\n    }\n}");
    }

    #[test]
    fn create_query_works() {
        let mut scp = Scope::new();
        scp.create_query(&System{ name: "test".to_string(), param: vec![], content: "".to_string(), visibility: "".to_string(), attributes: vec![] });
        assert_eq!(scp.to_string(), " fn test() {\n\n}");
    }

    #[test]
    fn create_component_works() {
        let mut scp = Scope::new();
        scp.create_component("TestPlugin", vec![]);
        assert_eq!(scp.to_string(), "#[derive(Component)]\nstruct TestPlugin;");
    }
}
