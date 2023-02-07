use crate::model::{BevyModel, Feature};

pub fn feature_write(features: &Vec<Feature>) -> String {
    let mut features_str = "".to_owned();
    if features.is_empty() {
        features_str.push_str("default-features = false");
    } else {
        features_str += "features = [";
        let len = features.len();
        for (i, feature) in features.iter().enumerate() {
            features_str += format!("\"{}\"", feature.to_feature()).as_str();
            if i != len - 1 {
                features_str += ",";
            }
        }
        features_str += "]";
    }
    features_str
}

pub fn default_cargo_components_template() -> String {
    r#"[package]
name = "components"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bevy = "0.9"

[features]
default = []
dynamic = ["bevy/dynamic"]
"#
    .to_string()
}

pub fn default_cargo_systems_template() -> String {
    r#"[package]
name = "systems"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["rlib", "dylib"]

[dependencies]
bevy = "0.9"
components = { path = "../components" }
log = "0.4.17"
rand = "0.8.5"

[features]
default = []
dynamic = ["bevy/dynamic", "components/dynamic"]
"#
    .to_string()
}

pub fn default_cargo_src_template(model: &BevyModel) -> String {
    let bevy_folder = model.meta.name.clone();

    let features = feature_write(&model.bevy_settings.features);
    let dev_features = feature_write(&model.bevy_settings.dev_features);

    let crate_deps = model
        .plugins
        .iter()
        .map(|d| {
            let mut s = "".to_owned();
            for b in d.dependencies.iter() {
                let k = if b.crate_version.starts_with('{') {
                    format!("{} = {}\n", b.crate_name, b.crate_version)
                } else {
                    format!("{} = \"{}\"\n", b.crate_name, b.crate_version)
                };
                s.push_str(&k);
            }
            s.to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    let buf = format!(
        r#"[package]
name = "{bevy_folder}"
version = "0.1.0"
edition = "2021"

[workspace]
resolver = "2"
members = ["systems", "components"]

# Enable only a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

# Maximize release performance with Link-Time-Optimization
[profile.release]
lto = "thin"
codegen-units = 1

[features]
default = []
reload = [
  "dep:hot-lib-reloader",
  # Make sure that the types don't change:
  "components/dynamic",
  # This is important on windows for avoiding file locking issues:
  "bevy/dynamic",
]

[dependencies]
components = {{ path = "components" }}
systems = {{ path = "systems" }}
hot-lib-reloader = {{ version = "0.6.5", optional = true }}
{crate_deps}

[dependencies.bevy]
version = "0.9"
{features}

[dev-dependencies.bevy]
version = "0.9"
{dev_features}
"#
    );
    buf
}
