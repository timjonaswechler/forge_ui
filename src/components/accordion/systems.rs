use bevy::prelude::*;

use super::components::*;
use super::events::AccordionToggledEvent;

/// Toggle accordion state on header click and emit [`AccordionToggledEvent`].
pub fn handle_accordion_clicks(
    mut query: Query<
        (Entity, &Interaction, &mut AccordionState),
        (With<AccordionHeaderMarker>, Changed<Interaction>),
    >,
    mut writer: EventWriter<AccordionToggledEvent>,
) {
    for (entity, interaction, mut state) in query.iter_mut() {
        info!("Handling accordion clicks");
        if *interaction == Interaction::Pressed && !state.disabled {
            info!("Toggling accordion state");
            state.open = !state.open;
            writer.write(AccordionToggledEvent {
                accordion_entity: entity,
                is_open: state.open,
            });
        }
    }
}

/// Update the body visibility when the [`AccordionState`] changes.
pub fn update_body_visibility(
    mut query: Query<(&AccordionState, &Children), Changed<AccordionState>>,
    mut vis_query: Query<&mut Visibility, With<AccordionBodyMarker>>,
) {
    for (state, children) in query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut vis) = vis_query.get_mut(child) {
                *vis = if state.open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
