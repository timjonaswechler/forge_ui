use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_context_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Context", theme, font);
    section.with_children(|vc| {
        ContextProviderBuilder::new(String::from("Provided value"))
            .content(|p, value, t, f| {
                p.spawn((
                    Text::new(value.clone()),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.gray.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
