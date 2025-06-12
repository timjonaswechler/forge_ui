use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for a base menu content container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseMenuContentMarker;

/// Basic style bundle for a menu content container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseMenuContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseMenuContentStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            focus_policy: FocusPolicy::Block,
        }
    }
}
