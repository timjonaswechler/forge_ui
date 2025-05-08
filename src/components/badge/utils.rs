// src/components/badge/utils.rs
use super::BadgeVariant;
use crate::theme::UiTheme;

use bevy::prelude::*;

pub fn get_badge_colors(variant: &BadgeVariant, theme: &UiTheme) -> (Color, Color, Color) {
    // Gibt zurück: (Hintergrund, Text, Rand)
    match variant {
        BadgeVariant::Default => (
            theme.color.gray.step01, // Hintergrund: Primär
            theme.color.gray.step11, // Text: Passend zu Primär
            theme.color.gray.step06, // Rand: Gleich wie Hintergrund (wirkt transparent)
        ),
        BadgeVariant::Secondary => (
            theme.color.gray.step02,
            theme.color.gray.step12,
            theme.color.gray.step07, // Rand: Gleich wie Hintergrund (wirkt transparent)
        ),
        BadgeVariant::Destructive => (
            theme.color.tomato.step01,
            theme.color.tomato.step11,
            theme.color.tomato.step06,
        ),
        BadgeVariant::Outline => (
            Color::NONE,             // Kein Hintergrund
            theme.color.gray.step11, // Text: Passend zu Primär
            theme.color.gray.step06, // Sichtbarer Rand
        ),
    }
}
