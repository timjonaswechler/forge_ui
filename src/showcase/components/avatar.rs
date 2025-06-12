use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_avatar_examples(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    icons: &Res<IconAssets>,
) {
    let mut section = create_variant_section(parent, "Avatars", theme, font);

    let fallback_icon = icons
        .0
        .get("person")
        .expect("missing 'person' icon")
        .clone();

    section.with_children(|vc| {
        let _ = AvatarBuilder::new()
            .size(AvatarSize::Small)
            .initials("AB")
            .spawn(vc, theme, font);

        let _ = AvatarBuilder::new()
            .size(AvatarSize::Medium)
            .initials("CD")
            .spawn(vc, theme, font);

        let _ = AvatarBuilder::new()
            .size(AvatarSize::Large)
            .image(fallback_icon)
            .spawn(vc, theme, font);
    });
}
