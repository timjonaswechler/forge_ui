use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_radio_group_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    // Vertical Radio Group
    let mut vertical_section = create_variant_section(parent, "Vertical Radio Group", theme, font);

    vertical_section.with_children(|vc| {
        let _ = RadioGroupBuilder::new("vertical-demo")
            .orientation(RadioGroupOrientation::Vertical)
            .option("one", "Option One")
            .option("two", "Option Two")
            .option("three", "Option Three")
            .selected("one")
            .spawn(vc, theme, font);
    });

    // Horizontal Radio Group
    let mut horizontal_section =
        create_variant_section(parent, "Horizontal Radio Group", theme, font);

    horizontal_section.with_children(|vc| {
        let _ = RadioGroupBuilder::new("horizontal-demo")
            .orientation(RadioGroupOrientation::Horizontal)
            .option("a", "Option A")
            .option("b", "Option B")
            .option("c", "Option C")
            .selected("b")
            .spawn(vc, theme, font);
    });

    // Disabled Radio Group
    let mut disabled_section = create_variant_section(parent, "Disabled Radio Group", theme, font);

    disabled_section.with_children(|vc| {
        let _ = RadioGroupBuilder::new("disabled-demo")
            .orientation(RadioGroupOrientation::Vertical)
            .option("yes", "Yes")
            .option("no", "No")
            .option("maybe", "Maybe")
            .selected("maybe")
            .disabled(true)
            .spawn(vc, theme, font);
    });
}
