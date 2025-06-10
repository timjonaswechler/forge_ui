use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_badge_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    icons: &Res<IconAssets>,
) {
    let check_icon_handle = icons
        .0
        .get("check")
        .expect("missing 'check' icon")
        .clone();
    let cross_icon_handle = icons
        .0
        .get("cross_1")
        .expect("missing 'cross_1' icon")
        .clone();
    let mut variants_section = create_variant_section(parent, "Badge Variants", theme, font);

    variants_section.with_children(|vc| {
        BadgeBuilder::new("Default").spawn(vc, theme, font);

        BadgeBuilder::new("Primary")
            .variant(BadgeVariant::Default)
            .spawn(vc, theme, font);

        BadgeBuilder::new("Secondary")
            .variant(BadgeVariant::Secondary)
            .spawn(vc, theme, font);

        BadgeBuilder::new("Outline")
            .variant(BadgeVariant::Outline)
            .spawn(vc, theme, font);

        BadgeBuilder::new("Destructive")
            .variant(BadgeVariant::Destructive)
            .spawn(vc, theme, font);
    });

    // Badges with Icons
    let mut icon_section = create_variant_section(parent, "Badges with Icons", theme, font);

    icon_section.with_children(|vc| {
        BadgeBuilder::new("Leading Icon")
            .leading_icon(check_icon_handle.clone())
            .spawn(vc, theme, font);

        BadgeBuilder::new("Trailing Icon")
            .trailing_icon(cross_icon_handle.clone())
            .spawn(vc, &theme, font);

        BadgeBuilder::new("Both Icons")
            .leading_icon(check_icon_handle.clone())
            .trailing_icon(cross_icon_handle.clone())
            .spawn(vc, theme, font);
    });
}
