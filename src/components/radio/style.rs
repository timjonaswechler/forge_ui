// src/components/radio/style.rs

use crate::components::radio::enums::{RadioSize, RadioVariant};
use crate::UiTheme;
use bevy::prelude::*;

/// Defines styling for each variant/state
pub struct RadioStyleDef {
    pub background: Color,
    pub border: Color,
    pub indicator: Color,
    pub disabled: Color,
    pub size: f32,
}

/// Lookup style from theme
pub fn get_radio_style_def(
    variant: RadioVariant,
    size: RadioSize,
    theme: &UiTheme,
) -> RadioStyleDef {
    // implement mapping from theme and variant/size
    match variant {
        RadioVariant::Primary => RadioStyleDef {
            background: theme.accent_color.step09,
            border: theme.accent_color.step07,
            indicator: theme.accent_color.step12,
            disabled: theme.color.black.step05,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
        RadioVariant::Secondary => RadioStyleDef {
            background: theme.gray_accent_color.step09,
            border: theme.gray_accent_color.step07,
            indicator: theme.gray_accent_color.step12,
            disabled: theme.color.black.step05,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
    }
}
