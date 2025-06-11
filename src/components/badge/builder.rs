// src/components/badge/builder.rs
use bevy::prelude::*;

use super::{style::BadgeStyle, BadgeMarker, BadgeVariant};
use crate::theme::UiTheme;

/// # BadgeBuilder
///
/// Ein flexibler Builder zum Erstellen und Konfigurieren von UI-Badges (kleine, abgerundete Labels).
///
/// Badges unterstützen:
/// - Verschiedene visuelle Varianten (`BadgeVariant`):
///   - `Default` (Primär)
///   - `Secondary` (Sekundär)
///   - `Destructive` (Warnung)
///   - `Outline` (nur Rand)
/// - Optional Icons vor (`leading_icon`) und nach (`trailing_icon`) dem Text
/// - Anpassbaren Text
///
/// ## Beispiele
///
/// ```rust
/// use bevy::prelude::*;
/// use forge_ui::components::badge::{BadgeBuilder, BadgeVariant};
/// use forge_ui::theme::UiTheme;
///
/// fn setup_ui(
///     mut commands: Commands,
///     theme: Res<UiTheme>,
///     asset_server: Res<AssetServer>,
///     icons: Res<IconAssets>,
/// ) {
///     commands.spawn(NodeBundle::default()).with_children(|parent| {
///         let font: Handle<Font> = asset_server.load("fonts/Roboto-Regular.ttf");
///
///         // Standard-Badge
///         BadgeBuilder::new("Neu")
///             .spawn(parent, &theme, &font);
///
///         // Sekundäre Variante mit Icons
///         let star_icon = icons
///             .0
///             .get("star")
///             .expect("missing 'star' icon")
///             .clone();
///         BadgeBuilder::new("Favorit")
///             .variant(BadgeVariant::Secondary)
///             .leading_icon(star_icon.clone())
///             .trailing_icon(star_icon)
///             .spawn(parent, &theme, &font);
///     });
/// }
/// ```
///
/// ## Methoden
///
/// - `BadgeBuilder::new(text: impl Into<String>)` – Erstellt einen Badge mit dem gegebenen Text und `Default`-Variante.
/// - `.variant(variant: BadgeVariant)` – Ändert die visuelle Variante des Badges.
/// - `.leading_icon(handle: Handle<Image>)` – Fügt ein Icon vor dem Text hinzu.
/// - `.trailing_icon(handle: Handle<Image>)` – Fügt ein Icon nach dem Text hinzu.
/// - `.spawn(parent, &UiTheme, &Handle<Font>)` – Spawnt das Badge im angegebenen `ChildSpawnerCommands`.
pub struct BadgeBuilder {
    /// Sichtbarer Text, der auf dem Badge erscheint.
    text: String,

    /// Gewählte visuelle Variante (Default = Primär).
    variant: BadgeVariant,

    /// Optionales Icon **links** vom Text.
    leading_icon: Option<Handle<Image>>,

    /// Optionales Icon **rechts** vom Text.
    trailing_icon: Option<Handle<Image>>,
}

impl BadgeBuilder {
    /// Erstellt einen neuen Badge-Builder mit Standard-Variante.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::default(), // Startet mit Default-Variante
            leading_icon: None,
            trailing_icon: None,
        }
    }

    /// Setzt die visuelle Variante des Badges.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Fügt ein führendes Icon hinzu.
    pub fn leading_icon(mut self, handle: Handle<Image>) -> Self {
        self.leading_icon = Some(handle);
        self
    }

    /// Fügt ein folgendes Icon hinzu.
    pub fn trailing_icon(mut self, handle: Handle<Image>) -> Self {
        self.trailing_icon = Some(handle);
        self
    }

    /// Spawnt das Badge als Kind des UI-Parents.
    #[must_use]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) -> Entity {
        // Build style bundle using theme and variant
        let style = BadgeStyle::new(self.variant, theme, font_handle);

        // Badge is basically a Node with text child
        parent
            .spawn((BadgeMarker, style))
            .with_children(|badge_node| {
                // Optional: Leading Icon hier spawnen

                // Text-Kind spawnen
                badge_node.spawn((
                    Text::new(self.text), // Text aus Builder
                    // font and color already handled by style
                ));

                // Optional: Trailing Icon hier spawnen
            })
            .id()
    }
}
