use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_base_tab_list_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Base Tab List", theme, font);
    section.with_children(|vc| {
        BaseTabListBuilder::new()
            .content(|cb, t, f| {
                cb.spawn((
                    Node {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    Name::new("Tab 1"),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Tab One"),
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
                    Name::new("Tab 2"),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Tab Two"),
                        TextFont { font: f.clone(), font_size: t.font.size.base, ..default() },
                        TextColor(t.color.slate.step12),
                    ));
                });
            })
            .spawn(vc, theme, font);
    });
}
