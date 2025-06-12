use bevy::prelude::*;

use crate::assets::IconAssets;
use crate::components::{checkbox::CheckboxBuilder, label::LabelBuilder};
use crate::theme::UiTheme;

use super::{
    components::CheckboxCardMarker,
    style::{spawn_disabled_overlay, CheckboxCardStyle},
};

/// Fluent builder to create a simple checkbox card with a label.
pub struct CheckboxCardBuilder {
    label: String,
    checked: bool,
    disabled: bool,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl CheckboxCardBuilder {
    /// Start a new card with the given label text.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            disabled: false,
            markers: Vec::new(),
        }
    }

    /// Set the initial checked state.
    pub fn checked(mut self, flag: bool) -> Self {
        self.checked = flag;
        self
    }

    /// Disable the card.
    pub fn disabled(mut self, flag: bool) -> Self {
        self.disabled = flag;
        self
    }

    /// Attach a custom modification to the spawned entity.
    pub fn add_marker<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    {
        self.markers.push(Box::new(func));
        self
    }

    /// Spawn the checkbox card as a child of the given parent node and return the created entity.
    #[must_use]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
        icons: &Res<IconAssets>,
    ) -> EntityCommands<'a> {
        let style = CheckboxCardStyle::new(theme);

        let mut cmd = parent.spawn((CheckboxCardMarker, style.clone()));

        for marker in self.markers {
            marker(&mut cmd);
        }

        cmd.with_children(|p| {
            let _ = CheckboxBuilder::new()
                .checked(self.checked)
                .disabled(self.disabled)
                .spawn(p, theme, icons);

            let _ = LabelBuilder::new(self.label)
                .margin(UiRect::left(Val::Px(8.0)))
                .spawn(p, theme, font);
        });

        if self.disabled {
            spawn_disabled_overlay(&mut cmd, theme, style.border_radius);
        }

        cmd
    }
}
