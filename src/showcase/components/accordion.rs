use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_accordion_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "Accordions", theme, font);

    section.with_children(|vc| {
        AccordionBuilder::new("Closed")
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Content"),
                    TextFont {
                        font: f.clone(),
                        font_size: t.font.size.base,
                        ..default()
                    },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);

        AccordionBuilder::new("Open")
            .open(true)
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Content"),
                    TextFont {
                        font: f.clone(),
                        font_size: t.font.size.base,
                        ..default()
                    },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);

        AccordionBuilder::new("Disabled")
            .disabled(true)
            .content(|p, t, f| {
                p.spawn((
                    Text::new("Content"),
                    TextFont {
                        font: f.clone(),
                        font_size: t.font.size.base,
                        ..default()
                    },
                    TextColor(t.color.slate.step12),
                ));
            })
            .spawn(vc, theme, font);
    });
}
