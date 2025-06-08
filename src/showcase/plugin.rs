use bevy::prelude::*;

use crate::plugin::UiState;
use crate::prelude::handle_button_release;
use crate::prelude::ButtonClickedEvent; // Falls du eine globale UI-State-Machine nutzt

use super::events::*;
use super::helpers::*;
use super::sidebar_layout; // <-- neues Modul
use super::systems::*;

/// Öffnen / Schließen des Showcase
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ShowcaseState {
    #[default]
    Closed,
    Open,
}

/// Registriert Events + Systeme
pub struct ShowcasePlugin;
impl Plugin for ShowcasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ShowcaseState>()
            // Events ---------------------------------------------------------
            .add_event::<ToggleShowcaseEvent>()
            .add_event::<ButtonClickedEvent<ShowcaseAction>>()
            // Start-Setup (wird genau einmal aufgerufen) ----------------------
            .add_systems(
                Startup,
                sidebar_layout::setup_ui.run_if(in_state(UiState::Ready)),
            )
            // Laufende Systeme -----------------------------------------------
            .add_systems(
                Update,
                (
                    // 1) Taste F12/Esc  → Toggle-Event feuern
                    handle_toggle_system.run_if(in_state(UiState::Ready)),
                    // 2) Toggle-Event → State-Wechsel
                    toggle_showcase_ui_system.run_if(in_state(UiState::Ready)),
                    // 3) State-Change → UI spawnen / despawnen
                    apply_showcase_ui_state_system.run_if(in_state(UiState::Ready)),
                    // 4) Sidebar-Buttons → Inhalt rechts tauschen
                    sidebar_layout::handle_select_element
                        .run_if(in_state(ShowcaseState::Open))
                        .run_if(in_state(UiState::Ready)),
                    // 5) Button-Release Helper (optional)
                    handle_button_release::<ShowcaseAction>.run_if(in_state(ShowcaseState::Open)),
                ),
            );
    }
}
