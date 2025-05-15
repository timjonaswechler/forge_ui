// crates/forge_ui/src/theme/typograph.rs

use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiTypographyData {
    #[serde(default)]
    pub size: UiFontSizeData,
    #[serde(default)]
    pub family: UiFontFamiliesData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiFontSizeData {
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
    #[serde(default)]
    pub x6l: f32,
    #[serde(default)]
    pub x7l: f32,
    #[serde(default)]
    pub x8l: f32,
    #[serde(default)]
    pub x9l: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiFontFamiliesData {
    #[serde(default)]
    pub default: String,
    #[serde(default)]
    pub sans: FontVariantsData,
    #[serde(default)]
    pub serif: FontVariantsData,
    #[serde(default)]
    pub mono: FontVariantsData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct FontVariantsData {
    #[serde(default)]
    pub light: String,
    #[serde(default)]
    pub light_italic: String,
    #[serde(default)]
    pub regular: String,
    #[serde(default)]
    pub regular_italic: String,
    #[serde(default)]
    pub medium: String,
    #[serde(default)]
    pub medium_italic: String,
    #[serde(default)]
    pub bold: String,
    #[serde(default)]
    pub bold_italic: String,
}

impl Default for UiTypographyData {
    fn default() -> Self {
        UiTypographyData {
            size: UiFontSizeData {
                xs: 0.75,
                sm: 0.875,
                base: 1.0,
                lg: 1.125,
                xl: 1.25,
                x2l: 1.5,
                x3l: 1.875,
                x4l: 2.25,
                x5l: 3.0,
                x6l: 3.75,
                x7l: 4.5,
                x8l: 6.0,
                x9l: 8.0,
            },
            family: UiFontFamiliesData {
                default: "fonts/Roboto-Regular.ttf".to_string(),
                sans: FontVariantsData {
                    light: "fonts/Roboto-Light.ttf".to_string(),
                    light_italic: "fonts/Roboto-LightItalic.ttf".to_string(),
                    regular: "fonts/Roboto-Regular.ttf".to_string(),
                    regular_italic: "fonts/Roboto-RegularItalic.ttf".to_string(),
                    medium: "fonts/Roboto-Medium.ttf".to_string(),
                    medium_italic: "fonts/Roboto-MediumItalic.ttf".to_string(),
                    bold: "fonts/Roboto-Bold.ttf".to_string(),
                    bold_italic: "fonts/Roboto-BoldItalic.ttf".to_string(),
                },
                serif: FontVariantsData {
                    light: "fonts/NotoSerif-Light.ttf".to_string(),
                    light_italic: "fonts/NotoSerif-LightItalic.ttf".to_string(),
                    regular: "fonts/NotoSerif-Regular.ttf".to_string(),
                    regular_italic: "fonts/NotoSerif-RegularItalic.ttf".to_string(),
                    medium: "fonts/NotoSerif-Medium.ttf".to_string(),
                    medium_italic: "fonts/NotoSerif-MediumItalic.ttf".to_string(),
                    bold: "fonts/NotoSerif-Bold.ttf".to_string(),
                    bold_italic: "fonts/NotoSerif-BoldItalic.ttf".to_string(),
                },
                mono: FontVariantsData {
                    light: "fonts/RobotoMono-Light.ttf".to_string(),
                    light_italic: "fonts/RobotoMono-LightItalic.ttf".to_string(),
                    regular: "fonts/RobotoMono-Regular.ttf".to_string(),
                    regular_italic: "fonts/RobotoMono-RegularItalic.ttf".to_string(),
                    medium: "fonts/RobotoMono-Medium.ttf".to_string(),
                    medium_italic: "fonts/RobotoMono-MediumItalic.ttf".to_string(),
                    bold: "fonts/RobotoMono-Bold.ttf".to_string(),
                    bold_italic: "fonts/RobotoMono-BoldItalic.ttf".to_string(),
                },
            },
        }
    }
}
