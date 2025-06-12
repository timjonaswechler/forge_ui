use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ProgressIndicatorMarker, ProgressMarker, ProgressState, ProgressTrackMarker};
use super::style::{ProgressIndicatorStyle, ProgressTrackStyle};

pub struct ProgressBuilder {
    value: f32,
    max: f32,
}

impl Default for ProgressBuilder {
    fn default() -> Self {
        Self { value: 0.0, max: 1.0 }
    }
}

impl ProgressBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ProgressBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, _font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((ProgressMarker, ProgressState { value: self.value, max: self.max }, Name::new("Progress")));
        cmd.with_children(|c| {
            c.spawn((ProgressTrackMarker, ProgressTrackStyle::new(theme)));
            c.spawn((
                ProgressIndicatorMarker,
                ProgressIndicatorStyle::new(theme, self.value / self.max),
            ));
        });
        cmd.id()
    }
}
