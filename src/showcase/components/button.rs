use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_button_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut variants_section = create_variant_section(parent, "Button Variants", &theme, font);

    variants_section.with_children(|vc| {
        // Standard Buttons in verschiedenen Varianten
        ButtonBuilder::new()
            .text("Solid")
            .variant(ButtonVariant::Solid)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Soft")
            .variant(ButtonVariant::Soft)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Outline")
            .variant(ButtonVariant::Outline)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Ghost")
            .variant(ButtonVariant::Ghost)
            .spawn(vc, &theme, font);
    });

    // Button Sizes
    let mut sizes_section = create_variant_section(parent, "Button Sizes", &theme, font);

    sizes_section.with_children(|vc| {
        ButtonBuilder::new()
            .text("Small")
            .size(ButtonSize::Small)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Medium")
            .size(ButtonSize::Default)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Large")
            .size(ButtonSize::Large)
            .spawn(vc, &theme, font);
    });

    // Disabled Buttons
    let mut disabled_section = create_variant_section(parent, "Disabled Buttons", &theme, font);

    disabled_section.with_children(|vc| {
        ButtonBuilder::new()
            .text("Disabled Primary")
            .variant(ButtonVariant::Solid)
            .disabled(true)
            .spawn(vc, &theme, font);

        ButtonBuilder::new()
            .text("Disabled Secondary")
            .variant(ButtonVariant::Soft)
            .disabled(true)
            .spawn(vc, &theme, font);
    });
}
