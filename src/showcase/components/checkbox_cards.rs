use super::super::helpers::*;
use crate::prelude::*;
use crate::components::checkbox_cards::CheckboxCardBuilder;
use bevy::prelude::*;

/// Demonstrates the simple checkbox card component.
pub fn show_checkbox_card_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    icons: &Res<IconAssets>,
) {
    let mut section = create_variant_section(parent, "Checkbox Cards", theme, font);

    section.with_children(|vc| {
        CheckboxCardBuilder::new("First")
            .spawn(vc, theme, font, icons);

        CheckboxCardBuilder::new("Second")
            .checked(true)
            .spawn(vc, theme, font, icons);
    });
}
