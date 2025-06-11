use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_collection_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Collection", theme, font);
    section.with_children(|vc| {
        CollectionBuilder::new()
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Item 1"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.slate.step12),
                ));
                p.spawn((
                    Text::new("Item 2"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
