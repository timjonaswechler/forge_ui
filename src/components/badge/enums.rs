// src/components/badge/enums.rs

// src/components/badge/enums.rs
/// Definiert die verschiedenen visuellen Varianten eines Badges.
///
/// - `Default`: Primärer Hintergrund mit passender Textfarbe.
/// - `Secondary`: Sekundärer (dezenterer) Hintergrund.
/// - `Destructive`: Warnfarbe (z. B. für Löschen/Fehler).
/// - `Outline`: Kein Hintergrund, nur Rand und Text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BadgeVariant {
    #[default]
    /// Primäre Variante (Standard).
    Default,
    /// Sekundäre Variante.
    Secondary,
    /// Destruktive/warnende Variante.
    Destructive,
    /// Nur Umrandung, transparenter Hintergrund.
    Outline,
}
