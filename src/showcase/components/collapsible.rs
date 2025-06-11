use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_collapsible_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Collapsible", theme, font);

    section.with_children(|vc| {
        // Closed by default
        CollapsibleBuilder::new()
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Hidden content"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);

        // Open
        CollapsibleBuilder::new()
            .open(true)
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Visible content"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
