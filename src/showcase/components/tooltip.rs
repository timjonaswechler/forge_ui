use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_tooltip_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Tooltip", theme, font);
    section.with_children(|vc| {
        let _ = TooltipBuilder::new("Hover me", "Tooltip text").spawn(vc, theme, font);
    });
}
