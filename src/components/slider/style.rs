use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct SliderTrackStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl SliderTrackStyle {
    pub fn new(theme: &UiTheme, width: Val) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                width,
                height: Val::Px(12.0),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SliderRangeStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl SliderRangeStyle {
    pub fn new(theme: &UiTheme, value: f32) -> Self {
        Self {
            node: Node {
                width: Val::Percent(value.clamp(0.0, 1.0) * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(theme.accent.step09),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SliderThumbStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl SliderThumbStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                width: Val::Px(12.0),
                height: Val::Px(12.0),
                ..default()
            },
            background_color: BackgroundColor(theme.color.white.step12),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

