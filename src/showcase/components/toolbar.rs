use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_toolbar_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Toolbar", theme, font);
    section.with_children(|vc| {
        ToolbarBuilder::new()
            .item(|p, t, f| {
                ButtonBuilder::<NoAction>::new_for_action()
                    .text("Item 1")
                    .spawn(p, t, f);
            })
            .item(|p, t, f| {
                ButtonBuilder::<NoAction>::new_for_action()
                    .text("Item 2")
                    .spawn(p, t, f);
            })
            .item(|p, t, f| {
                let _ = SeparatorBuilder::new().orientation(SeparatorOrientation::Vertical).length(Val::Px(20.0)).spawn(p, t, f);
            })
            .item(|p, t, f| {
                ToggleGroupBuilder::<NoAction>::new_with_action_type()
                    .add_item("Bold")
                    .add_item("Italic")
                    .spawn_into(p, t);
            })
            .spawn(vc, theme, font);
    });
}
