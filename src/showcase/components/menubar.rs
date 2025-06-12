use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_menubar_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Menubar", theme, font);
    section.with_children(|vc| {
        MenubarBuilder::new()
            .menu(
                MenubarMenuBuilder::new("File")
                    .item("New")
                    .item("Open")
                    .item("Save"),
            )
            .menu(MenubarMenuBuilder::new("Edit").item("Undo").item("Redo"))
            .spawn(vc, theme, font);
    });
}
