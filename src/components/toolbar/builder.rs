use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ToolbarMarker, ToolbarOrientation, ToolbarStyle};

pub struct ToolbarBuilder {
    orientation: ToolbarOrientation,
    controls: Vec<Box<dyn FnMut(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl ToolbarBuilder {
    pub fn new() -> Self {
        Self { orientation: ToolbarOrientation::Horizontal, controls: Vec::new() }
    }

    pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn control<F>(mut self, func: F) -> Self
    where
        F: FnMut(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.controls.push(Box::new(func));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ToolbarBuilder {
    type Output = Entity;

    fn spawn(mut self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((ToolbarMarker, ToolbarStyle::new(theme, self.orientation), Name::new("Toolbar")));
        cmd.with_children(|cb| {
            for mut func in self.controls {
                func(cb, theme, font);
            }
        });
        cmd.id()
    }
}
