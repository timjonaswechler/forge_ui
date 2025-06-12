use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_toolbar_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Toolbar", theme, font);
    section.with_children(|vc| {
        ToolbarBuilder::new()
            .control(|p, t, f| {
                ButtonBuilder::new().text("One").spawn(p, t, f);
            })
            .control(|p, t, f| {
                ButtonBuilder::new().text("Two").spawn(p, t, f);
            })
            .spawn(vc, theme, font);
    });
}
