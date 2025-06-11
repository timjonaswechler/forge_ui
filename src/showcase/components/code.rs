use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_code_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Code", theme, font);
    section.with_children(|vc| {
        CodeBuilder::new("fn main() {}")
            .spawn(vc, theme, font);
    });
}
