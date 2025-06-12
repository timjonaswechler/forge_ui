use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_slot_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Slot Example", theme, font);
    section.with_children(|vc| {
        SlotBuilder::new()
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Slotted content"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
