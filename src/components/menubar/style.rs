use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct MenubarStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl MenubarStyle {
    pub fn new(theme: &UiTheme) -> Self {
        MenubarStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct MenubarMenuStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl MenubarMenuStyle {
    pub fn new(theme: &UiTheme) -> Self {
        MenubarMenuStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct MenubarMenuContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl MenubarMenuContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        MenubarMenuContentStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct MenubarMenuItemStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl MenubarMenuItemStyle {
    pub fn new(theme: &UiTheme) -> Self {
        MenubarMenuItemStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.xs)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step04),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}
