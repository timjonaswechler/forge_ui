use bevy::prelude::*;

use super::components::*;
use super::events::*;
use crate::components::button::{ButtonBuilder, ButtonVariant};
use crate::components::dialog::{
    DialogBodyBuilder, DialogBuilder, DialogContentBuilder, DialogFooterBuilder,
    DialogHeaderBuilder, DialogTriggerBuilder,
};
use crate::theme::UiTheme;

/// Builder for an alert dialog with a confirm and cancel button.
pub struct AlertDialogBuilder {
    id: AlertDialogId,
    title: String,
    description: Option<String>,
    confirm_label: String,
    cancel_label: String,
    initially_open: bool,
}

impl AlertDialogBuilder {
    pub fn new(id: AlertDialogId, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            description: None,
            confirm_label: "Confirm".into(),
            cancel_label: "Cancel".into(),
            initially_open: false,
        }
    }

    pub fn new_unique(title: impl Into<String>) -> Self {
        Self::new(AlertDialogId::new_unique(), title)
    }

    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn initially_open(mut self, flag: bool) -> Self {
        self.initially_open = flag;
        self
    }

    /// Spawns the alert dialog and returns the root entity.
    #[must_use]
    pub fn spawn(
        self,
        commands: &mut Commands,
        theme: &UiTheme,
        font: &Handle<Font>,
        portal_root: Option<Res<crate::components::portal::ForgeUiPortalRoot>>,
    ) -> Entity {
        let header = DialogHeaderBuilder::new().title(self.title);

        let fs = theme.font.size.base;
        let color = theme.color.slate.step12;
        let body = DialogBodyBuilder::new().add_content({
            let desc = self.description.clone();
            move |p, _theme, font| {
                if let Some(text) = desc.clone() {
                    p.spawn((
                        Text::new(text),
                        TextFont { font: font.clone(), font_size: fs, ..default() },
                        TextColor(color),
                    ));
                }
            }
        });

        let confirm_label = self.confirm_label.clone();
        let cancel_label = self.cancel_label.clone();
        let id = self.id;

        let footer = DialogFooterBuilder::new().add_custom_content(move |p, theme, font| {
            ButtonBuilder::<AlertDialogAction>::new_for_action()
                .text(cancel_label.clone())
                .variant(ButtonVariant::Soft)
                .action(AlertDialogAction::Cancel(id))
                .spawn(p, theme, font);

            ButtonBuilder::<AlertDialogAction>::new_for_action()
                .text(confirm_label.clone())
                .variant(ButtonVariant::Solid)
                .action(AlertDialogAction::Confirm(id))
                .spawn(p, theme, font);
        });

        let content = DialogContentBuilder::new()
            .header(header)
            .body(body)
            .footer(footer);

        DialogBuilder::new(self.id.into())
            .initially_open(self.initially_open)
            .content(content)
            .spawn(commands, theme, font, portal_root)
    }

    /// Spawns a trigger button that opens the alert dialog.
    #[must_use]
    pub fn spawn_trigger<'w, 's>(
        &self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
        label: impl Into<String>,
    ) -> EntityCommands<'s> {
        let id = self.id;
        DialogTriggerBuilder::new(id.into())
            .text(label)
            .variant(ButtonVariant::Solid)
            .spawn(parent, theme, font)
    }
}

impl From<AlertDialogId> for crate::components::dialog::DialogId {
    fn from(id: AlertDialogId) -> Self {
        crate::components::dialog::DialogId(id.0)
    }
}
