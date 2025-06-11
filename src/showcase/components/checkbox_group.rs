use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_checkbox_group_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    icons: &Res<IconAssets>,
) {
    // Vertical group example
    let mut vertical_section = create_variant_section(parent, "Vertical Checkbox Group", theme, font);

    vertical_section.with_children(|vc| {
        CheckboxGroupBuilder::new()
            .orientation(CheckboxGroupOrientation::Vertical)
            .option("one", "Option One")
            .option("two", "Option Two")
            .option("three", "Option Three")
            .checked("two")
            .spawn(vc, theme, font, icons);
    });

    // Horizontal group example
    let mut horizontal_section = create_variant_section(parent, "Horizontal Checkbox Group", theme, font);
    horizontal_section.with_children(|vc| {
        CheckboxGroupBuilder::new()
            .orientation(CheckboxGroupOrientation::Horizontal)
            .option("a", "Alpha")
            .option("b", "Beta")
            .checked("a")
            .spawn(vc, theme, font, icons);
    });
}
