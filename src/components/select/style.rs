use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct SelectRootStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl SelectRootStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step01),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SelectTriggerStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl SelectTriggerStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_color: BorderColor(theme.color.gray.step06),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SelectContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl SelectContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct SelectOptionStyle {
    pub node: Node,
}

impl SelectOptionStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                ..default()
            },
        }
    }
}
