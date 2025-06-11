use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a collapsible container.
#[derive(Bundle, Clone, Debug)]
pub struct CollapsibleStyle {
    pub node: Node,
    pub visibility: Visibility,
}

impl CollapsibleStyle {
    pub fn new(_theme: &UiTheme) -> Self {
        CollapsibleStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            visibility: Visibility::Inherited,
        }
    }
}
