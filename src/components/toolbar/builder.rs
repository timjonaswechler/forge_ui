use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ToolbarMarker, ToolbarOrientation, ToolbarState, style::ToolbarStyle};

/// Builder for a simple toolbar container.
pub struct ToolbarBuilder {
    orientation: ToolbarOrientation,
    items: Vec<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>, 
}

impl ToolbarBuilder {
    pub fn new() -> Self {
        Self { orientation: ToolbarOrientation::Horizontal, items: Vec::new() }
    }

    /// Set the toolbar orientation.
    pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Add a custom child item using a closure.
    pub fn item<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.items.push(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ToolbarBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((
            ToolbarMarker,
            ToolbarState { orientation: self.orientation },
            ToolbarStyle::new(self.orientation, theme),
            Name::new("Toolbar"),
        ));

        cmd.with_children(|cb| {
            for item in self.items {
                item(cb, theme, font);
            }
        });

        cmd.id()
    }
}
