use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_toast_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Toast", theme, font);
    section.with_children(|vc| {
        let _ = ToastBuilder::new("Hello from Toast").duration(2.0).spawn(vc, theme, font);
    });
}
