// In src/components/button/plugin.rs (neu zu erstellen)
use crate::components::button::{
    handle_button_press, handle_button_release, update_button_visuals, ButtonClickedEvent, NoAction,
};
use crate::plugin::UiState;
use bevy::prelude::*; // Dein globaler UI-State

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickedEvent<NoAction>>().add_systems(
            Update,
            (
                // Diese Systeme werden nur für NoAction Buttons registriert.
                // Der Nutzer muss für eigene Action-Typen weiterhin
                // handle_button_press::<MyAction> und handle_button_release::<MyAction>
                // sowie add_event::<ButtonClickedEvent<MyAction>>() selbst hinzufügen.
                handle_button_press::<NoAction>,
                handle_button_release::<NoAction>,
                // update_button_visuals ist generisch und kümmert sich um alle Buttons.
                update_button_visuals,
            )
                .run_if(in_state(UiState::Ready)), // Annahme: Button-Systeme sollen nur im Ready-State laufen
        );
    }
}
