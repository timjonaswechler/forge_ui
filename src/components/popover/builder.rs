use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    PopoverContentMarker, PopoverContentStyle, PopoverMarker, PopoverState, PopoverTriggerMarker,
};

/// Builder for a simple popover component.
pub struct PopoverBuilder {
    trigger: String,
    content:
        Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
    open: bool,
}

impl PopoverBuilder {
    pub fn new(trigger: impl Into<String>) -> Self {
        Self {
            trigger: trigger.into(),
            content: None,
            open: false,
        }
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for PopoverBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let mut root = parent.spawn((
            PopoverMarker,
            PopoverState { open: self.open },
            Name::new("Popover"),
        ));
        root.with_children(|rc| {
            rc.spawn((
                PopoverTriggerMarker,
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Interaction::default(),
                FocusPolicy::Block,
            ))
            .with_children(|tc| {
                let _ = LabelBuilder::new(self.trigger.clone()).spawn(tc, theme, font);
            });

            let mut content_cmd = rc.spawn((
                PopoverContentMarker,
                PopoverContentStyle::new(theme),
                if self.open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
            ));
            if let Some(f) = self.content {
                content_cmd.with_children(|cc| {
                    f(cc, theme, font);
                });
            }
        });
        root.id()
    }
}
