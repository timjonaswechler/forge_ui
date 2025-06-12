use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct PopoverContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl PopoverContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        PopoverContentStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

