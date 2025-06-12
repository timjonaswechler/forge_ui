use bevy::prelude::*;

use super::ToolbarOrientation;
use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct ToolbarStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ToolbarStyle {
    pub fn new(theme: &UiTheme, orientation: ToolbarOrientation) -> Self {
        let flex_direction = match orientation {
            ToolbarOrientation::Horizontal => FlexDirection::Row,
            ToolbarOrientation::Vertical => FlexDirection::Column,
        };
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction,
                align_items: AlignItems::Center,
                column_gap: Val::Px(theme.layout.gap.sm),
                row_gap: Val::Px(theme.layout.gap.sm),
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_color: BorderColor(theme.color.gray.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}
