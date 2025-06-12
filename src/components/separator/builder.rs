use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{SeparatorMarker, SeparatorOrientation, SeparatorState, style::SeparatorStyle};

pub struct SeparatorBuilder {
    orientation: SeparatorOrientation,
    decorative: bool,
    length: Val,
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            decorative: false,
            length: Val::Percent(100.0),
        }
    }
}

impl SeparatorBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn orientation(mut self, o: SeparatorOrientation) -> Self {
        self.orientation = o;
        self
    }

    pub fn decorative(mut self, flag: bool) -> Self {
        self.decorative = flag;
        self
    }

    pub fn length(mut self, l: Val) -> Self {
        self.length = l;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for SeparatorBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, _font: &Handle<Font>) -> Self::Output {
        parent
            .spawn((
                SeparatorMarker,
                SeparatorState { orientation: self.orientation, decorative: self.decorative },
                SeparatorStyle::new(theme, self.orientation, self.length),
                Name::new("Separator"),
            ))
            .id()
    }
}
