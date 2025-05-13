// src/components/radio/style.rs
use bevy::prelude::*;

use super::*;
use crate::theme::UiTheme;

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
            background: theme.accent.step09,
            border: theme.accent.step07,
            indicator: theme.accent.step12,
            disabled: theme.color.black.step05,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
        RadioVariant::Secondary => RadioStyleDef {
            background: theme.gray_accent.step09,
            border: theme.gray_accent.step07,
            indicator: theme.gray_accent.step12,
            disabled: theme.color.black.step05,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
    }
}
