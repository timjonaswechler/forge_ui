use super::components::ShowcaseAction;
use super::events::*;
use super::systems::*;
use crate::plugin::UiState;
use crate::prelude::handle_button_release;
use crate::prelude::ButtonClickedEvent;
use bevy::prelude::*;

/// Globale UI‑State‑Machine. Vereinfacht Open/Close‑Logik.
#[derive(Component, States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ShowcaseState {
    #[default]
    Closed,
    Open,
}

#[derive(Component, Clone)]
pub enum ShowcaseElement {
    Button,
    Checkbox,
    ToggleSwitch,
    RadioGroup,
    // … jederzeit erweiterbar
}

/// Plugin, das alle Events & Systeme registriert.
pub struct ShowcasePlugin;
impl Plugin for ShowcasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ShowcaseState>()
            // Events
            .add_event::<ToggleShowcaseEvent>()
            .add_event::<ButtonClickedEvent<ShowcaseAction>>()
            // Systeme
            .add_systems(
                Update,
                (
                    // 1️⃣ Tastatureingabe → Toggle‑Event feuern
                    handle_toggle_system.run_if(in_state(UiState::Ready)),
                    // 2️⃣ Events in State‑Changes übersetzen
                    toggle_showcase_ui_system.run_if(in_state(UiState::Ready)),
                    // 3️⃣ UI je nach State spawnen bzw. despawnen
                    apply_showcase_ui_state_system.run_if(in_state(UiState::Ready)),
                    handle_button_release::<ShowcaseAction>.run_if(in_state(ShowcaseState::Open)),
                ),
            );
    }
}
