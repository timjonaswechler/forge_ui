use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_base_dialog_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Base Dialog", theme, font);
    section.with_children(|vc| {
        BaseDialogBuilder::new()
            .content(|cb, t, f| {
                cb.spawn((
                    Text::new("Dialog body"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
