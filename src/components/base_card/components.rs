use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for a base card container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseCardMarker;

/// Basic style bundle for a card-like container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseCardStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseCardStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            focus_policy: FocusPolicy::Block,
        }
    }
}

