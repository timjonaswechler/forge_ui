use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a blockquote element.
#[derive(Bundle, Clone, Debug)]
pub struct BlockquoteStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl BlockquoteStyle {
    pub fn new(theme: &UiTheme) -> Self {
        BlockquoteStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                border: UiRect::left(Val::Px(theme.layout.border.base)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step01),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
