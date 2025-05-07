// crates/forge_ui/src/theme/runtime/mod.rs
use bevy::{prelude::*, reflect::TypePath};

use crate::assets::FontAssets;
use crate::plugin::UiConfig;
use crate::theme::data::UiThemeData;

pub mod color;
pub mod layout;
pub mod typography;

pub use color::{UiAccentColorPalette, UiColorPalette, UiColorPalettes, UiGrayAccentColorPalette};
pub use layout::{UiLayout, UiRadius, UiSpacing};
pub use typography::{FontVariants, UiFontFamilies, UiFontSize, UiTypography};

#[derive(Debug, Clone, Asset, TypePath, Resource)]
pub struct UiTheme {
    pub font: UiTypography,
    pub layout: UiLayout,
    pub color: UiColorPalettes,
    pub accent_color: UiColorPalette,
    pub gray_accent_color: UiColorPalette,
}

impl UiTheme {
    /// Baut aus den rohen Theme-Daten und der Config das Runtime-Theme
    pub fn build_from_data(
        font_assets: Res<FontAssets>,
        data: &UiThemeData,
        config: &UiConfig,
    ) -> Self {
        let font = typography::build(&font_assets, &data.font, config);
        let layout = layout::build(&data.layout, config);
        let color = color::build_palettes(&data.color);
        let accent_color = color::build_palette(&data.accent_color);
        let gray_accent_color = color::build_palette(&data.gray_accent_color);

        UiTheme {
            font,
            layout,
            color,
            accent_color,
            gray_accent_color,
        }
    }
}
