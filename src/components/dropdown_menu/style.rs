use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style for the dropdown menu content container.
#[derive(Bundle, Clone, Debug)]
pub struct DropdownMenuStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl DropdownMenuStyle {
    pub fn new(theme: &UiTheme) -> Self {
        DropdownMenuStyle {
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

/// Style for dropdown menu items.
#[derive(Bundle, Clone, Debug)]
pub struct DropdownMenuItemStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl DropdownMenuItemStyle {
    pub fn new(theme: &UiTheme) -> Self {
        DropdownMenuItemStyle {
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
