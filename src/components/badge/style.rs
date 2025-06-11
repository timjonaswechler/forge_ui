use bevy::prelude::*;

use super::enums::BadgeVariant;
use super::utils::get_badge_colors;
use crate::theme::UiTheme;

/// Bundles all style related components for a badge.
#[derive(Bundle, Clone, Debug)]
pub struct BadgeStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub text_style: TextFont,
    pub text_color: TextColor,
}

impl BadgeStyle {
    /// Creates the style bundle for a badge based on the theme and variant.
    pub fn new(variant: BadgeVariant, theme: &UiTheme, font: &Handle<Font>) -> Self {
        let (bg, text, border) = get_badge_colors(&variant, theme);
        BadgeStyle {
            node: Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Auto,
                height: Val::Auto,
                min_height: Val::Px(18.0),
                ..default()
            },
            background_color: BackgroundColor(bg),
            border_color: BorderColor(border),
            border_radius: BorderRadius::all(Val::Percent(50.0)),
            text_style: TextFont {
                font: font.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            text_color: TextColor(text),
        }
    }
}

