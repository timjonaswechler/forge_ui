use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a card container.
#[derive(Bundle, Clone, Debug)]
pub struct CardStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl CardStyle {
    pub fn new(theme: &UiTheme) -> Self {
        CardStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step01),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
