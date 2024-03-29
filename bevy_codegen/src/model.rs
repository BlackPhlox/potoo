use std::fmt::Display;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::parse::ParseBevyModel;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ReadPo2Version {
    pub po2_version: Po2Version,
    #[serde(skip_deserializing)]
    pub model: BevyModel,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ConfirmPo2Version {
    pub po2_version: Po2Version,
    pub model: BevyModel,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Default, Debug, Resource)]
pub struct BevyModel {
    pub meta: Meta,
    pub bevy_settings: Settings,
    pub plugins: Vec<Plugin>,
    pub components: Vec<Component>,
    pub startup_systems: Vec<System>,
    pub systems: Vec<System>,
    pub custom: Vec<Custom>,
    pub imports: Vec<Import>,
    pub examples: Vec<BevyModel>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum BevyType {
    App,
    Plugin(String),
    PluginGroup(String),
    Example,
}
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub name: String,
    pub bevy_type: BevyType,
    pub asset_path: String,
    pub po2_version: Po2Version,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "bevy_default_meta".to_string(),
            bevy_type: BevyType::App,
            asset_path: "assets".to_string(),
            po2_version: Default::default(),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug, Default)]
#[non_exhaustive]
pub enum Po2Version {
    // Default points to the latest format version
    #[default]
    V0_0_1 = 0,
}

impl Display for Po2Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = format!("{self:?}");
        v.remove(0);
        v = v.replace('_', ".");
        let _ = write!(f, "{v}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ser_and_deserialize() {
        let def_bm = BevyModel {
            meta: Meta {
                //name: "HelloWorld".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        let wrap = ReadPo2Version {
            po2_version: Po2Version::V0_0_1,
            model: def_bm.clone(),
        };
        let json = serde_json::to_string(&wrap).unwrap();
        let obj = serde_json::from_str::<ReadPo2Version>(json.as_str()).unwrap();
        assert_eq!(def_bm, obj.model);
    }

    #[test]
    fn converts_to_the_correct_version() {
        assert_eq!("0.0.1", Po2Version::V0_0_1.to_string())
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct System {
    pub name: String,
    pub param: Vec<(String, String)>,
    pub content: String,
    pub visibility: String,
    pub attributes: Vec<String>,
}

impl Default for System {
    fn default() -> Self {
        Self {
            name: "test_system".to_string(),
            param: vec![],
            content: r#"println("Hello Bevy!")"#.to_string(),
            attributes: vec!["no_mangle".to_string()],
            visibility: "pub".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub name: String,
    pub content: Vec<(String, String)>,
    pub is_reflected: bool,
    pub attributes: Vec<String>,
    pub derives: Vec<String>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: "TestComponent".to_string(),
            content: Default::default(),
            attributes: Default::default(),
            derives: Default::default(),
            is_reflected: true,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Plugin {
    pub name: String,
    pub is_group: bool,
    pub dependencies: Vec<CargoDependency>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum Custom {
    Main(CustomCode),
    Component(CustomCode),
    System(CustomCode),
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct CustomCode {
    pub name: String,
    pub content: String,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            name: "DefaultPlugins".to_string(),
            is_group: true,
            dependencies: Default::default(),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Default, Debug)]
pub struct CargoDependency {
    pub name: String,
    pub dependency_type: DependencyType,
    pub paths: Vec<String>,
    pub features: Vec<String>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum DependencyType {
    Crate(String),
    Git(String, Option<String>, Option<String>),
    Path(String),
    Internal,
}

impl Default for DependencyType {
    fn default() -> Self {
        DependencyType::Crate("*".to_string())
    }
}

impl Display for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{self:?}");
        Ok(())
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug, Default)]
pub enum Used {
    #[default]
    Main,
    Components,
    Systems,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Default, Debug)]
pub struct Import {
    pub used: Used,
    pub dependency: CargoDependency,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Default, Debug)]
pub struct Settings {
    pub features: Vec<Feature>,
    pub dev_features: Vec<Feature>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum Feature {
    Default,
    BevyAudio,
    BevyGilrs,
    BevyWinit,
    Render,
    Png,
    Hdr,
    Vorbis,
    X11,
    FilesystemWatcher,
    TraceChrome,
    TraceTracy,
    Wayland,
    WgpuTrace,
    BevyCiTesting,
    BevySprite,
    Dynamic,
    BevyUi,
    Tga,
    Serialize,
    Mp3,
    BevyCorePipeline,
    Wav,
    Trace,
    SubpixelGlyphAtlas,
    Bmp,
    BevyGltf,
    Dds,
    BevyDynamicPlugin,
    BevyRender,
    BevyText,
    BevyAsset,
    Flac,
    BevyPbr,
    Jpeg,
    BevyDylib,
}

impl Feature {
    pub fn to_feature(&self) -> &'static str {
        match self {
            Feature::Default => "default",
            Feature::BevyAudio => "bevy_audio",
            Feature::BevyGilrs => "bevy_gilrs",
            Feature::BevyWinit => "bevy_winit",
            Feature::Render => "render",
            Feature::Png => "png",
            Feature::Hdr => "hdr",
            Feature::Vorbis => "vorbis",
            Feature::X11 => "x11",
            Feature::FilesystemWatcher => "filesystem_watcher",
            Feature::TraceChrome => "trace_chrome",
            Feature::TraceTracy => "trace_tracy",
            Feature::Wayland => "wayland",
            Feature::WgpuTrace => "wgpu_trace",
            Feature::BevyCiTesting => "bevy_ci_testing",
            Feature::BevySprite => "bevy_sprite",
            Feature::Dynamic => "dynamic",
            Feature::BevyUi => "bevy_ui",
            Feature::Tga => "tga",
            Feature::Serialize => "serialize",
            Feature::Mp3 => "mp3",
            Feature::BevyCorePipeline => "bevy_core_pipeline",
            Feature::Wav => "wav",
            Feature::Trace => "trace",
            Feature::SubpixelGlyphAtlas => "subpixel_glyph_atlas",
            Feature::Bmp => "bmp",
            Feature::BevyGltf => "bevy_gltf",
            Feature::Dds => "dds",
            Feature::BevyDynamicPlugin => "bevy_dynamic_plugin",
            Feature::BevyRender => "bevy_render",
            Feature::BevyText => "bevy_text",
            Feature::Flac => "flac",
            Feature::BevyPbr => "bevy_pbr",
            Feature::Jpeg => "jpeg",
            Feature::BevyDylib => "bevy_dylib",
            Feature::BevyAsset => "bevy_asset",
        }
    }
}

impl Display for BevyModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "BevyModel:");

        let _ = writeln!(f, "   Meta:");

        let _ = writeln!(f, "       {:?}", &self.meta);

        let _ = writeln!(f, "   Components:");
        self.components.iter().for_each(|d| {
            let _ = writeln!(f, "       {}", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Startup Systems:");
        self.startup_systems.iter().for_each(|d| {
            let _ = writeln!(f, "       {}", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Runtime Systems:");
        self.systems.iter().for_each(|d| {
            let _ = writeln!(f, "       {},", d.name);
        });

        let _ = writeln!(f);

        let _ = writeln!(f, "   Plugins:");
        self.plugins.iter().for_each(|d| {
            let _ = writeln!(f, "       {d:?},");
        });

        Ok(())
    }
}

impl From<ParseBevyModel> for BevyModel {
    fn from(value: ParseBevyModel) -> Self {
        let imports = value.imports.into_iter().map(|f| {
            let name = f.split("::").next().unwrap().to_string();
            Import {
                dependency: CargoDependency {
                    dependency_type: DependencyType::Crate(name.clone()),
                    name,
                    paths: vec![f],
                    ..Default::default()
                },
                ..Default::default()
            }
        });
        BevyModel {
            imports: imports.collect(),
            ..Default::default()
        }
    }
}
