// crates/forge_ui/src/theme/layout.rs

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiLayoutData {
    #[serde(default)]
    pub padding: UiSpacingData,
    #[serde(default)]
    pub margin: UiSpacingData,
    #[serde(default)]
    pub gap: UiSpacingData,
    #[serde(default)]
    pub radius: UiRadiusData,
    #[serde(default)]
    pub border: UiSpacingData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiSpacingData {
    #[serde(default)]
    pub xs: f32,
    #[serde(default)]
    pub sm: f32,
    #[serde(default)]
    pub base: f32,
    #[serde(default)]
    pub lg: f32,
    #[serde(default)]
    pub xl: f32,
    #[serde(default)]
    pub x2l: f32,
    #[serde(default)]
    pub x3l: f32,
    #[serde(default)]
    pub x4l: f32,
    #[serde(default)]
    pub x5l: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiRadiusData {
    #[serde(default)]
    pub xs: f32,
    #[serde(default)]
    pub sm: f32,
    #[serde(default)]
    pub base: f32,
    #[serde(default)]
    pub lg: f32,
    #[serde(default)]
    pub xl: f32,
    #[serde(default)]
    pub x2l: f32,
    #[serde(default)]
    pub x3l: f32,
    #[serde(default)]
    pub x4l: f32,
    #[serde(default)]
    pub full: f32,
}

impl Default for UiLayoutData {
    fn default() -> Self {
        UiLayoutData {
            padding: UiSpacingData {
                xs: 1.0,
                sm: 2.0,
                base: 3.0,
                lg: 4.0,
                xl: 5.0,
                x2l: 6.0,
                x3l: 7.0,
                x4l: 8.0,
                x5l: 9.0,
            },
            margin: UiSpacingData {
                xs: 1.0,
                sm: 2.0,
                base: 3.0,
                lg: 4.0,
                xl: 5.0,
                x2l: 6.0,
                x3l: 7.0,
                x4l: 8.0,
                x5l: 9.0,
            },
            gap: UiSpacingData {
                xs: 1.0,
                sm: 2.0,
                base: 3.0,
                lg: 4.0,
                xl: 5.0,
                x2l: 6.0,
                x3l: 7.0,
                x4l: 8.0,
                x5l: 9.0,
            },
            radius: UiRadiusData {
                xs: 0.125,
                sm: 0.25,
                base: 0.375,
                lg: 0.5,
                xl: 0.75,
                x2l: 1.0,
                x3l: 1.5,
                x4l: 2.0,
                full: f32::MAX,
            },
            border: UiSpacingData {
                xs: 1.0,
                sm: 2.0,
                base: 3.0,
                lg: 5.0,
                xl: 7.0,
                x2l: 9.0,
                x3l: 12.0,
                x4l: 15.0,
                x5l: 19.0,
            },
        }
    }
}
