use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_one_time_password_field_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "One Time Password Field", theme, font);
    section.with_children(|vc| {
        let _ = OneTimePasswordFieldBuilder::new().spawn(vc, theme, font);
    });
}
