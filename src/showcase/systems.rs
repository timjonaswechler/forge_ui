use crate::prelude::*;
use bevy::prelude::*;

use super::events::*;
use super::helpers::*;
use super::plugin::ShowcaseState;
use super::sidebar_layout::*;
use crate::assets::FontAssets;

/// ⌨️  F12 / Esc → Toggle-Event erzeugen
pub fn handle_toggle_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_toggle: EventWriter<ToggleShowcaseEvent>,
    mut bt_events: EventReader<ButtonClickedEvent<ShowcaseAction>>,
) {
    if keys.just_pressed(KeyCode::F12) || keys.just_pressed(KeyCode::Escape) {
        ev_toggle.write(ToggleShowcaseEvent);
    }
    for ev in bt_events.read() {
        if ev.action_id == Some(ShowcaseAction::Toggle) {
            ev_toggle.write(ToggleShowcaseEvent);
        }
    }
}

/// 🔄  Toggle-Event in Open/Closed-State umsetzen
pub fn toggle_showcase_ui_system(
    mut ev_toggle: EventReader<ToggleShowcaseEvent>,
    state: Res<State<ShowcaseState>>,
    mut next_state: ResMut<NextState<ShowcaseState>>,
) {
    if ev_toggle.is_empty() {
        return;
    }

    match *state.get() {
        ShowcaseState::Closed => next_state.set(ShowcaseState::Open),
        ShowcaseState::Open => next_state.set(ShowcaseState::Closed),
    }
    ev_toggle.clear();
}

/// 🖼️  UI spawnen bzw. entfernen
pub fn apply_showcase_ui_state_system(
    mut commands: Commands,
    state: Res<State<ShowcaseState>>,
    theme: Res<UiTheme>,
    font: Res<FontAssets>,
    query: Query<Entity, With<ShowcaseMarker>>,
) {
    match *state.get() {
        ShowcaseState::Open if query.is_empty() => {
            // Die setup_ui-Funktion direkt aufrufen
            setup_ui(commands, theme, font);
        }
        ShowcaseState::Closed if !query.is_empty() => {
            for entity in &query {
                // Entity mit allen Kindern despawnen
                commands.entity(entity).despawn();
            }
        }
        _ => {}
    }
}
