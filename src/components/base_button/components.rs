use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for a base button container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseButtonMarker;

/// Basic style bundle for a button-like container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseButtonStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseButtonStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            interaction: Interaction::None,
            focus_policy: FocusPolicy::Block,
        }
    }
}
