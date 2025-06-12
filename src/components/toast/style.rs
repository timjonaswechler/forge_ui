use bevy::prelude::*;

use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct ToastStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl ToastStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: Val::Px(theme.layout.gap.sm),
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                margin: UiRect::all(Val::Px(theme.layout.gap.sm)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step03),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}
