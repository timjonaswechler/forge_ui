use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for an aspect ratio container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct AspectRatioMarker;

/// Basic style bundle for an aspect ratio container.
#[derive(Bundle, Clone, Debug)]
pub struct AspectRatioStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
}

impl Default for AspectRatioStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                overflow: Overflow::clip(),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            focus_policy: FocusPolicy::Pass,
        }
    }
}
