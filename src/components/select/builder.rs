use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    components::{
        SelectContentMarker, SelectMarker, SelectOptionMarker, SelectState, SelectTriggerMarker,
        SelectTriggerTextMarker,
    },
    style::{SelectContentStyle, SelectOptionStyle, SelectRootStyle, SelectTriggerStyle},
};

/// Builder for a simple select dropdown.
pub struct SelectBuilder {
    options: Vec<(String, String)>,
    selected: Option<String>,
    width: Option<Val>,
    open: bool,
}

impl Default for SelectBuilder {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            width: None,
            open: false,
        }
    }
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn option(mut self, value: impl Into<String>, label: impl Into<String>) -> Self {
        self.options.push((value.into(), label.into()));
        self
    }

    pub fn selected(mut self, value: impl Into<String>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    pub fn open(mut self, flag: bool) -> Self {
        self.open = flag;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for SelectBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let mut root_style = SelectRootStyle::new(theme);
        if let Some(w) = self.width {
            root_style.node.width = w;
        }
        let mut root = parent.spawn((
            SelectMarker,
            root_style,
            SelectState {
                open: self.open,
                selected: self.selected.clone(),
            },
            Name::new("Select"),
        ));

        root.with_children(|rc| {
            // Trigger
            rc.spawn((
                SelectTriggerMarker,
                SelectTriggerStyle::new(theme),
                Interaction::default(),
                FocusPolicy::Block,
            ))
            .with_children(|tc| {
                let display = self
                    .selected
                    .as_ref()
                    .and_then(|v| {
                        self.options
                            .iter()
                            .find(|(val, _)| val == v)
                            .map(|(_, l)| l.clone())
                    })
                    .unwrap_or_else(|| "Select".to_string());
                let text_entity = LabelBuilder::new(display).spawn(tc, theme, font);
                tc.commands()
                    .entity(text_entity)
                    .insert(SelectTriggerTextMarker);
            });

            // Content with options
            let mut content = rc.spawn((
                SelectContentMarker,
                SelectContentStyle::new(theme),
                if self.open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
            ));
            content.with_children(|cc| {
                for (value, label_text) in self.options {
                    cc.spawn((
                        SelectOptionMarker {
                            value: value.clone(),
                            label: label_text.clone(),
                        },
                        SelectOptionStyle::new(theme),
                        Interaction::default(),
                        FocusPolicy::Block,
                    ))
                    .with_children(|oc| {
                        let _ = LabelBuilder::new(label_text).spawn(oc, theme, font);
                    });
                }
            });
        });

        root.id()
    }
}
