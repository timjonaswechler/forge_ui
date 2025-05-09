// src/components/portal/builder.rs
use super::components::ForgeUiPortalRoot;
use crate::theme::UiTheme;
use bevy::prelude::*;

pub(crate) struct PortalContentBuilder;

impl PortalContentBuilder {
    /// Spawnt den gegebenen Inhalt als Kind der `target_container_entity`.
    #[must_use]
    pub(crate) fn spawn_at_target<'w, 's>(
        commands: &'s mut Commands, // Nimmt `Commands` direkt
        target_container_entity: Entity,
        theme: &UiTheme,            // Für den Inhalt benötigt
        font_handle: &Handle<Font>, // Für den Inhalt benötigt
        content_builder_fn: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Vec<Entity> {
        // Gibt die IDs der gespawnten Top-Level-Kinder zurück
        let spawned_child_entities = Vec::new();
        commands
            .entity(target_container_entity)
            .with_children(|parent| {
                // Wir können hier keine EntityCommands direkt zurückgeben, da with_children das nicht einfach erlaubt.
                // Stattdessen rufen wir die content_builder_fn direkt auf und geben parent weiter.
                (content_builder_fn)(parent, theme, font_handle);

                // Wenn man IDs zurückgeben MÖCHTE (komplexer, erfordert Kooperation des content_builder_fn):
                // let mut entities_built = Vec::new();
                // (content_builder_fn)(&mut parent.entity_commands(), theme, font_handle, &mut entities_built);
                // spawned_child_entities = entities_built;
            });
        spawned_child_entities
    }

    /// Spawnt den gegebenen Inhalt als Kind des globalen Portal-Roots.
    /// Diese Funktion benötigt Zugriff auf die `GlobalPortalRoot` Ressource.
    #[must_use]
    pub(crate) fn spawn_in_global_root(
        commands: &mut Commands,
        global_root: Res<ForgeUiPortalRoot>, // Als Systemparameter reinbekommen oder vorher abfragen
        theme: &UiTheme,
        font_handle: &Handle<Font>,
        content_builder_fn: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Vec<Entity> {
        Self::spawn_at_target(
            commands,
            global_root.0,
            theme,
            font_handle,
            content_builder_fn,
        )
    }
}
