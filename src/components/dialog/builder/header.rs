// components/dialog/header_builder.rs (oder ähnlich)

use super::SectionElementBuilderFn;
use crate::theme::UiTheme;
use bevy::prelude::*; // Angenommen, wir haben ein builder_types.rs Modul

#[derive(Default)]
pub struct DialogHeaderBuilder {
    title: Option<String>,
    subtitle: Option<String>,
    custom_elements: Vec<SectionElementBuilderFn>,
    // Weitere spezifische Header-Optionen hier, z.B. Icon
}

impl DialogHeaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Fügt benutzerdefinierten Inhalt zum Header hinzu.
    pub fn add_custom_content(
        mut self,
        element_builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.custom_elements.push(Box::new(element_builder));
        self
    }

    // Diese Methode wird vom DialogBuilder aufgerufen
    pub(crate) fn spawn_into(
        self, // Konsumiert den Builder
        parent: &mut ChildSpawnerCommands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) {
        // Hier Logik zum Spawnen von Titel, Subtitle und custom_elements
        if let Some(title_text) = self.title {
            parent.spawn((
                Text::new(title_text),
                TextFont {
                    font: font_handle.clone(),
                    font_size: theme.font.font_size.h4,
                    ..default()
                },
                TextColor(theme.color.gray.step12),
                Node {
                    margin: UiRect::bottom(Val::Px(theme.layout.gap.xs)), // Kleiner Abstand falls Subtitle folgt
                    ..default()
                },
            ));
        }
        if let Some(subtitle_text) = self.subtitle {
            parent.spawn((
                Text::new(subtitle_text),
                TextFont {
                    font: font_handle.clone(),
                    font_size: theme.font.font_size.sm,
                    ..default()
                },
                TextColor(theme.color.gray.step10),
            ));
        }

        for custom_element_fn in self.custom_elements {
            (custom_element_fn)(parent, theme, font_handle);
        }
    }
}
