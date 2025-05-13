use crate::components::dialog::builder::{
    DialogBodyBuilder, DialogFooterBuilder, DialogHeaderBuilder,
};
use crate::components::dialog::helper::spawn_dialog_sections;
use crate::theme::UiTheme;
use crate::DialogId;
use bevy::prelude::*;

/// Bündelt Header, Body, Footer zu einem Ganzen.
#[derive(Default)]
pub struct DialogContentBuilder {
    header: Option<DialogHeaderBuilder>,
    body: Option<DialogBodyBuilder>,
    footer: Option<DialogFooterBuilder>,
}

impl DialogContentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // ---------- Weiterleitungs‑Setter ----------
    pub fn header(mut self, header: DialogHeaderBuilder) -> Self {
        self.header = Some(header);
        self
    }
    pub fn body(mut self, body: DialogBodyBuilder) -> Self {
        self.body = Some(body);
        self
    }
    pub fn footer(mut self, footer: DialogFooterBuilder) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Baut die drei Sektionen unter `parent` auf.
    pub(crate) fn spawn_into(
        self,
        parent: &mut ChildSpawnerCommands,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) {
        // Container‑Knoten anlegen (flex‑column)
        let content_style = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(theme.layout.gap.base),
            width: Val::Percent(100.0),
            ..default()
        };
        parent
            .spawn(content_style) // neue Entity für den gesamten Content
            .with_children(|content_parent: &mut ChildSpawnerCommands| {
                // ── Header / Body / Footer Platzhalter erzeugen
                let (header_e, body_e, footer_e) = spawn_dialog_sections(content_parent, theme);

                // Header füllen
                if let Some(h) = self.header {
                    content_parent // &mut ChildSpawnerCommands<'_>
                        .commands_mut() // &mut Commands
                        .entity(header_e) // -> EntityCommands
                        .with_children(|hdr| {
                            h.spawn_into(hdr, theme, font, DialogId::new_unique());
                            // TODO: DialogId bekommen. vom Root‑Dialog
                            // Header‑Builder weiterreichen
                        });
                }

                if let Some(b) = self.body {
                    content_parent
                        .commands_mut()
                        .entity(body_e)
                        .with_children(|bd| {
                            b.spawn_into(bd, theme, font);
                        });
                }

                if let Some(f) = self.footer {
                    content_parent
                        .commands_mut()
                        .entity(footer_e)
                        .with_children(|ft| {
                            f.spawn_into(ft, theme, font);
                        });
                }
            });
    }
}
