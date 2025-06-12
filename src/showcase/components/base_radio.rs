use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_base_radio_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Base Radio", theme, font);
    section.with_children(|vc| {
        BaseRadioBuilder::new()
            .content(|cb, t, f| {
                cb.spawn((
                    Text::new("●"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
