use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_visually_hidden_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Visually Hidden", theme, font);

    section.with_children(|vc| {
        VisuallyHiddenBuilder::new("Hidden text").spawn(vc, theme, font);
    });
}
