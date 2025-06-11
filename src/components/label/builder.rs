use bevy::prelude::*; // Für den Rückgabetyp von spawn

use crate::theme::UiTheme; // Theme für Styling
use super::components::LabelMarker;
/// forge_ui::label
///
/// Hilfs‑Modul zum komfortablen Erstellen konsistenter Text‑Labels
/// im **Bevy**‑UI.  Es kapselt wiederkehrende Parameter wie Farbe,
/// Schriftgröße, Margin und Ausrichtung und greift dabei zentral
/// auf ein [`UiTheme`](crate::theme::UiTheme) zurück.
///
/// # Beispiel
/// ```no_run
/// use bevy::prelude::*;
/// use forge_ui::label::LabelBuilder;
///
/// fn setup(mut cmds: Commands, theme: Res<UiTheme>, font: Res<Handle<Font>>) {
///     cmds.spawn(NodeBundle::default())
///         .with_children(|parent| {
///             LabelBuilder::new("Hallo Welt")
///                 .font_size(18.0)
///                 .color(Color::rgb(0.9, 0.2, 0.2))
///                 .spawn(parent, &theme, &font);
///         });
/// }
/// ```

/// Fluent‑Builder zum Erstellen von [`TextBundle`]‑Labels in Bevy.
///
/// Verwende `LabelBuilder::new("Mein Text")` und rufe anschließend
/// optionale Setter auf, um Farbe, Schriftgröße, Margin oder
/// Textausrichtung anzupassen.  Der abschließende Aufruf
/// [`spawn`](Self::spawn) fügt das Label als Kind‑Entity eines
/// gegebenen UI‑Parents hinzu.
///
/// *Standardwerte* werden aus dem bereitgestellten
/// [`UiTheme`](crate::theme::UiTheme) übernommen.
///
/// # Lebensdauer‑Parameter
/// * `'w` – World‑Lebensdauer für [`ChildSpawnerCommands`]
/// * `'a` – Borrow‑Lebensdauer für den Parent‑Spawner
///
/// # Fehler
/// Gibt das erzeugte [`Entity`] zurück; derzeit sind keine Fehlerfälle
/// möglich.
pub struct LabelBuilder {
    text: String,
    // Optional: Spezifische Farbe überschreiben?
    color: Option<Color>,
    // Optional: Spezifische Schriftgröße überschreiben?
    font_size: Option<f32>,
    margin: Option<UiRect>,
    // Optional: Styling-Variante (falls LabelStyle Enum verwendet wird)
    // label_style: LabelStyle,
    // Optional: Textausrichtung
    alignment: JustifyText,
}

impl LabelBuilder {
    /// Erstellt einen neuen `LabelBuilder` mit dem angegebenen Textinhalt.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
            font_size: None,
            margin: None,

            alignment: JustifyText::Left, // Standardmäßig linksbündig
        }
    }

    /// Überschreibt die Textfarbe, die sonst vom Theme kommt.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Überschreibt die Schriftgröße, die sonst Standard (z.B. 14.0) ist.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Setzt die Textausrichtung (Links, Zentriert, Rechts).
    pub fn align(mut self, alignment: JustifyText) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Spawnt das Label als Kind des gegebenen UI‑Parents und gibt
    /// das erstellte [`Entity`] zurück.
    ///
    /// Der Builder liest fehlende Werte (Farbe, Schriftgröße) aus dem
    /// übergebenen `theme` und verwendet den `font_handle` für das
    /// [`Text`]‑Element.
    #[must_use = "Commands should generally be used, e.g. to get the entity ID"]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) -> Entity {
        // Bestimme Farbe und Größe basierend auf Builder-Optionen und Theme
        let final_color = self.color.unwrap_or(theme.color.gray.step11); // Standard: Normale Vordergrundfarbe
        let final_font_size = self.font_size.unwrap_or(theme.font.size.base); // Standardgröße aus Theme

        // Optional: Farbe basierend auf LabelStyle anpassen
        // let final_color = match self.label_style {
        //     LabelStyle::Normal => self.color.unwrap_or(theme.foreground),
        //     LabelStyle::Muted => self.color.unwrap_or(theme.muted_foreground),
        //     LabelStyle::Error => self.color.unwrap_or(theme.destructive), // Beispiel
        // };

        let mut entity_commands = parent.spawn((
            LabelMarker,
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::visible(),
                margin: self.margin.unwrap_or_default(),
                ..default()
            },
        ));
        entity_commands.with_children(|builder| {
            builder.spawn((
                Text::new(self.text.clone()),
                TextFont {
                    font_size: final_font_size,
                    font: font_handle.clone(),
                    ..default()
                },
                TextLayout::new_with_justify(self.alignment).with_no_wrap(),
                TextColor(final_color),
            ));
        });
        entity_commands.id()
    }
}
