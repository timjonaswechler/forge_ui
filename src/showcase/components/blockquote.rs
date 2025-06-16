use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_blockquote_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Blockquote", theme, font);

    section.with_children(|vc| {
        let _ = BlockquoteBuilder::new("To be, or not to be, that is the question.")
            .cite("William Shakespeare")
            .spawn(vc, theme, font);
    });
}
