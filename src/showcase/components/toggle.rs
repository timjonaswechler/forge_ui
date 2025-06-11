use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_toggle_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut variants_section = create_variant_section(parent, "Toggle Variants", theme, font);

    variants_section.with_children(|vc| {
        // Default Toggle
        vc.spawn((
            Text::new("Default Toggle:"),
            TextFont {
                font: font.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        ToggleBuilder::<NoAction>::new()
            .pressed(false)
            .spawn_into(vc, theme);

        // Pressed Toggle
        vc.spawn((
            Text::new("Pressed Toggle:"),
            TextFont {
                font: font.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        ToggleBuilder::<NoAction>::new()
            .pressed(true)
            .spawn_into(vc, theme);
    });

    // Toggle Sizes
    let mut sizes_section = create_variant_section(parent, "Toggle Sizes", theme, font);

    sizes_section.with_children(|vc| {
        ToggleBuilder::<NoAction>::new()
            .size(ToggleSize::Small)
            .spawn_into(vc, theme);

        ToggleBuilder::<NoAction>::new()
            .size(ToggleSize::Medium)
            .spawn_into(vc, theme);

        ToggleBuilder::<NoAction>::new()
            .size(ToggleSize::Large)
            .spawn_into(vc, theme);
    });

    // Toggle Variants
    let mut types_section = create_variant_section(parent, "Toggle Styles", theme, font);

    types_section.with_children(|vc| {
        ToggleBuilder::<NoAction>::new()
            .variant(ToggleVariant::Default)
            .pressed(true)
            .spawn_into(vc, theme);

        ToggleBuilder::<NoAction>::new()
            .variant(ToggleVariant::Primary)
            .pressed(true)
            .spawn_into(vc, theme);

        ToggleBuilder::<NoAction>::new()
            .variant(ToggleVariant::Secondary)
            .pressed(true)
            .spawn_into(vc, theme);
    });

    // Disabled Toggles
    let mut disabled_section = create_variant_section(parent, "Disabled Toggles", theme, font);

    disabled_section.with_children(|vc| {
        ToggleBuilder::<NoAction>::new()
            .pressed(false)
            .disabled(true)
            .spawn_into(vc, theme);

        ToggleBuilder::<NoAction>::new()
            .pressed(true)
            .disabled(true)
            .spawn_into(vc, theme);
    });
}

