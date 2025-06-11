use bevy::prelude::*;

use crate::theme::UiTheme;
use super::CalloutVariant;

/// Style bundle for a callout box.
#[derive(Bundle, Clone, Debug)]
pub struct CalloutStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl CalloutStyle {
    pub fn new(variant: CalloutVariant, theme: &UiTheme, _font: &Handle<Font>) -> Self {
        let palette = match variant {
            CalloutVariant::Info => &theme.color.blue,
            CalloutVariant::Success => &theme.color.green,
            CalloutVariant::Warning => &theme.color.amber,
            CalloutVariant::Error => &theme.color.tomato,
        };
        CalloutStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: Val::Px(theme.layout.gap.base),
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                ..default()
            },
            background_color: BackgroundColor(palette.step03),
            border_color: BorderColor(palette.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
