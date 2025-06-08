use crate::theme::UiTheme;

use bevy::prelude::*; // Optional, falls Theme Styling beeinflusst

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UiContainer;

impl UiContainer {
    pub fn spawn(commands: &mut Commands, theme: &UiTheme) -> Entity {
        commands
            .spawn((
                Name::new("UiContainer"), // Optional, falls Theme Styling beeinflusst
                UiContainer,
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
            ))
            .id()
    }
    pub fn spawn_as_child(parent: &mut ChildSpawnerCommands, theme: &UiTheme) -> Entity {
        parent
            .spawn((
                UiContainer,
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
            ))
            .id()
    }
}
