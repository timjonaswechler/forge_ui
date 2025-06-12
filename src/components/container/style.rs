use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a responsive container.
#[derive(Bundle, Clone, Debug)]
pub struct ContainerStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl ContainerStyle {
    pub fn new(theme: &UiTheme) -> Self {
        ContainerStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                padding: UiRect::axes(Val::Px(theme.layout.padding.base), Val::Px(0.0)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step01),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
