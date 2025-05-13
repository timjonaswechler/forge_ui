use bevy::prelude::*;

use super::*;

/// Zustandsspezifische Farben
#[derive(Debug, Clone, Copy)]
pub struct ToggleStateColors {
    pub off: Color,
    pub on: Color,
    pub disabled: Color,
}

/// Vollständige Stildefinition (siehe template.md §4)
#[derive(Debug, Clone, Copy)]
pub struct ToggleStyleDef {
    pub bg_colors: ToggleStateColors,
    pub icon_colors: ToggleStateColors,
    pub size_px: f32,
    pub border_radius: f32,
}

/// Ableitung aus `UiTheme` (Platzhalter‑Implementation – passe Mapping an dein Theme an)
pub fn get_toggle_style_def(
    theme: &crate::theme::UiTheme,
    variant: ToggleVariant,
    size: ToggleSize,
) -> ToggleStyleDef {
    // ––––– Variant‑Mapping
    let (off, on, disabled) = match variant {
        ToggleVariant::Primary => (
            theme.color.green.step10,
            theme.color.green.step12,
            theme.color.green.step07,
        ),
        ToggleVariant::Secondary => (
            theme.gray_accent.step10,
            theme.gray_accent.step12,
            theme.gray_accent.step07,
        ),
        ToggleVariant::Default => (
            theme.accent.step10,
            theme.accent.step12,
            theme.accent.step07,
        ),
    };

    // ––––– Size‑Mapping
    let (size_px, radius) = match size {
        ToggleSize::Small => (theme.layout.padding.sm, theme.layout.radius.sm),
        ToggleSize::Medium => (theme.layout.padding.base, theme.layout.radius.base),
        ToggleSize::Large => (theme.layout.padding.lg, theme.layout.radius.lg),
    };

    ToggleStyleDef {
        bg_colors: ToggleStateColors { off, on, disabled },
        icon_colors: ToggleStateColors {
            off: theme.color.gray.step09,
            on: theme.color.gray.step12,
            disabled: theme.color.gray.step07,
        },
        size_px,
        border_radius: radius,
    }
}
