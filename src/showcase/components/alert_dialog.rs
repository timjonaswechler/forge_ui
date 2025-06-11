use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_alert_dialog_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Alert Dialog", theme, font);

    section.with_children(|vc| {
        let builder = AlertDialogBuilder::new_unique("Delete item")
            .description("Are you sure you want to delete this item?");

        builder.spawn_trigger(vc, theme, font, "Open Alert");

        let mut cmds = vc.commands_mut();
        builder.spawn(&mut cmds, theme, font, None);
    });
}
