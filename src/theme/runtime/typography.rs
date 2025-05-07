// src/theme/runtime/typography.rs
use crate::assets::FontAssets;
use crate::plugin::UiConfig;
use crate::theme::data::UiTypographyData;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct UiTypography {
    pub font_size: UiFontSize,
    pub font_family: UiFontFamilies,
}

#[derive(Debug, Clone)]
pub struct UiFontSize {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub x2l: f32,
    pub x3l: f32,
    pub x4l: f32,
    pub x5l: f32,
    pub x6l: f32,
    pub x7l: f32,
    pub x8l: f32,
    pub x9l: f32,
}

#[derive(Debug, Clone)]
pub struct UiFontFamilies {
    pub default: Handle<Font>,
    pub sans: FontVariants,
    pub serif: FontVariants,
    pub mono: FontVariants,
}

#[derive(Debug, Clone)]
pub struct FontVariants {
    pub light: Handle<Font>,
    pub light_italic: Handle<Font>,
    pub regular: Handle<Font>,
    pub regular_italic: Handle<Font>,
    pub medium: Handle<Font>,
    pub medium_italic: Handle<Font>,
    pub bold: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

pub fn build(assets: &FontAssets, data: &UiTypographyData, config: &UiConfig) -> UiTypography {
    let size = |v: f32| v * config.font_size_base * config.scaling;

    UiTypography {
        font_size: UiFontSize {
            xs: size(data.font_size.xs),
            sm: size(data.font_size.sm),
            base: size(data.font_size.base),
            lg: size(data.font_size.lg),
            xl: size(data.font_size.xl),
            x2l: size(data.font_size.x2l),
            x3l: size(data.font_size.x3l),
            x4l: size(data.font_size.x4l),
            x5l: size(data.font_size.x5l),
            x6l: size(data.font_size.x6l),
            x7l: size(data.font_size.x7l),
            x8l: size(data.font_size.x8l),
            x9l: size(data.font_size.x9l),
        },
        font_family: UiFontFamilies {
            default: assets.default.clone(),
            sans: FontVariants {
                light: assets.sans_light.clone(),
                light_italic: assets.sans_light_italic.clone(),
                regular: assets.sans_regular.clone(),
                regular_italic: assets.sans_regular_italic.clone(),
                medium: assets.sans_medium.clone(),
                medium_italic: assets.sans_medium_italic.clone(),
                bold: assets.sans_bold.clone(),
                bold_italic: assets.sans_bold_italic.clone(),
            },
            serif: FontVariants {
                light: assets.serif_light.clone(),
                light_italic: assets.serif_light_italic.clone(),
                regular: assets.serif_regular.clone(),
                regular_italic: assets.serif_regular_italic.clone(),
                medium: assets.serif_medium.clone(),
                medium_italic: assets.serif_medium_italic.clone(),
                bold: assets.serif_bold.clone(),
                bold_italic: assets.serif_bold_italic.clone(),
            },
            mono: FontVariants {
                light: assets.mono_light.clone(),
                light_italic: assets.mono_light_italic.clone(),
                regular: assets.mono_regular.clone(),
                regular_italic: assets.mono_regular_italic.clone(),
                medium: assets.mono_medium.clone(),
                medium_italic: assets.mono_medium_italic.clone(),
                bold: assets.mono_bold.clone(),
                bold_italic: assets.mono_bold_italic.clone(),
            },
        },
    }
}
