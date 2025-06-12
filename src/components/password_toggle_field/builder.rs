use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    PasswordHiddenTextMarker, PasswordInputMarker, PasswordToggleFieldMarker,
    PasswordToggleFieldState, PasswordToggleMarker, PasswordVisibleTextMarker,
    PasswordInputStyle, PasswordToggleStyle, PasswordToggleFieldStyle,
};

/// Builder for a simple password toggle field.
pub struct PasswordToggleFieldBuilder {
    password: String,
    visible: bool,
}

impl PasswordToggleFieldBuilder {
    pub fn new() -> Self {
        Self { password: "secret".into(), visible: false }
    }

    pub fn password(mut self, text: impl Into<String>) -> Self {
        self.password = text.into();
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for PasswordToggleFieldBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let bullet = "\u{2022}".repeat(self.password.len());

        let mut root = parent.spawn((
            PasswordToggleFieldMarker,
            PasswordToggleFieldStyle::new(theme),
            Name::new("PasswordToggleField"),
        ));

        root.with_children(|rc| {
            let mut input_cmd = rc.spawn((PasswordInputMarker, PasswordInputStyle::new(theme)));
            input_cmd.with_children(|ic| {
                let visible_label = LabelBuilder::new(self.password.clone()).spawn(ic, theme, font);
                ic.commands().entity(visible_label).insert(PasswordVisibleTextMarker);
                let hidden_label = LabelBuilder::new(bullet.clone()).spawn(ic, theme, font);
                ic.commands().entity(hidden_label).insert(PasswordHiddenTextMarker);
                ic.commands().entity(visible_label).insert(if self.visible { Visibility::Inherited } else { Visibility::Hidden });
                ic.commands().entity(hidden_label).insert(if self.visible { Visibility::Hidden } else { Visibility::Inherited });
            });

            rc.spawn((PasswordToggleMarker, PasswordToggleStyle::new(theme), Interaction::default()))
                .with_children(|tc| {
                    LabelBuilder::new("Toggle").spawn(tc, theme, font);
                });
        });

        root.insert(PasswordToggleFieldState { visible: self.visible, password: self.password });

        root.id()
    }
}

