use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_select_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Select", theme, font);
    section.with_children(|vc| {
        SelectBuilder::new()
            .option("apple", "Apple")
            .option("banana", "Banana")
            .option("cherry", "Cherry")
            .spawn(vc, theme, font);
    });
}
