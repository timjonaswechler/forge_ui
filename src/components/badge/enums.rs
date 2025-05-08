// src/components/badge/enums.rs

// Definiert die verschiedenen visuellen Stile des Badges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeVariant {
    #[default]
    Default, // Primärfarbe
    Secondary,
    Destructive,
    Outline,
}
