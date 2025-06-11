use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_portal_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    portal_root: Res<ForgeUiPortalRoot>,
) {
    let mut section = create_variant_section(parent, "Portal Example", theme, font);

    section.with_children(|vc| {
        vc.spawn((
            Text::new("This text is inside the regular layout."),
            TextFont {
                font: font.clone(),
                font_size: theme.font.size.base,
                ..default()
            },
            TextColor(theme.color.slate.step12),
        ));
    });

    PortalContentBuilder::spawn_in_global_root(parent, portal_root, theme, font, |p, t, f| {
        p.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(t.color.tomato.step06),
            Name::new("Portal Overlay"),
        ))
        .with_children(|c| {
            c.spawn((
                Text::new("I'm in the portal"),
                TextFont {
                    font: f.clone(),
                    font_size: t.font.size.base,
                    ..default()
                },
                TextColor(t.color.slate.step12),
            ));
        });
    });
}

