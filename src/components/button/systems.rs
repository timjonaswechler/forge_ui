use super::components::{ButtonMarker, ButtonState}; // NoAction nicht unbedingt hier nötig
use super::events::ButtonClickedEvent; // Generisches Event importieren
use super::style::get_button_style_def;
use crate::theme::UiTheme;
use bevy::prelude::*;
use std::collections::HashMap;

/// System to update the button's background, border, and children's colors
pub fn update_button_visuals(
    theme_opt: Option<Res<UiTheme>>,
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonState,
            Option<&Children>,
        ),
        (
            Or<(Changed<Interaction>, Changed<ButtonState>)>, // <<< MODIFIED Check if theme changed
            With<ButtonMarker>,
        ),
    >,
    mut text_query: Query<&mut TextColor>, // TextColor direkt ändern
    // Zugriff auf BackgroundColor der ImageBundle Node
    mut image_bg_color_query: Query<&mut BackgroundColor, (With<ImageNode>, Without<ButtonMarker>)>,
) {
    // --- ADD THIS GUARD ---
    let Some(theme) = theme_opt else {
        warn!("UiTheme not available for update_button_visuals, skipping frame.");
        return;
    };
    // --- END GUARD ---
    for (interaction, mut bg_color, mut border_color, state, children_opt) in buttons.iter_mut() {
        // 1. Update Button Background and Border using ButtonState.style_def methods
        // Diese Methoden enthalten jetzt die neue Overlay-Logik
        let style_def = get_button_style_def(state.variant, &theme);
        *bg_color = style_def.background(*interaction, state.disabled);
        *border_color = style_def.border(*interaction, state.disabled);

        // 2. Update Children Colors
        if let Some(children) = children_opt {
            // Hole die korrekte *aktuelle* Textfarbe basierend auf dem Disabled-Status
            // (Interaktion beeinflusst Textfarbe normalerweise nicht, nur Disabled)
            let child_target_color = style_def.text_color(state.disabled); // <<< MODIFIED

            for child_entity in children.iter() {
                // Text aktualisieren (direkt TextColor Komponente)
                if let Ok(mut text_color_component) = text_query.get_mut(child_entity) {
                    *text_color_component = TextColor(child_target_color);
                }

                // Icon (ImageBundle BackgroundColor) aktualisieren
                if let Ok(mut image_bg) = image_bg_color_query.get_mut(child_entity) {
                    *image_bg = BackgroundColor(child_target_color);
                }
            }
        }
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
