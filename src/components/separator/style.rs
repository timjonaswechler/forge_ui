use bevy::prelude::*;

use super::SeparatorOrientation;
use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct SeparatorStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl SeparatorStyle {
    pub fn new(theme: &UiTheme, orientation: SeparatorOrientation, length: Val) -> Self {
        let (width, height) = match orientation {
            SeparatorOrientation::Horizontal => (length, Val::Px(1.0)),
            SeparatorOrientation::Vertical => (Val::Px(1.0), length),
        };
        Self {
            node: Node {
                display: Display::Flex,
                width,
                height,
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step06),
        }
    }
}
