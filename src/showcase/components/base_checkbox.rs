use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_base_checkbox_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Base Checkbox", theme, font);
    section.with_children(|vc| {
        BaseCheckboxBuilder::new()
            .content(|cb, t, f| {
                cb.spawn((
                    Text::new("✓"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}

