// crates/forge_ui/src/theme/runtime/mod.rs
use bevy::{prelude::*, reflect::TypePath};

use crate::assets::FontAssets;
use crate::theme::data::UiThemeData;

pub mod appearance;
pub mod color;
pub mod layout;
pub mod typography;

pub use appearance::Appearance;
pub use color::{UiAccentColorPalette, UiColorPalette, UiColorPalettes, UiGrayAccentColorPalette};
pub use layout::{UiLayout, UiRadius, UiSpacing};
pub use typography::{FontVariants, UiFontFamilies, UiFontSize, UiTypography};

#[derive(Debug, Clone, Asset, TypePath, Resource)]
pub struct UiTheme {
    pub appearance: Appearance,
    pub high_contrast: bool,
    pub ui_scaling: f32,
    pub rem: f32,
    pub font: UiTypography,
    pub layout: UiLayout,
    pub color: UiColorPalettes,
    pub accent_color: UiAccentColorPalette,
    pub gray_accent_color: UiGrayAccentColorPalette,
}

impl UiTheme {
    /// Baut aus den rohen Theme-Daten und der Config das Runtime-Theme
    pub fn build_from_data(font_assets: Res<FontAssets>, data: &UiThemeData) -> Self {
        let appearance = Appearance::from_str(&data.appearance);
        let high_contrast = data.high_contrast;
        let ui_scaling = data.ui_scaling;
        let rem = data.rem;

        let font = typography::build(&font_assets, &data.font, rem, ui_scaling);
        let layout = layout::build(&data.layout, rem, ui_scaling);
        let color = color::build(&data.color);
        let accent_color = color::build_accent_color(&data.accent_color);
        let gray_accent_color = color::build_gray_accent_color(&data.gray_accent_color);

        UiTheme {
            appearance,
            high_contrast,
            ui_scaling,
            rem,
            font,
            layout,
            color,
            accent_color,
            gray_accent_color,
        }
    }
}
