use bevy::prelude::*;

use super::TabsOrientation;
use crate::theme::UiTheme;

#[derive(Bundle, Clone, Debug)]
pub struct TabsRootStyle {
    pub node: Node,
}

impl TabsRootStyle {
    pub fn new(theme: &UiTheme, orientation: TabsOrientation) -> Self {
        let flex_direction = match orientation {
            TabsOrientation::Horizontal => FlexDirection::Row,
            TabsOrientation::Vertical => FlexDirection::Column,
        };
        Self {
            node: Node {
                display: Display::Flex,
                flex_direction,
                ..default()
            },
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct TabTriggerStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
}

impl TabTriggerStyle {
    pub fn new(theme: &UiTheme, active: bool) -> Self {
        Self {
            node: Node {
                padding: UiRect::all(Val::Px(4.0)),
                margin: UiRect::horizontal(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(if active {
                theme.accent.step09
            } else {
                theme.color.gray.step03
            }),
            border_color: BorderColor(theme.color.gray.step06),
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct TabContentStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
}

impl TabContentStyle {
    pub fn new(theme: &UiTheme) -> Self {
        Self {
            node: Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: BackgroundColor(theme.color.gray.step01),
        }
    }
}
