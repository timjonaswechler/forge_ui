// src/components/radio/style.rs

use crate::components::radio::enums::{RadioSize, RadioVariant};
use crate::components::text::TextStyle;
use crate::components::UiTheme;
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
            background: theme.primary_bg,
            border: theme.primary_border,
            indicator: theme.primary_focus,
            disabled: theme.disabled,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
        RadioVariant::Secondary => RadioStyleDef {
            background: theme.secondary_bg,
            border: theme.secondary_border,
            indicator: theme.secondary_focus,
            disabled: theme.disabled,
            size: match size {
                RadioSize::Small => 12.0,
                RadioSize::Medium => 16.0,
                RadioSize::Large => 20.0,
            },
        },
    }
}
