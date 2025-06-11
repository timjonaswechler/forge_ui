use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for a generic box container.
#[derive(Bundle, Clone, Debug)]
pub struct BoxStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl BoxStyle {
    pub fn new(theme: &UiTheme) -> Self {
        BoxStyle {
            node: Node { display: Display::Flex, ..default() },
            background_color: BackgroundColor(theme.color.gray.step01),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
