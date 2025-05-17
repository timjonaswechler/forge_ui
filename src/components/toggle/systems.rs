use bevy::prelude::*;

use super::*;

/// Behandelt `Interaction::Pressed` und toggelt den Zustand.
pub fn handle_toggle_interaction<A: Component + Clone + Send + Sync + 'static>(
    mut interactions: Query<
        (Entity, &Interaction, &mut ToggleState, Option<&A>),
        (Changed<Interaction>, With<ToggleMarker>),
    >,
    mut toggle_events: EventWriter<ToggleChangedEvent<A>>,
) {
    for (entity, interaction, mut state, action_opt) in interactions.iter_mut() {
        if *interaction == Interaction::Pressed && !state.disabled {
            state.pressed = !state.pressed;
            toggle_events.write(ToggleChangedEvent {
                source_entity: entity,
                pressed: state.pressed,
                action_id: action_opt.cloned(),
            });
        }
    }
}

/// Aktualisiert Farben auf Basis von Zustand & Interaction.
pub fn update_toggle_visuals(
    mut toggles: Query<
        (
            &ToggleState,
            &Interaction,
            &mut BackgroundColor,
            Option<&Children>,
        ),
        With<ToggleMarker>,
    >,
    theme: Res<crate::theme::UiTheme>,
    mut images: Query<&mut BackgroundColor, Without<ToggleMarker>>, // Icon‑Child‑Farben
) {
    for (state, _interaction, mut bg, children_opt) in toggles.iter_mut() {
        let style = get_toggle_style_def(&theme, state.variant, state.size);

        // Hintergrundfarbe
        *bg = if state.disabled {
            style.bg_colors.disabled.into()
        } else if state.pressed {
            style.bg_colors.on.into()
        } else {
            style.bg_colors.off.into()
        };

        // Icon‑Farbe
        if let Some(children) = children_opt {
            for &child in children {
                if let Ok(mut icon_bg) = images.get_mut(child) {
                    *icon_bg = if state.disabled {
                        style.icon_colors.disabled.into()
                    } else if state.pressed {
                        style.icon_colors.on.into()
                    } else {
                        style.icon_colors.off.into()
                    };
                }
            }
        }
    }
}
