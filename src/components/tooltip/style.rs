use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for tooltip content.
#[derive(Bundle, Clone, Debug)]
pub struct TooltipContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl TooltipContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        TooltipContentStyle {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}
