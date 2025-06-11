use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

/// Demonstrates the radio card component.
pub fn show_radio_card_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Radio Cards", theme, font);

    section.with_children(|vc| {
        RadioCardBuilder::new("one", "First", "demo")
            .spawn(vc, theme, font);

        RadioCardBuilder::new("two", "Second", "demo")
            .checked(true)
            .spawn(vc, theme, font);
    });
}
