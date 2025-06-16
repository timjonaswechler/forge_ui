use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_aspect_ratio_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Aspect Ratio", theme, font);
    section.with_children(|vc| {
        let _ = AspectRatioBuilder::new(16.0 / 9.0)
            .content(|p, t, _| {
                p.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(t.color.indigo.step05),
                ));
            })
            .spawn(vc, theme, font);
    });
}
