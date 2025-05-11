use crate::components::checkbox::components::{CheckboxMarker, CheckboxState, CheckmarkIconEntity};
use crate::components::checkbox::events::CheckboxChangedEvent;
use crate::theme::UiTheme;
use bevy::prelude::*;

/// Aktualisiert das visuelle Erscheinungsbild aller Checkboxen.
///
/// - Reagiert auf Änderungen an `Interaction` (Hover/Press) und `CheckboxState`.
/// - Passt Hintergrund- und Randfarben basierend auf `UiTheme` an.
///
/// # Warnung
/// Überspringt das Update, wenn `UiTheme` nicht verfügbar ist.
pub fn update_checkbox_visuals(
    theme_opt: Option<Res<UiTheme>>,
    mut checkbox_query: Query<
        (
            &Interaction,
            &CheckboxState,
            &mut BackgroundColor,
            &mut BorderColor,
            // &CheckmarkIconEntity, // Zum Holen der Icon-Entität
        ),
        (
            Or<(Changed<Interaction>, Changed<CheckboxState>)>,
            With<CheckboxMarker>,
        ), // Reaktion auf Klick oder Statusänderung
    >,
) {
    // --- ADD THIS GUARD ---
    let Some(theme) = theme_opt else {
        warn!("UiTheme not available for update_checkbox_visuals, skipping frame.");
        return;
    };
    // --- END GUARD ---
    for (interaction, state, mut bg_color, mut border_color) in checkbox_query.iter_mut() {
        // Basis-Styling basierend auf checked/disabled
        let base_bg_color = theme.color.gray.step02; // Oder theme.input / theme.background ?
        let base_border_color = theme.color.gray.step06; // Oder theme.input / theme.border ?

        // Endgültige Farben (Disabled und Hover/Pressed)
        if state.disabled {
            *bg_color = BackgroundColor(base_bg_color.with_alpha(0.1));
            *border_color = BorderColor(base_border_color.with_alpha(0.1));
        } else {
            // Einfacher Hover-Effekt (optional)
            let hover_factor = 0.1;
            let pressed_factor = 0.1;

            *bg_color = match *interaction {
                Interaction::Hovered => BackgroundColor(base_bg_color.lighter(hover_factor)),
                Interaction::Pressed => BackgroundColor(base_bg_color.darker(-pressed_factor)),
                Interaction::None => BackgroundColor(base_bg_color),
            };
            *border_color = match *interaction {
                Interaction::Hovered => BorderColor(base_border_color.lighter(hover_factor)),
                Interaction::Pressed => BorderColor(base_border_color.darker(-pressed_factor)),
                Interaction::None => BorderColor(base_border_color),
            };
        }
    }
}

/// Handhabt Klick-Interaktionen auf Checkboxen.
///
/// - Schaltet den `checked`-Status um bei `Pressed` und `!disabled`.
/// - Sendet `CheckboxChangedEvent` bei Statusänderung.
pub fn handle_checkbox_clicks(
    mut checkbox_query: Query<
        (Entity, &Interaction, &mut CheckboxState),
        (Changed<Interaction>, With<CheckboxMarker>),
    >,
    mut ev_checkbox_changed: EventWriter<CheckboxChangedEvent>,
    // Wir brauchen hier jetzt KEINE icon_visibility_query mehr,
    // das Aussehen wird vom update_checkbox_visuals System gesteuert,
    // sobald sich der CheckboxState geändert hat (im nächsten Frame).
) {
    for (entity, interaction, mut state) in checkbox_query.iter_mut() {
        // Umschalten bei Klick (Released = Interaction geht von Pressed weg)
        if *interaction == Interaction::Pressed && !state.disabled {
            // Den Zustand direkt hier umschalten
            state.checked = !state.checked;
            info!("Checkbox {:?} toggled to {}", entity, state.checked);

            // Event senden, damit andere Systeme reagieren können
            ev_checkbox_changed.write(CheckboxChangedEvent {
                checkbox_entity: entity,
                is_checked: state.checked,
            });

            // WICHTIG: Der update_checkbox_visuals muss auch reagieren,
            // wenn sich *nur* der CheckboxState ändert, nicht nur die Interaction.
            // Dies geschieht implizit, wenn der nächste Frame gerendert wird, ODER
            // man fügt Changed<CheckboxState> zur Query in update_checkbox_visuals hinzu.
        }
    }
}

/// Aktualisiert die Sichtbarkeit des Checkmark-Icons bei Änderung des Checkbox-Zustands.
///
/// - Reagiert auf `Changed<CheckboxState>` und schaltet `Visibility` des Icon-Entities.
pub fn update_checkmark_visibility_on_state_change(
    checkbox_query: Query<(&CheckboxState, &CheckmarkIconEntity), Changed<CheckboxState>>, // Nur wenn State sich ändert
    mut icon_visibility_query: Query<&mut Visibility>,
) {
    for (state, checkmark_icon) in checkbox_query.iter() {
        if let Ok(mut icon_visibility) = icon_visibility_query.get_mut(checkmark_icon.0) {
            *icon_visibility = if state.checked {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        } else {
            error!(
                "CheckmarkIconEntity {:?} not found when state changed!",
                checkmark_icon.0
            );
        }
    }
}
