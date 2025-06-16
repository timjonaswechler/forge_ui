use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_context_menu_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Context Menu", theme, font);
    section.with_children(|vc| {
        let _ = ContextMenuBuilder::new()
            .item("First")
            .item("Second")
            .item("Third")
            .spawn(vc, theme, font);
    });
}
