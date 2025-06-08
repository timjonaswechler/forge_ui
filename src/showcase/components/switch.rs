use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_switch_variants(parent: &mut ChildSpawnerCommands, theme: &UiTheme) {
    let mut variants_section =
        create_variant_section(parent, "Switch States", theme, &theme.font.family.default);

    variants_section.with_children(|vc| {
        // Off
        vc.spawn((
            Text::new("Off:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        SwitchBuilder::new().checked(false).spawn(vc, theme);

        // On
        vc.spawn((
            Text::new("On:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        SwitchBuilder::new().checked(true).spawn(vc, theme);
    });

    // Disabled Switches
    let mut disabled_section = create_variant_section(
        parent,
        "Disabled Switches",
        theme,
        &theme.font.family.default,
    );

    disabled_section.with_children(|vc| {
        vc.spawn((
            Text::new("Disabled Off:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        SwitchBuilder::new()
            .checked(false)
            .disabled(true)
            .spawn(vc, theme);

        vc.spawn((
            Text::new("Disabled On:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        SwitchBuilder::new()
            .checked(true)
            .disabled(true)
            .spawn(vc, theme);
    });

    // Custom Switches
    let mut custom_section =
        create_variant_section(parent, "Custom Switches", theme, &theme.font.family.default);

    custom_section.with_children(|vc| {
        SwitchBuilder::new()
            .with_radius(8.0)
            .with_color(theme.color.amber.step09)
            .checked(true)
            .spawn(vc, theme);

        SwitchBuilder::new()
            .with_radius(12.0)
            .with_color(theme.color.crimson.step09)
            .checked(true)
            .spawn(vc, theme);
    });
}
