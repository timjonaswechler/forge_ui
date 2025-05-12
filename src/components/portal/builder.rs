// src/components/portal/builder.rs
use super::components::ForgeUiPortalRoot;
use crate::theme::UiTheme;
use bevy::prelude::*;

pub(crate) struct PortalContentBuilder;

impl PortalContentBuilder {
    /// Spawnt dynamischen UI‑Inhalt unter `target_container_entity`
    /// und gibt die obersten neu erzeugten Kinder zurück.
    pub(crate) fn spawn_at_target<'w>(
        parent: &mut ChildSpawnerCommands<'w>,
        target_container_entity: Entity,
        theme: &UiTheme,
        font: &Handle<Font>,
        content_builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Vec<Entity> {
        // ‑‑> Root‑Node für diesen Content anlegen
        let mut root_cmd = parent.spawn_empty(); // neues leeres Entity
        root_cmd.insert(ChildOf(target_container_entity));

        let root_id = root_cmd.id();

        // jetzt die eigentlichen Widgets darunter bauen
        root_cmd.with_children(|builder| {
            content_builder(builder, theme, font);
        });

        vec![root_id] // oder sammle mehr, wenn nötig
    }

    /// Variante: hängt an das globale Portal‑Root an.
    pub(crate) fn spawn_in_global_root<'w>(
        parent: &mut ChildSpawnerCommands<'w>,
        global_root: Res<ForgeUiPortalRoot>,
        theme: &UiTheme,
        font: &Handle<Font>,
        content_builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Vec<Entity> {
        Self::spawn_at_target(parent, global_root.0, theme, font, content_builder)
    }
}
