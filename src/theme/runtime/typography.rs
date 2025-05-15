// src/theme/runtime/typography.rs
use crate::assets::FontAssets;
use crate::plugin::UiConfig;
use crate::theme::data::UiTypographyData;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct UiTypography {
    pub size: UiFontSize,
    pub family: UiFontFamilies,
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
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
    pub h4: f32,
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
        size: UiFontSize {
            xs: size(data.size.xs),
            sm: size(data.size.sm),
            base: size(data.size.base),
            lg: size(data.size.lg),
            xl: size(data.size.xl),
            x2l: size(data.size.x2l),
            x3l: size(data.size.x3l),
            x4l: size(data.size.x4l),
            x5l: size(data.size.x5l),
            x6l: size(data.size.x6l),
            x7l: size(data.size.x7l),
            x8l: size(data.size.x8l),
            x9l: size(data.size.x9l),
            h1: size(data.size.x4l),
            h2: size(data.size.x2l),
            h3: size(data.size.xl),
            h4: size(data.size.lg),
        },
        family: UiFontFamilies {
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
