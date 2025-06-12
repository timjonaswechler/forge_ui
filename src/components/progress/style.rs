use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct ProgressTrackStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ProgressTrackStyle {
    pub fn new(theme: &UiTheme) -> Self {
        ProgressTrackStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                width: Val::Px(120.0),
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
pub struct ProgressIndicatorStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl ProgressIndicatorStyle {
    pub fn new(theme: &UiTheme, ratio: f32) -> Self {
        ProgressIndicatorStyle {
            node: Node {
                width: Val::Percent((ratio.clamp(0.0, 1.0)) * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(theme.accent.step09),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}
