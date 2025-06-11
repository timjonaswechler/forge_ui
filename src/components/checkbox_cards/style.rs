use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crate::theme::UiTheme;

/// Style bundle for a checkbox card container.
#[derive(Bundle, Clone, Debug)]
pub struct CheckboxCardStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl CheckboxCardStyle {
    /// Creates the default style for checkbox cards using the [`UiTheme`].
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.slate.step01),
            border_color: BorderColor(theme.color.slate.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        }
    }
}

/// Spawns a translucent overlay to indicate disabled state.
pub fn spawn_disabled_overlay(cmd: &mut EntityCommands, theme: &UiTheme, radius: BorderRadius) {
    cmd.with_children(|parent| {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.color.black.step08),
            FocusPolicy::Block,
            Visibility::Visible,
            radius,
        ));
    });
}
