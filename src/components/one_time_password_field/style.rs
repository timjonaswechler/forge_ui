use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct OtpFieldStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl OtpFieldStyle {
    pub fn new(theme: &UiTheme) -> Self {
        OtpFieldStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(theme.layout.padding.xs),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct OtpInputStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl OtpInputStyle {
    pub fn new(theme: &UiTheme) -> Self {
        OtpInputStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}
