use super::SectionElementBuilderFn;
use crate::theme::UiTheme;
use bevy::prelude::*;

#[derive(Default)]
pub struct DialogBodyBuilder {
    elements: Vec<SectionElementBuilderFn>,
    // Spezifische Optionen für den Body, z.B. automatisches Scrolling etc.
}

impl DialogBodyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_content(
        mut self,
        element_builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.elements.push(Box::new(element_builder));
        self
    }

    pub(crate) fn spawn_into(
        self,
        parent: &mut ChildSpawnerCommands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) {
        for element_fn in self.elements {
            (element_fn)(parent, theme, font_handle);
        }
    }
}
