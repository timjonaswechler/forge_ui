use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_checkbox_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    icons: &Res<IconAssets>,
) {
    let mut variants_section =
        create_variant_section(parent, "Checkbox States", theme, &theme.font.family.default);

    variants_section.with_children(|vc| {
        // Unchecked
        vc.spawn((
            Text::new("Unchecked:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        CheckboxBuilder::new()
            .checked(false)
            .spawn(vc, theme, &icons);

        // Checked
        vc.spawn((
            Text::new("Checked:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));
    });

    // Disabled Checkboxes
    let mut disabled_section = create_variant_section(
        parent,
        "Disabled Checkboxes",
        theme,
        &theme.font.family.default,
    );

    disabled_section.with_children(|vc| {
        vc.spawn((
            Text::new("Disabled Unchecked:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        CheckboxBuilder::new()
            .checked(false)
            .disabled(true)
            .spawn(vc, theme, &icons);

        vc.spawn((
            Text::new("Disabled Checked:"),
            TextFont {
                font: theme.font.family.default.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));

        CheckboxBuilder::new()
            .checked(true)
            .disabled(true)
            .spawn(vc, theme, &icons);
    });
}
