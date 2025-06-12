use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_popover_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Popover", theme, font);
    section.with_children(|vc| {
        PopoverBuilder::new("Open Popover")
            .content(|c, theme, font| {
                LabelBuilder::new("Popover content").spawn(c, theme, font);
            })
            .spawn(vc, theme, font);
    });
}

