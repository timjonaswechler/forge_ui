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

pub fn show_toggle_group_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    check_icon_handle: Handle<Image>,
    cross_icon_handle: Handle<Image>,
) {
    // Horizontal Toggle Group
    let mut horizontal_section =
        create_variant_section(parent, "Horizontal Toggle Group", theme, font);

    // horizontal_section.with_children(|vc| {
    //     ToggleGroupBuilder::new()
    //         .orientation(ToggleGroupOrientation::Horizontal)
    //         .variant(ToggleGroupVariant::Default)
    //         .add_item("option1")
    //         .add_item("option2")
    //         .add_item("option3")
    //         .default_value("option2")
    //         .spawn_into(vc, theme);
    // });

    // // Vertical Toggle Group
    // let mut vertical_section = create_variant_section(parent, "Vertical Toggle Group", theme, font);

    // vertical_section.with_children(|vc| {
    //     ToggleGroupBuilder::new()
    //         .orientation(ToggleGroupOrientation::Vertical)
    //         .variant(ToggleGroupVariant::Default)
    //         .add_item("one")
    //         .add_item("two")
    //         .add_item("three")
    //         .default_value("one")
    //         .spawn_into(vc, theme);
    // });

    // Toggle Group Variants
    let mut variants_section = create_variant_section(parent, "Toggle Group Variants", theme, font);

    // variants_section.with_children(|vc| {
    //     ToggleGroupBuilder::new()
    //         .variant(ToggleGroupVariant::Default)
    //         .add_item("subtle1")
    //         .add_item("subtle2")
    //         .default_value("subtle1")
    //         .spawn_into(vc, theme);

    //     ToggleGroupBuilder::new()
    //         .variant(ToggleGroupVariant::Primary)
    //         .add_item("outline1")
    //         .add_item("outline2")
    //         .default_value("outline1")
    //         .spawn_into(vc, theme);

    //     ToggleGroupBuilder::new()
    //         .variant(ToggleGroupVariant::Secondary)
    //         .add_item("solid1")
    //         .add_item("solid2")
    //         .default_value("solid1")
    //         .spawn_into(vc, theme);
    // });

    // Toggle Group Types
    let mut types_section = create_variant_section(parent, "Toggle Group Types", theme, font);

    // types_section.with_children(|vc| {
    //     let mut builder_single = ToggleGroupBuilder::new();
    //     builder_single
    //         .group_type(ToggleGroupType::Single)
    //         .add_item("single1")
    //         .add_item("single2")
    //         .add_item("single3")
    //         .default_value("single1")
    //         .spawn_into(vc, theme);

    //     let mut builder_multi = ToggleGroupBuilder::new();
    //     builder_multi
    //         .group_type(ToggleGroupType::Multiple)
    //         .add_item("multi1")
    //         .add_item("multi2")
    //         .add_item("multi3")
    //         .default_values(vec!["multi1", "multi3"])
    //         .spawn_into(vc, theme);
    // });
    let mut icon_section = create_variant_section(parent, "Toggle Group with Icons", theme, font);

    // icon_section.with_children(|vc| {
    //     ToggleGroupBuilder::new()
    //         .add_item_with_icon("icon1", check_icon_handle.clone())
    //         .add_item_with_icon("icon2", cross_icon_handle.clone())
    //         .default_value("icon1")
    //         .spawn_into(vc, theme);
    // });
}
