use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_box_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Box", theme, font);

    section.with_children(|vc| {
        BoxBuilder::new()
            .padding(UiRect::all(Val::Px(theme.layout.padding.base)))
            .content(|p, theme, font| {
                p.spawn((
                    Text::new("Content inside a box"),
                    TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                    TextLayout::default(),
                    TextColor(theme.color.gray.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
