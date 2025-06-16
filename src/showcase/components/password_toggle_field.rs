use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_password_toggle_field_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Password Toggle Field", theme, font);
    section.with_children(|vc| {
        let _ = PasswordToggleFieldBuilder::new().spawn(vc, theme, font);
    });
}

