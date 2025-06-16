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
        vc.spawn(
            ButtonBuilder::<NoAction>::new("Solid")
                .text("Solid")
                .variant(ButtonVariant::Solid)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Soft")
                .text("Soft")
                .variant(ButtonVariant::Soft)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Outline")
                .text("Outline")
                .variant(ButtonVariant::Outline)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Ghost")
                .text("Ghost")
                .variant(ButtonVariant::Ghost)
                .build(&theme, font),
        );
    });

    // Button Sizes
    let mut sizes_section = create_variant_section(parent, "Button Sizes", &theme, font);

    sizes_section.with_children(|vc| {
        vc.spawn(
            ButtonBuilder::<NoAction>::new("Small")
                .text("Small")
                .size(ButtonSize::Small)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Medium")
                .text("Medium")
                .size(ButtonSize::Default)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Large")
                .text("Large")
                .size(ButtonSize::Large)
                .build(&theme, font),
        );
    });

    // Disabled Buttons
    let mut disabled_section = create_variant_section(parent, "Disabled Buttons", &theme, font);

    disabled_section.with_children(|vc| {
        vc.spawn(
            ButtonBuilder::<NoAction>::new("Disabled Primary")
                .text("Disabled Primary")
                .variant(ButtonVariant::Solid)
                .disabled(true)
                .build(&theme, font),
        );

        vc.spawn(
            ButtonBuilder::<NoAction>::new("Disabled Secondary")
                .text("Disabled Secondary")
                .variant(ButtonVariant::Soft)
                .disabled(true)
                .build(&theme, font),
        );
    });
}
