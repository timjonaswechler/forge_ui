use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct PasswordToggleFieldStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl PasswordToggleFieldStyle {
    pub fn new(theme: &UiTheme) -> Self {
        PasswordToggleFieldStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(theme.layout.padding.xs),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct PasswordInputStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl PasswordInputStyle {
    pub fn new(theme: &UiTheme) -> Self {
        PasswordInputStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(theme.layout.padding.xs)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct PasswordToggleStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl PasswordToggleStyle {
    pub fn new(theme: &UiTheme) -> Self {
        PasswordToggleStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(theme.layout.padding.xs)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step04),
        }
    }
}

