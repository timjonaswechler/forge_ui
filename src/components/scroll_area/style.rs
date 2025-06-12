use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct ScrollAreaStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl ScrollAreaStyle {
    pub fn new(theme: &UiTheme) -> Self {
        ScrollAreaStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                width: Val::Px(200.0),
                height: Val::Px(120.0),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct ScrollAreaViewportStyle {
    pub node: Node,
}

impl ScrollAreaViewportStyle {
    pub fn new(_theme: &UiTheme) -> Self {
        ScrollAreaViewportStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        }
    }
}
