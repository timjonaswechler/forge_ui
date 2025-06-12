use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{components::*, style::*};

/// Builder for a simple horizontal slider.
pub struct SliderBuilder {
    value: f32,
    width: Val,
}

impl Default for SliderBuilder {
    fn default() -> Self {
        Self { value: 0.0, width: Val::Px(120.0) }
    }
}

impl SliderBuilder {
    pub fn new() -> Self { Self::default() }

    /// Set the initial value between 0.0 and 1.0
    pub fn value(mut self, v: f32) -> Self {
        self.value = v.clamp(0.0, 1.0);
        self
    }

    /// Override the slider width.
    pub fn width(mut self, w: Val) -> Self {
        self.width = w;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for SliderBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, _font: &Handle<Font>) -> Self::Output {
        let mut root = parent.spawn((SliderMarker, SliderState { value: self.value }, Name::new("Slider")));
        root.with_children(|rc| {
            rc.spawn((
                SliderTrackMarker,
                SliderTrackStyle::new(theme, self.width),
            ))
            .with_children(|tc| {
                tc.spawn((
                    SliderRangeMarker,
                    SliderRangeStyle::new(theme, self.value),
                ));
                tc.spawn((SliderThumbMarker, SliderThumbStyle::new(theme)));
            });
        });
        root.id()
    }
}

