use crate::theme::UiTheme;
use bevy::prelude::*;

use crate::{
    SwitchChangedEvent, SwitchMarker, SwitchOverlayMarker, SwitchState, SwitchThumbMarker,
    SwitchTrackColor,
};

/// Update-System passt Thumb & Overlay an
pub fn update_toggle_switch_visuals(
    theme_opt: Option<Res<UiTheme>>,
    mut params: ParamSet<(
        Query<
            (
                Entity,
                &SwitchState,
                &Children,
                &mut Node,
                &mut BackgroundColor,
                Option<&SwitchTrackColor>,
            ),
            Changed<SwitchState>,
        >,
        Query<&mut BackgroundColor, With<SwitchThumbMarker>>,
        Query<&mut Visibility, With<SwitchOverlayMarker>>,
    )>,
) {
    let theme = if let Some(t) = theme_opt {
        t
    } else {
        return;
    };

    // 1️⃣ Phase: Track-Query ausführen und Child-Entitäten sammeln
    let mut thumb_entities: Vec<Entity> = Vec::new();
    let mut overlay_entities: Vec<(Entity, bool)> = Vec::new();

    for (_track_e, state, children, mut node, mut bg, track_color) in params.p0().iter_mut() {
        // Track-Farbe & Position updaten
        let color = if state.checked {
            track_color
                .and_then(|c| c.0) // hier unwrappt Option<Option<Color>>
                .unwrap_or(theme.accent_color.step09)
        } else {
            theme.color.gray.step05
        };
        *bg = BackgroundColor(color);
        node.justify_content = if state.checked {
            JustifyContent::FlexEnd
        } else {
            JustifyContent::FlexStart
        };

        // Für jeden Child merken, ob Overlay sichtbar sein muss und Daumen aktualisiert werden muss
        for child in children.iter() {
            overlay_entities.push((child, state.disabled));
            thumb_entities.push(child);
        }
    } // <-- hier endet der p0()-Borrow

    // 2️⃣ Phase: Overlay-Visibility setzen
    for (child, disabled) in overlay_entities {
        if let Ok(mut vis) = params.p2().get_mut(child) {
            *vis = if disabled {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }

    // 3️⃣ Phase: Daumen-Farbe setzen
    for child in thumb_entities {
        if let Ok(mut thumb_bg) = params.p1().get_mut(child) {
            *thumb_bg = theme.color.white.step12.into();
        }
    }
}

/// Klick-Handler toggelt Zustand
pub fn handle_toggle_switch_clicks(
    mut q: Query<
        (Entity, &Interaction, &mut SwitchState),
        (Changed<Interaction>, With<SwitchMarker>),
    >,
    mut ev: EventWriter<SwitchChangedEvent>,
) {
    for (entity, int, mut state) in q.iter_mut() {
        if *int == Interaction::Pressed && !state.disabled {
            state.checked = !state.checked;
            ev.write(SwitchChangedEvent {
                switch_entity: entity,
                is_checked: state.checked,
            });
        }
    }
}
