use bevy::prelude::*;

use crate::layout::*;
use crate::theme::UiTheme;

pub(crate) fn spawn_dialog_sections<'w>(
    parent: &mut ChildSpawnerCommands<'w>,
    theme: &UiTheme,
) -> (
    Entity, /*header*/
    Entity, /*body*/
    Entity, /*footer*/
) {
    // ── Header ─────────────────────────────────
    let header = HorizontalStackBuilder::new()
        .justify(JustifyContent::SpaceBetween)
        .align(AlignItems::Center)
        .gap(Val::Px(theme.layout.gap.sm))
        .height(Val::Auto)
        .spawn(parent) // •── creates Node + returns EntityCommands
        .id();

    // ── Body ───────────────────────────────────
    let body = VerticalStackBuilder::new("Body")
        .gap(Val::Px(theme.layout.gap.base))
        .justify(JustifyContent::FlexStart)
        .align(AlignItems::Stretch)
        .spawn(parent)
        .id();

    // ── Footer ─────────────────────────────────
    let footer = HorizontalStackBuilder::new()
        .justify(JustifyContent::FlexEnd)
        .gap(Val::Px(theme.layout.gap.sm))
        .padding_rect(UiRect::top(Val::Px(theme.layout.padding.sm)))
        .spawn(parent)
        .id();

    (header, body, footer)
}
