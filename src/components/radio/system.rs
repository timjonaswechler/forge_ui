// src/components/radio/systems.rs

use crate::components::radio::components::{OnSelect, Radio, RadioState};
use crate::components::radio::events::RadioSelectedEvent;
use crate::components::radio::style::get_radio_style_def;
use crate::components::UiTheme;
use bevy::prelude::*;

/// System to update visuals based on state
pub fn update_radio_visuals(
    mut query: Query<(&RadioState, &mut UiColor, &mut Transform), With<Radio>>,
    theme: Res<UiTheme>,
) {
    for (state, mut color, mut transform) in query.iter_mut() {
        let style = get_radio_style_def(state.variant, state.size, &theme);
        *color = UiColor(style.background);
        // show/hide inner indicator by scale
        // transform.scale = Vec3::splat(if state.checked { 1.0 } else { 0.0 });
    }
}

/// System to handle selection events
pub fn handle_radio_selects(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut RadioState, Option<&OnSelect>),
        With<Radio>,
    >,
    mut ev_writer: EventWriter<RadioSelectedEvent>,
) {
    for (entity, interaction, mut state, on_select) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked && !state.disabled {
            state.checked = true;
            // call callback
            if let Some(OnSelect(cb)) = on_select {
                cb(state.value.clone());
            }
            ev_writer.send(RadioSelectedEvent {
                entity,
                value: state.value.clone(),
            });
        }
    }
}
