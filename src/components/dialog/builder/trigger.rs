// NEUE DATEI: /Users/tim-jonaswechler/GitHub-Projekte/forge_of_stories/crates/forge_ui/src/components/dialog/builder/trigger.rs

use crate::components::button::{ButtonBuilder, ButtonSize, ButtonVariant}; // ButtonChild ggf. anpassen, falls Icons anders gehandhabt werden
use crate::components::dialog::components::DialogId;
use crate::theme::UiTheme;
use crate::DialogAction;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct DialogTriggerBuilder {
    target_dialog_id: DialogId,
    text: Option<String>,
    icon: Option<Handle<Image>>, // Angenommen, ButtonBuilder kann Icons
    variant: ButtonVariant,
    size: ButtonSize,
    custom_button_setup: Vec<Box<dyn FnOnce(&mut ButtonBuilder<DialogAction>) + Send + Sync>>,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl DialogTriggerBuilder {
    pub fn new(target_dialog_id: DialogId) -> Self {
        Self {
            target_dialog_id,
            text: None,
            icon: None,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            custom_button_setup: Vec::new(),
            markers: Vec::new(),
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn icon(mut self, icon_handle: Handle<Image>) -> Self {
        self.icon = Some(icon_handle);
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Ermöglicht eine erweiterte Konfiguration des internen `ButtonBuilder`.
    pub fn configure_button(
        mut self,
        config_fn: impl FnOnce(&mut ButtonBuilder<DialogAction>) + Send + Sync + 'static,
    ) -> Self {
        self.custom_button_setup.push(Box::new(config_fn));
        self
    }

    /// Fügt eine benutzerdefinierte Funktion hinzu, die direkt auf die [`EntityCommands`]
    /// des Trigger-Buttons nach dem Spawnen angewendet wird.
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }

    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) -> EntityCommands<'s> {
        let mut button_builder = ButtonBuilder::<DialogAction>::new_for_action()
            .action(DialogAction::Open(self.target_dialog_id))
            .variant(self.variant)
            .size(self.size);

        if let Some(text) = self.text {
            button_builder = button_builder.text(text);
        }
        if let Some(icon_handle) = self.icon {
            button_builder = button_builder.icon(icon_handle);
        }

        for config_fn in self.custom_button_setup {
            (config_fn)(&mut button_builder);
        }

        // Wichtig: Die `add_marker` vom ButtonBuilder selbst wird hier für die Payload `OpenDialogActionPayload` nicht benötigt,
        // da wir die Action direkt setzen. Die `add_marker` hier im `DialogTriggerBuilder`
        // ist für zusätzliche, benutzerdefinierte Marker auf der Button-Entität.
        // Um Verwirrung zu vermeiden und die `add_marker`-Logik klar zu trennen:
        // Der ButtonBuilder hat seine eigene `add_marker`. Wenn wir Marker für den DialogTrigger-Button
        // außerhalb der `OpenDialogActionPayload`-Komponente hinzufügen wollen, können wir das so machen:
        let mut entity_cmds = button_builder.spawn(parent, theme, font_handle);
        for marker_fn in self.markers {
            (marker_fn)(&mut entity_cmds);
        }
        entity_cmds
    }
}
