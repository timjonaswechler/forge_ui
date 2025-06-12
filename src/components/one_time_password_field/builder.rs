use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{OtpFieldStyle, OtpInputMarker, OtpInputStyle, OneTimePasswordFieldMarker};

/// Builder for a simple one-time password field.
pub struct OneTimePasswordFieldBuilder {
    length: usize,
}

impl OneTimePasswordFieldBuilder {
    pub fn new() -> Self {
        Self { length: 6 }
    }

    pub fn length(mut self, len: usize) -> Self {
        self.length = len;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for OneTimePasswordFieldBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut root = parent.spawn((OneTimePasswordFieldMarker, OtpFieldStyle::new(theme), Name::new("OTPField")));
        root.with_children(|rc| {
            for _ in 0..self.length {
                let mut input_cmd = rc.spawn((OtpInputMarker, OtpInputStyle::new(theme), Interaction::default()));
                input_cmd.with_children(|ic| {
                    LabelBuilder::new("\u{00A0}").spawn(ic, theme, font);
                });
            }
        });
        root.id()
    }
}
