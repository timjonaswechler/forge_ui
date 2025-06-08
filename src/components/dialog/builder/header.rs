// components/dialog/header_builder.rs (oder ähnlich)
use bevy::{color, prelude::*}; // Angenommen, wir haben ein builder_types.rs Modul

use super::super::*;
use super::*;
use crate::components::button::*;
use crate::layout::*;
use crate::theme::UiTheme;

#[derive(Default)]
pub struct DialogHeaderBuilder {
    title: Option<String>,
    subtitle: Option<String>,
    custom_elements: Vec<SectionElementBuilderFn>,

    show_close: bool,
    close_icon: Handle<Image>,
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

    pub fn with_close_button(
        mut self,
        icon: Handle<Image>, // None ⇒ Text‑Button «×»
                             // z. B. `ButtonVariant::Ghost`
                             // z. B. `ButtonSize::Xs`
    ) -> Self {
        self.show_close = true;
        self.close_icon = icon;

        self
    }

    // Diese Methode wird vom DialogBuilder aufgerufen
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn spawn_into(
        self,
        parent: &mut ChildSpawnerCommands,
        theme: &UiTheme,
        font: &Handle<Font>,
        dialog_id: DialogId,
    ) {
        // move fields so we can use them inside the closure without borrowing `self`
        let title = self.title;
        let subtitle = self.subtitle;
        let custom_elems = self.custom_elements; // will be moved into closure
        let show_close = self.show_close;
        let close_icon = self.close_icon;

        // outer horizontal row (space‑between)
        let mut row_cmd = HorizontalStackBuilder::new()
            .justify(JustifyContent::SpaceBetween)
            .align(AlignItems::FlexStart)
            .width(Val::Percent(100.0))
            .spawn(parent);

        row_cmd.with_children(move |row| {
            let mut col_entity = VerticalStackBuilder::new("Header")
                .align(AlignItems::FlexStart)
                .gap(Val::Px(theme.layout.gap.xs))
                .spawn(row);

            col_entity.with_children(|col| {
                // Titel / Untertitel sind ebenfalls Kinder der Column
                if let Some(t) = title.clone() {
                    col.spawn((
                        Text::new(t),
                        TextFont {
                            font: font.clone(),
                            font_size: theme.font.size.h4,
                            ..default()
                        },
                        TextColor(theme.color.gray.step12),
                    ));
                }
                if let Some(st) = subtitle.clone() {
                    col.spawn((
                        Text::new(st),
                        TextFont {
                            font: font.clone(),
                            font_size: theme.font.size.sm,
                            ..default()
                        },
                        TextColor(theme.color.gray.step10),
                    ));
                }

                // Custom‑Element‑Callbacks ausführen
                for elem in custom_elems {
                    (elem)(col, theme, font); // ← jetzt korrekt!
                }
            });

            // ── right close button ──────────────────────────────────────────
            if show_close {
                let btn = ButtonBuilder::<DialogAction>::new_for_action()
                    .action(DialogAction::Close(dialog_id))
                    .variant(ButtonVariant::Soft)
                    .color(theme.color.red.clone());

                // btn = btn.icon(close_icon);

                let mut btn_cmd = btn.spawn(row, theme, font);
                // push to the far right & top
                btn_cmd.insert(Node {
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Auto,
                        ..default()
                    },
                    ..default()
                });
            }
        });
    }
}
