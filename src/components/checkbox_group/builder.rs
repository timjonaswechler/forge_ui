use bevy::prelude::*;
use std::collections::HashSet;

use crate::assets::IconAssets;
use crate::components::{checkbox::CheckboxBuilder, label::LabelBuilder};
use crate::theme::UiTheme;

use super::components::{CheckboxGroupItem, CheckboxGroupMarker, CheckboxGroupOrientation, CheckboxGroupState};

/// Builder to spawn a group of checkboxes with labels.
pub struct CheckboxGroupBuilder {
    orientation: CheckboxGroupOrientation,
    disabled: bool,
    options: Vec<(String, String)>,
    checked: HashSet<String>,
}

impl CheckboxGroupBuilder {
    /// Create a new builder instance.
    pub fn new() -> Self {
        Self {
            orientation: CheckboxGroupOrientation::Vertical,
            disabled: false,
            options: Vec::new(),
            checked: HashSet::new(),
        }
    }

    /// Set the group orientation.
    pub fn orientation(mut self, o: CheckboxGroupOrientation) -> Self {
        self.orientation = o;
        self
    }

    /// Disable the entire group.
    pub fn disabled(mut self, flag: bool) -> Self {
        self.disabled = flag;
        self
    }

    /// Add an option with value and label text.
    pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }

    /// Mark an option as initially checked.
    pub fn checked(mut self, value: impl Into<String>) -> Self {
        self.checked.insert(value.into());
        self
    }

    /// Spawn the configured checkbox group as a child of `parent`.
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
        icons: &Res<IconAssets>,
    ) -> EntityCommands<'s> {
        let mut cmd = parent.spawn( (
            Node {
                display: Display::Flex,
                flex_direction: match self.orientation {
                    CheckboxGroupOrientation::Horizontal => FlexDirection::Row,
                    CheckboxGroupOrientation::Vertical => FlexDirection::Column,
                },
                align_items: AlignItems::Center,
                column_gap: Val::Px(theme.layout.gap.sm),
                row_gap: Val::Px(theme.layout.gap.sm),
                padding: UiRect::all(Val::Px(theme.layout.padding.sm)),
                ..default()
            },
            CheckboxGroupMarker,
            CheckboxGroupState {
                orientation: self.orientation,
                disabled: self.disabled,
                checked_values: self.checked.clone(),
            },
        ));

        let options = self.options;
        let checked = self.checked;
        let disabled = self.disabled;

        cmd.with_children(move |parent| {
            for (value, label_text) in options.into_iter() {
                let is_checked = checked.contains(&value);
                parent
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(theme.layout.gap.xs),
                        row_gap: Val::Px(theme.layout.gap.xs),
                        ..default()
                    })
                    .with_children(|cb| {
                        CheckboxBuilder::new()
                            .checked(is_checked)
                            .disabled(disabled)
                            .add_marker({
                                let v = value.clone();
                                move |ec| {
                                    ec.insert(CheckboxGroupItem { value: v.clone() });
                                }
                            })
                            .spawn(cb, theme, icons);

                        LabelBuilder::new(label_text)
                            .margin(UiRect::left(Val::Px(4.0)))
                            .spawn(cb, theme, font);
                    });
            }
        });

        cmd
    }
}
