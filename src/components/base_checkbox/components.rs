use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for a base checkbox container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseCheckboxMarker;

/// Basic style bundle for a checkbox-like container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseCheckboxStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseCheckboxStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            focus_policy: FocusPolicy::Block,
        }
    }
}

