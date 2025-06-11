use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_callout_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Callout", theme, font);

    section.with_children(|vc| {
        CalloutBuilder::new("Something went wrong")
            .variant(CalloutVariant::Error)
            .spawn(vc, theme, font);
    });
}
