use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a collection container.
#[derive(Bundle, Clone, Debug)]
pub struct CollectionStyle {
    pub node: Node,
}

impl CollectionStyle {
    pub fn new(_theme: &UiTheme) -> Self {
        CollectionStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        }
    }
}
