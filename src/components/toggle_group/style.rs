use super::enums::{ToggleGroupSize, ToggleGroupVariant};
use bevy::prelude::*;

/// Style definition for ToggleGroups
#[derive(Debug, Clone, Copy)]
pub struct ToggleGroupStyleDef {
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub border_radius: f32,
    pub spacing: f32,
    pub padding: UiRect,
}

/// Derives style from UI theme
pub fn get_toggle_group_style_def(
    theme: &crate::theme::UiTheme,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
) -> ToggleGroupStyleDef {
    // Background and border colors based on variant
    let (bg_color, border_color) = match variant {
        ToggleGroupVariant::Primary => (theme.color.green.step05, theme.color.green.step08),
        ToggleGroupVariant::Secondary => (theme.gray_accent.step05, theme.gray_accent.step08),
        ToggleGroupVariant::Default => (theme.accent.step05, theme.accent.step08),
    };

    // Sizing based on selected size
    let (border_radius, spacing, padding) = match size {
        ToggleGroupSize::Small => (
            theme.layout.radius.sm,
            theme.layout.gap.xs,
            UiRect::all(Val::Px(theme.layout.padding.xs)),
        ),
        ToggleGroupSize::Medium => (
            theme.layout.radius.base,
            theme.layout.gap.sm,
            UiRect::all(Val::Px(theme.layout.padding.sm)),
        ),
        ToggleGroupSize::Large => (
            theme.layout.radius.lg,
            theme.layout.gap.base,
            UiRect::all(Val::Px(theme.layout.padding.base)),
        ),
    };

    ToggleGroupStyleDef {
        background_color: bg_color,
        border_color,
        border_width: 1.0,
        border_radius,
        spacing,
        padding,
    }
}
