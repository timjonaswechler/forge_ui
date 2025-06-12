use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for the context menu container.
#[derive(Bundle, Clone, Debug)]
pub struct ContextMenuStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ContextMenuStyle {
    pub fn new(theme: &UiTheme) -> Self {
        ContextMenuStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

/// Style bundle for menu items.
#[derive(Bundle, Clone, Debug)]
pub struct ContextMenuItemStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl ContextMenuItemStyle {
    pub fn new(theme: &UiTheme) -> Self {
        ContextMenuItemStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.xs)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}
