// NEUE DATEI: /Users/tim-jonaswechler/GitHub-Projekte/forge_of_stories/crates/forge_ui/src/components/dialog/builder/trigger.rs
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use super::super::*;

use crate::components::button::*;
use crate::theme::UiTheme;
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
            variant: ButtonVariant::Solid,
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
        let mut button_builder = ButtonBuilder::<DialogAction>::new("alert_dialog_close")
            .action(DialogAction::Open(self.target_dialog_id))
            .variant(self.variant)
            .size(self.size);

        if let Some(text) = self.text {
            button_builder = button_builder.text(text);
        }
        // if let Some(icon_handle) = self.icon {
        //     button_builder = button_builder.icon(icon_handle);
        // }

        for config_fn in self.custom_button_setup {
            (config_fn)(&mut button_builder);
        }
        parent.spawn((button_builder.build(theme, font_handle), ButtonMarker))
    }
}
