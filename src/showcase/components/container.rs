use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_container_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Container", theme, font);
    section.with_children(|vc| {
        let _ = ContainerBuilder::new()
            .width(Val::Px(400.0))
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Content inside container"),
                    TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(t.color.gray.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
