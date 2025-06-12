use bevy::prelude::*;

use crate::components::{
    label::LabelBuilder,
    radio::{RadioBuilder, RadioSize, RadioVariant},
};
use crate::theme::UiTheme;

use super::{
    components::RadioCardMarker,
    style::{spawn_disabled_overlay, RadioCardStyle},
};

/// Fluent builder for a single radio card with label.
pub struct RadioCardBuilder {
    label: String,
    value: String,
    group: String,
    checked: bool,
    disabled: bool,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl RadioCardBuilder {
    /// Creates a new radio card for the given `value` and `label` in the provided `group`.
    pub fn new(
        value: impl Into<String>,
        label: impl Into<String>,
        group: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            group: group.into(),
            checked: false,
            disabled: false,
            markers: Vec::new(),
        }
    }

    /// Set whether the card is initially selected.
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

    /// Spawn the radio card as a child of `parent` and return the created entity.
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> EntityCommands<'s> {
        let style = RadioCardStyle::new(theme);

        let mut cmd = parent.spawn((RadioCardMarker, style.clone()));

        for marker in self.markers {
            marker(&mut cmd);
        }

        cmd.with_children(|p| {
            let _ = RadioBuilder::new(&self.value)
                .variant(RadioVariant::Primary)
                .size(RadioSize::Medium)
                .group(self.group.clone())
                .checked(self.checked)
                .disabled(self.disabled)
                .spawn(p, theme, font);

            let _ = LabelBuilder::new(self.label.clone())
                .margin(UiRect::left(Val::Px(8.0)))
                .spawn(p, theme, font);
        });

        if self.disabled {
            spawn_disabled_overlay(&mut cmd, theme, style.border_radius);
        }

        cmd
    }
}
