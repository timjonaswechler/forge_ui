use bevy::prelude::*;

use crate::theme::UiTheme;
use super::ToastVariant;

/// Style bundle for a toast notification.
#[derive(Bundle, Clone, Debug)]
pub struct ToastStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl ToastStyle {
    pub fn new(variant: ToastVariant, theme: &UiTheme, _font: &Handle<Font>) -> Self {
        let palette = match variant {
            ToastVariant::Info => &theme.color.blue,
            ToastVariant::Success => &theme.color.green,
            ToastVariant::Warning => &theme.color.amber,
            ToastVariant::Error => &theme.color.tomato,
        };
        ToastStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                margin: UiRect::vertical(Val::Px(theme.layout.gap.base)),
                ..default()
            },
            background_color: BackgroundColor(palette.step03),
            border_color: BorderColor(palette.step06),
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
        }
    }
}
