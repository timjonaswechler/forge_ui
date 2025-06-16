use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_progress_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Progress", theme, font);
    section.with_children(|vc| {
        let _ = ProgressBuilder::new().value(0.5).spawn(vc, theme, font);
    });
}
