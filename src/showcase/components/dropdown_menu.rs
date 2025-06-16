use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_dropdown_menu_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Dropdown Menu", theme, font);
    section.with_children(|vc| {
        let _ = DropdownMenuBuilder::new("Options")
            .item("First")
            .item("Second")
            .item("Third")
            .spawn(vc, theme, font);
    });
}
