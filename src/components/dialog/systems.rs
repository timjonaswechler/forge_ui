use bevy::ecs::observer::TriggerTargets;
// components/dialog/systems.rs
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy_tweening::{lens::*, Animator, Tween};
use std::time::Duration;

use super::components::*; // Dialog-spezifische Komponenten
use super::events::*; // Dialog-spezifische Events

// Das System setup_dialog_portal_container wird aus dem `portal`-Modul verwendet
// und hier nicht mehr benötigt. Stelle sicher, dass `portal::setup_global_portal_root`
// in deiner App aufgerufen wird (z.B. in einem Core-UI-Plugin).

/// System zum Öffnen eines Dialogs über Event.
pub fn open_dialog_system(
    mut commands: Commands,
    mut ev_open: EventReader<OpenDialogEvent>,
    mut q_dialogs: Query<(
        Entity,
        &DialogConfig,
        &mut DialogState,
        &mut Visibility,
        Option<&Children>,
    )>,
    mut active_modals: ResMut<ActiveDialogs>,
    q_animatable_content: Query<Entity, With<DialogContent>>,
) {
    for OpenDialogEvent(id_to_open) in ev_open.read() {
        // — First pass: find and open the requested dialog —
        // We’ll also remember whether it was modal and the opened entity.
        let mut just_opened: Option<(Entity, bool)> = None;

        for (entity, config, mut state, mut vis, children_opt) in q_dialogs.iter_mut() {
            if config.id == *id_to_open && !state.open {
                // open it
                *vis = Visibility::Inherited;
                state.open = true;
                info!("Opened dialog {:?} ({:?})", entity, config.id);

                // run your content‐animation code here …
                if let Some(children) = children_opt {
                    for &child in children {
                        if let Ok(content) = q_animatable_content.get(child) {
                            commands.entity(content).insert(Animator::new(Tween::new(
                                EaseFunction::QuadraticInOut,
                                Duration::from_millis(250),
                                TransformScaleLens {
                                    start: Vec3::new(0.9, 0.9, 1.0), // Leichter Skalierungseffekt
                                    end: Vec3::ONE,
                                },
                            )));
                            break;
                        }
                    }
                }

                just_opened = Some((entity, config.modal));
                break;
            }
        }

        // — Second pass: if it was modal, close any *other* active modals —
        if let Some((opened_entity, true)) = just_opened {
            // collect all the ones we need to close
            let to_close: Vec<Entity> = active_modals
                .modals
                .iter()
                .copied()
                .filter(|&e| e != opened_entity)
                .collect();

            for e in to_close.iter() {
                if let Ok((_, other_config, mut other_state, mut other_vis, _)) =
                    q_dialogs.get_mut(*e)
                {
                    if other_state.open {
                        warn!(
                            "Dialog {:?} is modal; closing other modal {:?}",
                            other_config.id, e
                        );
                        *other_vis = Visibility::Hidden;
                        other_state.open = false;
                        // optionally send a close event here…
                    }
                }
                active_modals.modals.remove(e);
            }

            active_modals.modals.insert(opened_entity);
        }
    }
}

/// System zum Schließen von Dialogen.
pub fn close_dialog_system(
    mut commands: Commands, // Für Animationen
    mut ev_close: EventReader<CloseDialogEvent>,
    q_close_triggers: Query<
        &Interaction,
        (Changed<Interaction>, With<DialogCloseTrigger>, With<Button>),
    >,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_modals: ResMut<ActiveDialogs>,
    // Query für DialogRootMarker, DialogConfig, DialogState, Visibility und KeepMounted
    mut q_dialogs: Query<
        (
            Entity,
            &DialogConfig,
            &mut DialogState,
            &mut Visibility,
            Option<&Children>,
            Option<&KeepMounted>,
        ),
        With<DialogRootMarker>,
    >,
    q_animatable_content: Query<Entity, With<DialogContent>>,
) {
    let mut ids_to_process_for_close: HashSet<DialogId> = HashSet::new();
    let mut close_all_modals = false;

    // Events verarbeiten
    for event in ev_close.read() {
        if let Some(id) = &event.id_to_close {
            ids_to_process_for_close.insert(id.clone());
        } else {
            // Schließe den/die obersten modalen Dialog(e)
            close_all_modals = true; // Flag setzen, da wir active_modals nicht iterieren und gleichzeitig modifizieren können
        }
    }

    // ESC-Taste verarbeiten (schließt alle aktiven modalen Dialoge)
    if keyboard.just_pressed(KeyCode::Escape) && !active_modals.modals.is_empty() {
        close_all_modals = true;
    }

    // CloseTrigger-Buttons verarbeiten
    // Finde heraus, zu welchem Dialog der geklickte Trigger gehört
    for interaction in q_close_triggers.iter() {
        if *interaction == Interaction::Pressed {
            // Hier bräuchten wir eine Möglichkeit, vom Trigger-Button zum DialogRoot zu gelangen.
            // Entweder über Parent-Queries oder der Trigger speichert die DialogId/DialogRootEntity.
            // Für Einfachheit nehmen wir an, dass ein Klick auf *irgendeinen* DialogCloseTrigger
            // den obersten modalen Dialog schließt, wenn `active_modals` nicht leer ist.
            // TODO: Trigger spezifischer machen (z.B. Trigger hat Komponente `ParentDialog(Entity)`).
            if !active_modals.modals.is_empty() {
                warn!(
                    "DialogCloseTrigger geklickt, versuche obersten modalen Dialog zu schließen."
                );
                close_all_modals = true; // Sicherstellen, dass dies zum Schließen führt
            }
        }
    }

    if close_all_modals {
        for modal_entity in active_modals.modals.iter() {
            if let Ok((_, config, _, _, _, _)) = q_dialogs.get(*modal_entity) {
                ids_to_process_for_close.insert(config.id.clone());
            }
        }
    }

    let mut actually_closed_modals = HashSet::new();

    for (entity, config, mut state, mut vis, children_opt, keep_mounted_opt) in q_dialogs.iter_mut()
    {
        if ids_to_process_for_close.contains(&config.id) && state.open {
            info!("Dialog {:?} ({:?}) wird geschlossen.", entity, config.id);
            state.open = false;
            let keep_mounted = if let Some(km) = keep_mounted_opt {
                km.0
            } else {
                false
            };

            for child_entity in children_opt.iter() {
                if let Ok(content_entity) =
                    q_animatable_content.get(child_entity.entities().next().unwrap())
                {
                    if keep_mounted {
                        // Start closing animation for the content
                        commands
                            .entity(content_entity)
                            .insert(Animator::new(Tween::new(
                                EaseFunction::QuadraticInOut,
                                Duration::from_millis(200), // Kürzere Ausblendanimation
                                TransformScaleLens {
                                    start: Vec3::ONE,
                                    end: Vec3::new(0.9, 0.9, 1.0), // Schrumpft leicht
                                },
                                // Optional: füge eine Tween hinzu, um Visibility nach Animation zu setzen
                                // Dies erfordert ein Event oder ein Polling-System nach der Animation
                                // Fürs Erste: Wir setzen Visibility sofort, wenn nicht `KeepMounted`
                            )));
                        // Hier wäre ein gutes TODO: Setze Visibility::Hidden NACHDEM die Animation fertig ist.
                        // Dies könnte über ein Event von bevy_tweening geschehen oder ein Delay.
                        // Temporär: Visibility wird gesetzt, nachdem die Animation *gestartet* wurde.
                        // Wenn KeepMounted, dann nicht sofort verstecken. Ein anderes System kümmert sich drum.
                    }
                }
            }
            if !keep_mounted {
                // Wenn nicht KeepMounted, sofort verstecken
                *vis = Visibility::Hidden;
            }
        } else {
            // Keine Kinder, oder keine animierbare Node gefunden
            *vis = Visibility::Hidden;
        }

        if config.modal {
            actually_closed_modals.insert(entity);
        }
    }
    if !actually_closed_modals.is_empty() {
        active_modals
            .modals
            .retain(|e| !actually_closed_modals.contains(e));
    }
}

/// System, das Klicks auf das DialogOverlay abfängt (nur wenn modal).
pub fn handle_overlay_click_system(
    // Query auf das Overlay, seinen Parent (DialogRoot) und dessen DialogConfig
    mut q_overlays: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<DialogOverlay>)>,
    q_dialog_config: Query<&DialogConfig>, // Um Modalität zu prüfen
    mut ev_close: EventWriter<CloseDialogEvent>,
) {
    for (interaction, overlay_parent) in q_overlays.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Finde die DialogConfig des Dialogs, zu dem das Overlay gehört
            if let Ok(dialog_config) = q_dialog_config.get(overlay_parent.parent()) {
                if dialog_config.modal {
                    info!(
                        "Overlay für modalen Dialog {:?} geklickt, sende CloseDialogEvent.",
                        dialog_config.id
                    );
                    // Schließe diesen spezifischen Dialog, nicht nur den "aktuellen modalen"
                    ev_close.write(CloseDialogEvent::specific(dialog_config.id.clone()));
                }
            }
        }
    }
}

/// System zum initialen Registrieren von `initially_open` Dialogen bei `ActiveDialogs`.
pub fn register_initially_open_dialogs(
    mut active_modals: ResMut<ActiveDialogs>,
    // Reagiert auf neu hinzugefügte DialogConfig-Komponenten (die den initialen Zustand tragen)
    query: Query<(Entity, &DialogConfig), Added<DialogConfig>>,
) {
    for (entity, config) in query.iter() {
        if config.initially_open && config.modal {
            // Wenn es bereits andere initially_open modale Dialoge gab, wurde der active_modals
            // Set möglicherweise von einem vorherigen Eintrag geleert.
            // Hier stellen wir sicher, dass der LETZTE gefundene als der aktive gilt.
            if !active_modals.modals.is_empty() && !active_modals.modals.contains(&entity) {
                warn!(
                    "Mehrere 'initially_open' modale Dialoge gefunden. {:?} ({:?}) wird aktiv, vorherige werden implizit ignoriert, wenn nicht schon geschlossen.",
                    entity, config.id
                );
                active_modals.modals.clear(); // Alte `initially_open` verwerfen
            }
            if active_modals.modals.insert(entity) {
                // Füge hinzu und prüfe, ob es neu war
                info!(
                    "Initialisiere modalen Dialog {:?} ({:?}) als aktiv.",
                    entity, config.id
                );
            }
        }
    }
}
