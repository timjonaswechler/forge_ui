// src/components/badge/utils.rs
use super::BadgeVariant;
use crate::theme::UiTheme;

use bevy::prelude::*;

/// Liefert die Farbwerte für Badge-Hintergrund, -Text und -Rand basierend auf der Variante.
///
/// # Parameter
///
/// - `variant`: Ausgewählte Variante des Badges.
/// - `theme`: Eure UI-Theme-Ressource mit vordefinierten Farbwerten.
///
/// # Rückgabe
///
/// Ein Tuple `(background_color, text_color, border_color)`.
///
/// ## Beispiele
///
/// ```rust
/// # use forge_ui::components::badge::{get_badge_colors, BadgeVariant};
/// # use forge_ui::theme::UiTheme;
/// let theme = UiTheme::default();
/// let (bg, txt, border) = get_badge_colors(&BadgeVariant::Destructive, &theme);
/// assert_eq!(border, theme.color.tomato.step06);
/// ```
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
