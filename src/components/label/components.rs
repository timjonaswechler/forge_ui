use bevy::prelude::*; // Für den Rückgabetyp von spawn

/// Marker-Komponente für UI-Labels.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct LabelMarker;

// --- Styling-Optionen (Optional, oft direkt vom Theme gesteuert) ---
// Ähnlich wie bei Card-Elementen, könnten wir Stile definieren,
// aber für einfache Labels reicht oft die Theme-Farbe.
// #[derive(Clone, Copy, Default, PartialEq, Eq)]
// pub enum LabelStyle {
//     #[default] Normal,
//     Muted,
//     Error, // Beispiel für semantische Farben
// }
