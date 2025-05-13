// src/components/badge/builder.rs
use bevy::prelude::*;

use super::utils::get_badge_colors;
use super::{BadgeMarker, BadgeVariant}; // Importiere den Marker
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
/// ) {
///     commands.spawn(NodeBundle::default()).with_children(|parent| {
///         let font: Handle<Font> = asset_server.load("fonts/Roboto-Regular.ttf");
///
///         // Standard-Badge
///         BadgeBuilder::new("Neu")
///             .spawn(parent, &theme, &font);
///
///         // Sekundäre Variante mit Icons
///         let star_icon: Handle<Image> = asset_server.load("icons/star.png");
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
#[allow(dead_code)]
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
        // Styling ableiten
        let (bg_color, text_color, border_color) = get_badge_colors(&self.variant, theme);

        // Badge ist im Grunde ein Node mit einem Text-Kind
        parent
            .spawn((
                BadgeMarker,
                Node {
                    // Flexbox, um Text (und ggf. Icons) anzuordnen
                    display: Display::Flex,
                    align_items: AlignItems::Center, // Vertikal zentrieren
                    justify_content: JustifyContent::Center, // Horizontal zentrieren

                    // Padding & Border für den Look
                    // Shadcn: px-2.5 py-0.5 -> ca. 10px horizontal, 2px vertikal
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(2.0),
                        bottom: Val::Px(2.0),
                    },
                    border: UiRect::all(Val::Px(1.0)), // Immer 1px Randbreite
                    // Volle Rundung ('rounded-full')
                    // Vollständig abrunden
                    // Sicherstellen, dass die Größe sich an den Inhalt anpasst
                    width: Val::Auto,
                    height: Val::Auto,
                    // Ggf. min_height für Konsistenz
                    min_height: Val::Px(18.0), // Höhe anpassen
                    ..default()
                },
                BorderRadius::percent(50.0, 50.0, 50.0, 50.0), // Vollständig abrunden
                BackgroundColor(bg_color),
                BorderColor(border_color),
            ))
            .with_children(|badge_node| {
                // Optional: Leading Icon hier spawnen

                // Text-Kind spawnen
                badge_node.spawn((
                    Text::new(self.text), // Text aus Builder
                    TextFont {
                        font: font_handle.clone(),
                        // Kleinere Schrift für Badges ('text-xs')
                        font_size: theme.font.font_size.base,
                        ..default()
                    },
                    TextColor(text_color), // Textfarbe
                ));

                // Optional: Trailing Icon hier spawnen
            })
            .id()
    }
}
