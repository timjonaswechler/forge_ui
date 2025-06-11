use bevy::prelude::*;

use crate::theme::UiTheme;

/// Style bundle for inline code text.
#[derive(Bundle, Clone, Debug)]
pub struct CodeStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl CodeStyle {
    pub fn new(theme: &UiTheme) -> Self {
        CodeStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.xs)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.slate.step03),
            border_color: BorderColor(theme.color.slate.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.xs)),
        }
    }
}
