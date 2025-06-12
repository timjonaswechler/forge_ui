use bevy::{prelude::*, ui::FocusPolicy};

/// Marker component for the dialog overlay container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseDialogOverlayMarker;

/// Marker component for the scroll container inside the dialog.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseDialogScrollMarker;

/// Marker component for the scroll padding element.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseDialogScrollPaddingMarker;

/// Marker component for the actual dialog content container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct BaseDialogContentMarker;

/// Style for the dialog overlay covering the screen.
#[derive(Bundle, Clone, Debug)]
pub struct BaseDialogOverlayStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseDialogOverlayStyle {
    fn default() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.4)),
            focus_policy: FocusPolicy::Block,
        }
    }
}

/// Style for the scroll container holding padding and content.
#[derive(Bundle, Clone, Debug)]
pub struct BaseDialogScrollStyle {
    pub node: Node,
}

impl Default for BaseDialogScrollStyle {
    fn default() -> Self {
        Self {
            node: Node {
                overflow: Overflow::clip(),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }
}

/// Style for the inner padding element used to center content.
#[derive(Bundle, Clone, Debug)]
pub struct BaseDialogScrollPaddingStyle {
    pub node: Node,
}

impl Default for BaseDialogScrollPaddingStyle {
    fn default() -> Self {
        Self {
            node: Node {
                padding: UiRect::all(Val::Px(16.0)),
                width: Val::Percent(100.0),
                height: Val::Auto,
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }
}

/// Style for the dialog content container.
#[derive(Bundle, Clone, Debug)]
pub struct BaseDialogContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub focus_policy: FocusPolicy,
}

impl Default for BaseDialogContentStyle {
    fn default() -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                min_width: Val::Px(240.0),
                max_width: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            focus_policy: FocusPolicy::Block,
        }
    }
}
