// src/theme/data/mod.rs
mod color;
mod layout;
mod typography;

pub use color::{UiColorPaletteData, UiColorPalettesData};
pub use layout::{UiLayoutData, UiRadiusData, UiSpacingData};
pub use typography::{FontVariantsData, UiFontFamiliesData, UiFontSizeData, UiTypographyData};

use bevy::asset::Asset;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Asset, Reflect)]
#[serde(default)]
pub struct UiThemeData {
    pub font: UiTypographyData,
    pub layout: UiLayoutData,
    pub color: UiColorPalettesData,
    pub accent_color: UiColorPaletteData,
    pub gray_accent_color: UiColorPaletteData,
}

impl Default for UiThemeData {
    fn default() -> Self {
        UiThemeData {
            font: UiTypographyData::default(),
            layout: UiLayoutData::default(),
            color: UiColorPalettesData::default_light(),
            accent_color: UiColorPalettesData::default_light().blue,
            gray_accent_color: UiColorPalettesData::default_light().gray,
        }
    }
}
