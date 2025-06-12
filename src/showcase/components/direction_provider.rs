use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_direction_provider_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Direction Provider", theme, font);
    section.with_children(|vc| {
        DirectionProviderBuilder::new(ReadingDirection::Rtl)
            .content(|p, dir, t, f| {
                p.spawn((
                    Text::new(format!("Direction: {:?}", dir)),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
