use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BevyModel {
    pub plugins: Vec<Plugin>,
    pub components: Vec<Component>,
    pub startup_systems: Vec<System>,
    pub systems: Vec<System>,
    pub bevy_settings: Settings,
    pub meta: Meta,
    pub examples: Vec<BevyModel>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum BevyType {
    App,
    Plugin(String),
    PluginGroup(String),
    Example,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub name: String,
    pub bevy_type: BevyType,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: "bevy_default_meta".to_string(),
            bevy_type: BevyType::App,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct System {
    pub name: String,
    pub param: Vec<(String, String)>,
    pub content: String,
    pub visibility: String,
    pub attributes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub name: String,
    pub content: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plugin {
    pub name: String,
    pub is_group: bool,
    pub dependencies: Vec<PluginDependency>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PluginDependency {
    pub crate_name: String,
    pub crate_version: String,
    pub crate_paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Settings {
    pub features: Vec<Feature>,
    pub dev_features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
            let _ = writeln!(f, "       {:?},", d);
        });

        Ok(())
    }
}
