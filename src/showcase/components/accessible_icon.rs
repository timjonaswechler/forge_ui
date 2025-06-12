use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_accessible_icon_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    icons: &Res<IconAssets>,
) {
    let mut section = create_variant_section(parent, "Accessible Icon", theme, font);
    let gear = icons
        .1
        .get("gear")
        .expect("missing 'gear' icon")
        .clone();

    section.with_children(|vc| {
        AccessibleIconBuilder::new(gear, "Settings").spawn(vc, theme, font);
    });
}
