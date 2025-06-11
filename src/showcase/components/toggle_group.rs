use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

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

    horizontal_section.with_children(|vc| {
        ToggleGroupBuilder::new()
            .orientation(ToggleGroupOrientation::Horizontal)
            .add_item("option1")
            .add_item("option2")
            .add_item("option3")
            .default_value("option2")
            .spawn_into(vc, theme);
    });

    // Vertical Toggle Group
    let mut vertical_section =
        create_variant_section(parent, "Vertical Toggle Group", theme, font);

    vertical_section.with_children(|vc| {
        ToggleGroupBuilder::new()
            .orientation(ToggleGroupOrientation::Vertical)
            .add_item("one")
            .add_item("two")
            .add_item("three")
            .default_value("one")
            .spawn_into(vc, theme);
    });

    // Toggle Group Variants
    let mut variants_section = create_variant_section(parent, "Toggle Group Variants", theme, font);

    variants_section.with_children(|vc| {
        ToggleGroupBuilder::new()
            .variant(ToggleGroupVariant::Default)
            .add_item("subtle1")
            .add_item("subtle2")
            .default_value("subtle1")
            .spawn_into(vc, theme);

        ToggleGroupBuilder::new()
            .variant(ToggleGroupVariant::Primary)
            .add_item("outline1")
            .add_item("outline2")
            .default_value("outline1")
            .spawn_into(vc, theme);

        ToggleGroupBuilder::new()
            .variant(ToggleGroupVariant::Secondary)
            .add_item("solid1")
            .add_item("solid2")
            .default_value("solid1")
            .spawn_into(vc, theme);
    });

    // Toggle Group Types
    let mut types_section = create_variant_section(parent, "Toggle Group Types", theme, font);

    types_section.with_children(|vc| {
        ToggleGroupBuilder::new()
            .group_type(ToggleGroupType::Single)
            .add_item("single1")
            .add_item("single2")
            .add_item("single3")
            .default_value("single1")
            .spawn_into(vc, theme);

        ToggleGroupBuilder::new()
            .group_type(ToggleGroupType::Multiple)
            .add_item("multi1")
            .add_item("multi2")
            .add_item("multi3")
            .default_values(vec!["multi1", "multi3"])
            .spawn_into(vc, theme);
    });

    // Toggle Group with Icons
    let mut icon_section = create_variant_section(parent, "Toggle Group with Icons", theme, font);

    icon_section.with_children(|vc| {
        ToggleGroupBuilder::new()
            .add_item_with_icon("icon1", check_icon_handle)
            .add_item_with_icon("icon2", cross_icon_handle)
            .default_value("icon1")
            .spawn_into(vc, theme);
    });
}
