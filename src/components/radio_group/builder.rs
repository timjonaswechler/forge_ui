use bevy::prelude::*;

use crate::components::{
    label::LabelBuilder, // already exists in forge_ui
    radio::{RadioBuilder, RadioSize, RadioVariant},
};
use crate::theme::UiTheme;

use super::components::{RadioGroupMarker, RadioGroupState};

/// Orientation of the group – controls `flex_direction` on the root `Node`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RadioGroupOrientation {
    Horizontal,
    Vertical,
}

/// Fluent builder to spawn a complete `RadioGroup` including its contained
/// `Radio` buttons and labels.
pub struct RadioGroupBuilder {
    name: String,
    disabled: bool,
    orientation: RadioGroupOrientation,
    options: Vec<(String, String)>, // (value, label)
    selected: Option<String>,
}

impl RadioGroupBuilder {
    /// Create a new builder instance. `name` must be **unique** within the UI so
    /// that the radio selection logic (see `handle_radio_click`) can identify
    /// sibling buttons that belong together.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            disabled: false,
            orientation: RadioGroupOrientation::Vertical,
            options: Vec::new(),
            selected: None,
        }
    }

    pub fn disabled(mut self, flag: bool) -> Self {
        self.disabled = flag;
        self
    }

    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Pre‑select the radio whose `value` matches the given string.
    pub fn selected(mut self, value: impl Into<String>) -> Self {
        self.selected = Some(value.into());
        self
    }

    /// Add a new radio option – takes a machine‑readable `value` and a human
    /// readable `label`.
    pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }

    /// Spawns the group under `parent` and returns the created `EntityCommands`
    /// so you can attach additional components if required.
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> EntityCommands<'s> {
        // ── Root container ────────────────────────────────────────────────────
        let mut group_entity = parent.spawn((
            Node {
                display: Display::Flex,
                flex_direction: match self.orientation {
                    RadioGroupOrientation::Horizontal => FlexDirection::Row,
                    RadioGroupOrientation::Vertical => FlexDirection::Column,
                },
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                ..default()
            },
            RadioGroupMarker,
            RadioGroupState {
                name: self.name.clone(),
                selected_value: self.selected.clone(),
                disabled: self.disabled,
            },
        ));

        // ── Child radios + labels ─────────────────────────────────────────────
        group_entity.with_children({
            let name = self.name.clone();
            let selected = self.selected.clone();
            let disabled = self.disabled;
            let options = self.options;
            move |parent| {
                for (value, label_text) in options.into_iter() {
                    let is_checked = selected.as_ref().map_or(false, |s| s == &value);

                    // A small container holding the radio and its label side‑by‑side
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                            ..default()
                        })
                        .with_children(|parent| {
                            let _ = RadioBuilder::new(&value)
                                .variant(RadioVariant::Primary)
                                .size(RadioSize::Medium)
                                .group(name.clone())
                                .disabled(disabled)
                                .checked(is_checked)
                                .spawn(parent, theme, font);

                            let _ = LabelBuilder::new(label_text).spawn(parent, theme, font);
                        });
                }
            }
        });

        group_entity
    }
}
