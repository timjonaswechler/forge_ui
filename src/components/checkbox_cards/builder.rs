use bevy::prelude::*;

use crate::assets::IconAssets;
use crate::components::{checkbox::CheckboxBuilder, label::LabelBuilder};
use crate::theme::UiTheme;

use super::components::CheckboxCardMarker;

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
        let mut cmd = parent.spawn((
            CheckboxCardMarker,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(theme.color.slate.step01),
            BorderColor(theme.color.slate.step06),
            BorderRadius::all(Val::Px(theme.layout.radius.sm)),
        ));

        for marker in self.markers {
            marker(&mut cmd);
        }

        cmd.with_children(|p| {
            CheckboxBuilder::new()
                .checked(self.checked)
                .disabled(self.disabled)
                .spawn(p, theme, icons);

            LabelBuilder::new(self.label)
                .spawn(p, theme, font);
        });

        cmd
    }
}
