// src/theme/data/mod.rs
mod color;
mod layout;
mod typography;

pub use color::{
    UiAccentColorPaletteData, UiColorPaletteData, UiColorPalettesData, UiGrayAccentColorPaletteData,
};
pub use layout::{UiLayoutData, UiRadiusData, UiSpacingData};
pub use typography::{FontVariantsData, UiFontFamiliesData, UiFontSizeData, UiTypographyData};

use bevy::asset::Asset;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Asset, Reflect)]
#[serde(default)]
pub struct UiThemeData {
    pub appearance: String,
    pub high_contrast: bool,
    pub ui_scaling: f32,
    pub rem: f32,

    pub font: UiTypographyData,
    pub layout: UiLayoutData,
    pub color: UiColorPalettesData,
    pub accent_color: UiAccentColorPaletteData,
    pub gray_accent_color: UiGrayAccentColorPaletteData,
}

impl Default for UiThemeData {
    fn default() -> Self {
        const DEFAULT_REM: f32 = 16.0;
        const DEFAULT_UI_SCALING: f32 = 1.0;

        UiThemeData {
            appearance: "light".to_string(),
            high_contrast: false,
            ui_scaling: DEFAULT_UI_SCALING,
            rem: DEFAULT_REM,
            font: UiTypographyData::default(),
            layout: UiLayoutData::default(),
            color: UiColorPalettesData::default(),
            accent_color: UiAccentColorPaletteData::default(),
            gray_accent_color: UiGrayAccentColorPaletteData::default(),
        }
    }
}
