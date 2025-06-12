use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for a base tab list container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseTabListMarker;

/// Basic style bundle for a tab list container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseTabListStyle {
    pub node: Node,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseTabListStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                overflow: Overflow::clip_x(),
                ..default()
            },
            focus_policy: FocusPolicy::Block,
        }
    }
}
