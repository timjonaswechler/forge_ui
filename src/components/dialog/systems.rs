use bevy::ecs::observer::TriggerTargets;
// components/dialog/systems.rs
use bevy::platform::collections::HashSet;
use bevy::prelude::*;

use crate::*;

/// System zum Öffnen eines Dialogs über Event.
pub fn open_dialog_system(
    mut commands: Commands, // Nicht mehr für Animationen, aber bleibt für ECS-Operationen
    mut ev_open: EventReader<OpenDialogEvent>,
    mut q_dialogs: Query<(
        Entity,
        &DialogConfig,
        &mut DialogState,
        &mut Visibility, // Direktes Ändern der Sichtbarkeit
    )>, // Keine Children, KeepMounted oder q_animatable_content mehr für die Kernlogik nötig
    mut active_modals: ResMut<ActiveDialogs>,
) {
    for OpenDialogEvent(id_to_open) in ev_open.read() {
        for (entity, config, mut state, mut vis) in q_dialogs.iter_mut() {
            if config.id == *id_to_open && !state.open {
                *vis = Visibility::Inherited; // Dialog sichtbar machen
                state.open = true;
                info!("Dialog {:?} ({:?}) geöffnet.", entity, config.id);
                active_modals.modals.insert(entity);
                break; // Dialog gefunden und verarbeitet
            }
        }
    }
}

/// System zum Schließen von Dialogen.
pub fn close_dialog_system(
    mut commands: Commands, // Vorerst nicht genutzt, aber könnte für zukünftige, nicht-animierte ECS-Ops nützlich sein
    mut ev_close: EventReader<CloseDialogEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_modals: ResMut<ActiveDialogs>,
    mut q_dialogs: Query<
        (
            Entity,
            &DialogConfig,
            &mut DialogState,
            &mut Visibility, // Direktes Ändern der Sichtbarkeit
        ),
        With<DialogRootMarker>, // Stelle sicher, dass wir nur Dialog-Roots bearbeiten
    >,
    // Keine q_animatable_content oder KeepMounted Query mehr
) {
    let mut ids_to_process_for_close: HashSet<DialogId> = HashSet::new();
    let mut close_all_current_modals = false;

    // Events verarbeiten, um herauszufinden, welche Dialoge geschlossen werden sollen
    for event in ev_close.read() {
        if let Some(id) = &event.id_to_close {
            ids_to_process_for_close.insert(*id);
        } else {
            close_all_current_modals = true;
        }
    }

    // ESC-Taste verarbeiten (schließt alle aktiven modalen Dialoge)
    if keyboard.just_pressed(KeyCode::Escape) && !active_modals.modals.is_empty() {
        close_all_current_modals = true;
        info!("ESC gedrückt, schließe alle modalen Dialoge.");
    }

    // Wenn alle modalen geschlossen werden sollen, deren IDs sammeln
    if close_all_current_modals {
        for modal_entity_id in active_modals.modals.iter() {
            if let Ok((_, config, _, _)) = q_dialogs.get(*modal_entity_id) {
                ids_to_process_for_close.insert(config.id);
            }
        }
    }

    if ids_to_process_for_close.is_empty() {
        return; // Nichts zu tun
    }
    info!(
        "Versuche Dialoge zu schließen. IDs: {:?}",
        ids_to_process_for_close
    );

    let mut actually_closed_entities: HashSet<Entity> = HashSet::new();

    for (entity, config, mut state, mut visibility) in q_dialogs.iter_mut() {
        if ids_to_process_for_close.contains(&config.id) && state.open {
            info!("Dialog {:?} ({:?}) wird geschlossen.", entity, config.id);
            state.open = false; // Zustand als geschlossen markieren
            *visibility = Visibility::Hidden; // DialogRootMarker sofort verstecken

            actually_closed_entities.insert(entity);
        }
    }

    // Modale Dialoge aus der ActiveDialogs-Liste entfernen, die tatsächlich geschlossen wurden
    if !actually_closed_entities.is_empty() {
        active_modals
            .modals
            .retain(|e| !actually_closed_entities.contains(e));
        info!("ActiveDialogs aktualisiert: {:?}", active_modals.modals);
    }
}

/// System, das Klicks auf das DialogOverlay abfängt (nur wenn modal).
pub fn handle_overlay_click_system(
    q_overlays: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<DialogOverlay>)>, // Interaction nur lesen -> iter()
    q_dialog_root: Query<(Entity, &DialogConfig), With<DialogRootMarker>>,
    mut ev_close: EventWriter<CloseDialogEvent>,
) {
    for (interaction, overlay_childof) in q_overlays.iter() {
        if *interaction == Interaction::Pressed {
            let root_entity = overlay_childof.parent();
            if let Ok((_dialog_entity, dialog_config)) = q_dialog_root.get(root_entity) {
                info!(
                    "Overlay für Dialog {:?} (Entity {:?}) geklickt, sende CloseDialogEvent.",
                    dialog_config.id, root_entity
                );
                ev_close.write(CloseDialogEvent::specific(dialog_config.id));
            }
        }
    }
}

/// System, das auf Klicks von DialogTrigger-Buttons lauscht (die eine `OpenDialogActionPayload` haben)
/// und ein `OpenDialogEvent` für die entsprechende DialogId sendet.
pub fn handle_dialog_action_buttons(
    mut button_events: EventReader<ButtonClickedEvent<DialogAction>>,
    mut ev_open_dialog: EventWriter<OpenDialogEvent>,
    mut ev_close_dialog: EventWriter<CloseDialogEvent>,
) {
    for event in button_events.read() {
        if let ButtonClickedEvent {
            action_id: Some(action),
            ..
        } = event
        {
            match action {
                DialogAction::Open(id) => {
                    info!(
                        "DialogAction::Open für {:?} empfangen. Sende OpenDialogEvent.",
                        id
                    );
                    ev_open_dialog.write(OpenDialogEvent(*id));
                }
                DialogAction::Close(id) => {
                    info!(
                        "DialogAction::Close für {:?} empfangen. Sende CloseDialogEvent.",
                        id
                    );
                    ev_close_dialog.write(CloseDialogEvent::specific(*id));
                }
            }
        }
    }
}
