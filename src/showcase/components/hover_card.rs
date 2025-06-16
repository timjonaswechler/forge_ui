use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_hover_card_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Hover Card", theme, font);
    section.with_children(|vc| {
        let _ = HoverCardBuilder::new("Hover me")
            .content(|c, theme, font| {
                let _ = LabelBuilder::new("Card content").spawn(c, theme, font);
            })
            .spawn(vc, theme, font);
    });
}
