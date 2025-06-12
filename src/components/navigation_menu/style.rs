use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct NavigationMenuStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl NavigationMenuStyle {
    pub fn new(theme: &UiTheme) -> Self {
        NavigationMenuStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct NavigationMenuItemStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl NavigationMenuItemStyle {
    pub fn new(theme: &UiTheme) -> Self {
        NavigationMenuItemStyle {
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
pub struct NavigationMenuContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl NavigationMenuContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        NavigationMenuContentStyle {
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
pub struct NavigationMenuLinkStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl NavigationMenuLinkStyle {
    pub fn new(theme: &UiTheme) -> Self {
        NavigationMenuLinkStyle {
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
