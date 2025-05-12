use crate::theme::UiTheme;

use bevy::prelude::*; // Optional, falls Theme Styling beeinflusst

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UiRoot;

impl UiRoot {
    pub fn spawn(commands: &mut Commands, theme: &UiTheme) -> Entity {
        commands
            .spawn((
                UiRoot,
                Node {
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
            ))
            .id()
    }
}
