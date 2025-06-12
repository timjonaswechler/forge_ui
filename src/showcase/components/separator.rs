use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_separator_example(parent: &mut ChildSpawnerCommands, theme: &UiTheme, font: &Handle<Font>) {
    let mut section = create_variant_section(parent, "Separator", theme, font);
    section.with_children(|vc| {
        SeparatorBuilder::new().spawn(vc, theme, font);
        vc.spawn(Node { height: Val::Px(8.0), ..default() });
        SeparatorBuilder::new()
            .orientation(SeparatorOrientation::Vertical)
            .length(Val::Px(40.0))
            .spawn(vc, theme, font);
    });
}
