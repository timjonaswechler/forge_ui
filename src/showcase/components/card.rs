use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_card_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Card", theme, font);
    section.with_children(|vc| {
        CardBuilder::new().content(|c, theme, font| {
            LabelBuilder::new("Card content")
                .spawn(c, theme, font);
        }).spawn(vc, theme, font);
    });
}
