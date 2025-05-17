use super::components::{ButtonMarker, ButtonState}; // NoAction nicht unbedingt hier nötig
use super::events::ButtonClickedEvent; // Generisches Event importieren
use super::style::ButtonStyle;
use bevy::prelude::*;
use std::collections::HashMap;

/// System to update the button's background, border, and children's colors
pub fn update_button_visuals(
    mut query: Query<
        (
            &ButtonState,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<ButtonMarker>,
    >,
) {
    for (state, interaction, mut bg, mut border) in query.iter_mut() {
        *bg = ButtonStyle::background(&state.color_palette, state.variant, *interaction);

        *border = ButtonStyle::border(&state.color_palette, state.variant, *interaction);
    }
}

/// System, das Button-Klicks erkennt und ein generisches `ButtonClickedEvent<A>` sendet.
pub fn handle_button_press<A: Component + Clone>(
    mut writer: EventWriter<ButtonClickedEvent<A>>,
    // Nur Buttons, deren Interaction sich geändert hat:
    query: Query<(Entity, &Interaction, &A), (With<Button>, Changed<Interaction>)>,
) {
    for (entity, interaction, action) in query.iter() {
        if *interaction == Interaction::Pressed {
            writer.write(ButtonClickedEvent {
                source_entity: entity,
                action_id: Some(action.clone()),
            });
        }
    }
}

pub fn handle_button_release<A: Component + Clone>(
    mut writer: EventWriter<ButtonClickedEvent<A>>,
    mut prev: Local<HashMap<Entity, Interaction>>,
    query: Query<(Entity, &Interaction, &A), With<Button>>,
) {
    for (entity, interaction, action) in query.iter() {
        let last = *prev.get(&entity).unwrap_or(&Interaction::None);
        // Auslösen, wenn zuvor Pressed und jetzt Released (Hovered oder None)
        if last == Interaction::Pressed
            && (*interaction == Interaction::Hovered || *interaction == Interaction::None)
        {
            writer.write(ButtonClickedEvent {
                source_entity: entity,
                action_id: Some(action.clone()),
            });
        }
        prev.insert(entity, *interaction);
    }
}
