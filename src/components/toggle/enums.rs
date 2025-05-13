/// Visuelle Varianten (z.B. Farbe)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    /// Passende Standard‑Farbe aus dem Theme
    #[default]
    Default,
    /// Haupt‑Akzentfarbe
    Primary,
    /// Graustufen etc.
    Secondary,
}

/// Größen‑Abstufungen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    Small,
    #[default]
    Medium,
    Large,
}
