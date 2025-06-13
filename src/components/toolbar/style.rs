use bevy::prelude::*;

use crate::theme::UiTheme;
use super::ToolbarOrientation;

#[derive(Bundle, Clone, Debug)]
pub struct ToolbarStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ToolbarStyle {
    pub fn new(orientation: ToolbarOrientation, theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: match orientation {
                    ToolbarOrientation::Horizontal => FlexDirection::Row,
                    ToolbarOrientation::Vertical => FlexDirection::Column,
                },
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                column_gap: Val::Px(theme.layout.gap.sm),
                row_gap: Val::Px(theme.layout.gap.sm),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step02),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}
