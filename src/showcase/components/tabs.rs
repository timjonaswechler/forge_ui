use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_tabs_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Tabs", theme, font);
    section.with_children(|vc| {
        TabsBuilder::new()
            .add_tab(TabDefinition::new("Tab One", "one").content(|p, t, f| {
                p.spawn((
                    Text::new("Content One"),
                    TextFont {
                        font: f.clone(),
                        font_size: t.font.size.base,
                        ..default()
                    },
                    TextColor(t.color.slate.step12),
                ));
            }))
            .add_tab(TabDefinition::new("Tab Two", "two").content(|p, t, f| {
                p.spawn((
                    Text::new("Content Two"),
                    TextFont {
                        font: f.clone(),
                        font_size: t.font.size.base,
                        ..default()
                    },
                    TextColor(t.color.slate.step12),
                ));
            }))
            .spawn(vc, theme, font);
    });
}
