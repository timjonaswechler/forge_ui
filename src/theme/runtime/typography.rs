// src/theme/runtime/typography.rs

use crate::theme::data::UiTypographyData;
use bevy::{asset::AssetServer, prelude::*};

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

pub fn build(
    asset_server: &AssetServer,
    data: &UiTypographyData,
    rem: f32,
    ui_scaling: f32,
) -> UiTypography {
    let load_font = |path: &str| {
        if path.is_empty() {
            Handle::default()
        } else {
            asset_server.load(path)
        }
    };
    let size = |v: f32| v * rem * ui_scaling;

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
            default: load_font(&data.font_family.default),
            sans: FontVariants {
                light: load_font(&data.font_family.sans.light),
                light_italic: load_font(&data.font_family.sans.light_italic),
                regular: load_font(&data.font_family.sans.regular),
                regular_italic: load_font(&data.font_family.sans.regular_italic),
                medium: load_font(&data.font_family.sans.medium),
                medium_italic: load_font(&data.font_family.sans.medium_italic),
                bold: load_font(&data.font_family.sans.bold),
                bold_italic: load_font(&data.font_family.sans.bold_italic),
            },
            serif: FontVariants {
                light: load_font(&data.font_family.serif.light),
                light_italic: load_font(&data.font_family.serif.light_italic),
                regular: load_font(&data.font_family.serif.regular),
                regular_italic: load_font(&data.font_family.serif.regular_italic),
                medium: load_font(&data.font_family.serif.medium),
                medium_italic: load_font(&data.font_family.serif.medium_italic),
                bold: load_font(&data.font_family.serif.bold),
                bold_italic: load_font(&data.font_family.serif.bold_italic),
            },
            mono: FontVariants {
                light: load_font(&data.font_family.mono.light),
                light_italic: load_font(&data.font_family.mono.light_italic),
                regular: load_font(&data.font_family.mono.regular),
                regular_italic: load_font(&data.font_family.mono.regular_italic),
                medium: load_font(&data.font_family.mono.medium),
                medium_italic: load_font(&data.font_family.mono.medium_italic),
                bold: load_font(&data.font_family.mono.bold),
                bold_italic: load_font(&data.font_family.mono.bold_italic),
            },
        },
    }
}
