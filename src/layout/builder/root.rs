use crate::theme::UiTheme;

use bevy::prelude::*; // Optional, falls Theme Styling beeinflusst

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UiRoot;

impl UiRoot {
    /// Returns the bundle of components used for the root UI node. This mirrors
    /// the approach in `widget.rs` where a helper function returns `impl Bundle`
    /// instead of spawning the entity directly.
    pub fn bundle(theme: &UiTheme) -> impl Bundle {
        (
            UiRoot,
            // Style und Hintergrundfarbe bleiben wie gehabt
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                ..default()
            },
            BackgroundColor(theme.color.gray.step01),
            // ─── Hier kommen die fehlenden UI-Bundles ───
            Visibility::Visible,
        )
    }
}
