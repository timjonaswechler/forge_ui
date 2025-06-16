use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_scroll_area_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Scroll Area", theme, font);
    section.with_children(|vc| {
        let _ = ScrollAreaBuilder::new().content(|c, theme, font| {
            for i in 0..10 {
                let text = format!("Item {}", i + 1);
                let label = LabelBuilder::new(text);
                let _ = label.spawn(c, theme, font);
            }
        }).spawn(vc, theme, font);
    });
}
