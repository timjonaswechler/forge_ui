use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_base_menu_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Base Menu", theme, font);
    section.with_children(|vc| {
        BaseMenuBuilder::new()
            .content(|cb, t, f| {
                cb.spawn((
                    Node {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    Name::new("Item 1"),
                )).with_children(|p| {
                    p.spawn((
                        Text::new("First Item"),
                        TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                        TextColor(t.color.slate.step12),
                    ));
                });
                cb.spawn((
                    Node {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    Name::new("Item 2"),
                )).with_children(|p| {
                    p.spawn((
                        Text::new("Second Item"),
                        TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                        TextColor(t.color.slate.step12),
                    ));
                });
            })
            .spawn(vc, theme, font);
    });
}
