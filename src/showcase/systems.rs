use crate::components::checkbox;
use crate::prelude::*;
use crate::{assets::FontAssets, showcase::components::ShowcaseAction};
use bevy::prelude::*;

use super::{components::ShowcaseMarker, events::*, plugin::*};

/// ⌨️  Showcase‑Taste → Toggle‑Event erzeugen.
pub fn handle_toggle_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_toggle: EventWriter<ToggleShowcaseEvent>,
    mut bt_events: EventReader<ButtonClickedEvent<ShowcaseAction>>,
) {
    if keys.just_pressed(KeyCode::F12) || keys.just_pressed(KeyCode::Escape) {
        ev_toggle.write(ToggleShowcaseEvent);
    }
    for event in bt_events.read() {
        if let Some(action) = &event.action_id {
            match action {
                ShowcaseAction::Toggle => {
                    info!("Button zum Umschalten des Showcase UI geklickt!");
                    ev_toggle.write(ToggleShowcaseEvent);
                }
            }
        }
    }
}

/// 🔄  Toggle‑Event in Open/Close‑State umsetzen.
pub fn toggle_showcase_ui_system(
    mut ev_toggle: EventReader<ToggleShowcaseEvent>,
    state: Res<State<ShowcaseState>>,
    mut next_state: ResMut<NextState<ShowcaseState>>,
) {
    if ev_toggle.is_empty() {
        return;
    }

    match state.get() {
        ShowcaseState::Closed => next_state.set(ShowcaseState::Open),
        ShowcaseState::Open => next_state.set(ShowcaseState::Closed),
    }

    ev_toggle.clear();
}

/// 🖼️  UI spawnen/entfernen, sobald sich der State ändert.
pub fn apply_showcase_ui_state_system(
    mut commands: Commands,
    state: Res<State<ShowcaseState>>,
    theme: Res<UiTheme>,   // <- Passe an deine tatsächliche Theme‑Resource an
    font: Res<FontAssets>, // <- dito für Fonts
    query: Query<Entity, With<ShowcaseMarker>>, // vorhandenes UI finden
) {
    // Hinweis: `State` implementiert `Deref<Target = T>`, aber kein `is_changed()`;
    // wir prüfen daher manuell, ob das UI bereits existiert.
    match state.get() {
        ShowcaseState::Open if query.is_empty() => {
            spawn_showcase_ui(&mut commands, &theme, &font);
        }
        ShowcaseState::Closed if !query.is_empty() => {
            for ent in &query {
                commands.entity(ent).despawn();
            }
        }
        _ => {}
    }
}

fn spawn_showcase_ui(commands: &mut Commands, theme: &UiTheme, font: &FontAssets) {
    use crate::prelude::*; // Falls du Forge‑UI nutzt

    let root = UiRoot::spawn(commands, theme);

    commands
        .entity(root)
        .insert(ShowcaseMarker)
        .with_children(|parent| {
            // let _ = VerticalStackBuilder::new()
            //     .gap(Val::Px(theme.layout.gap.base))
            //     .add_entity(
            //         LabelBuilder::new("Debug-Panel")
            //             .align(JustifyText::Center)
            //             .spawn(parent, theme, &font.default),
            //     )
            //     .add_entity(
            //         ButtonBuilder::<ShowcaseAction>::new_for_action()
            //             .text("Schließen")
            //             .action(ShowcaseAction::Toggle)
            //             .spawn(parent, theme, &font.default)
            //             .id(),
            //     );
            // sidebar
            let _ = VerticalStackBuilder::new()
                .position_type(PositionType::Absolute)
                .top(Val::Vh(0.0))
                .left(Val::Vw(0.0))
                .gap(Val::Px(theme.layout.gap.sm))
                .background(theme.color.green.step10)
                .add_entity(
                    ButtonBuilder::<ShowcaseAction>::new_for_action()
                        .text("Toggle")
                        .variant(ButtonVariant::Ghost)
                        .action(ShowcaseAction::Toggle)
                        .spawn(parent, theme, &font.default)
                        .id(),
                )
                .add_entity(
                    ButtonBuilder::<ShowcaseAction>::new_for_action()
                        .text("Button 2")
                        .action(ShowcaseAction::Toggle)
                        .spawn(parent, theme, &font.default)
                        .id(),
                )
                .spawn(parent);
        });
}
