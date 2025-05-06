// crates/forge_ui/src/theme/runtime/mod.rs
use bevy::{prelude::*, reflect::TypePath};

use crate::plugin::UiConfig;
use crate::theme::data::UiThemeData;

pub mod color;
pub mod layout;
pub mod typography;

pub use color::{UiColor, UiColors};
pub use layout::{UiLayout, UiRadius, UiSpacing};
pub use typography::{FontVariants, UiFontFamilies, UiFontSize, UiTypography};

#[derive(Debug, Clone, Asset, TypePath, Resource)]
pub struct UiTheme {
    pub ui_scaling: f32,
    pub rem: f32,
    pub font: UiTypography,
    pub layout: UiLayout,
    pub color: UiColors,
}

impl UiTheme {
    /// Baut aus den rohen Theme-Daten und der Config das Runtime-Theme
    pub fn build_from_data(
        asset_server: &AssetServer,
        data: &UiThemeData,
        config: &UiConfig,
    ) -> Self {
        let ui_scaling = data.ui_scaling;
        let rem = config.rem.unwrap_or(data.rem);

        let font = typography::build(asset_server, &data.font, rem, ui_scaling);
        let layout = layout::build(&data.layout, rem, ui_scaling);
        let color = color::build(&data.color);

        UiTheme {
            ui_scaling,
            rem,
            font,
            layout,
            color,
        }
    }
}
