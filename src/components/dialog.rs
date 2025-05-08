use std::time::Duration;

// crates/forge_ui/src/dialog.rs
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use uuid::Uuid;

use crate::components::button::{ButtonBuilder, ButtonSize, ButtonVariant};
use crate::theme::UiTheme; // Für Schließen-Button

// --- Ressourcen (ggf. global definieren) ---

/// In deiner Datei ganz oben (oder in einem eigenen Modul):
#[derive(Resource, Debug, Clone)]
pub struct DialogPortalContainer(pub Entity);

impl Default for DialogPortalContainer {
    fn default() -> Self {
        DialogPortalContainer(Entity::from_raw(0))
    }
}

#[derive(Resource, Default)]
pub struct ActiveDialogs {
    pub modals: HashSet<Entity>,
}

#[derive(Bundle, Clone, Default)]
pub struct DialogContentStyle {
    pub node: Node,
    pub border_radius: BorderRadius,
    pub transform: Transform,
    pub background_color: BackgroundColor,
    // Weitere Style-Komponenten nach Bedarf
}
// --- Komponenten ---

#[derive(Component)]
pub struct KeepMounted(pub bool);

#[derive(Component)]
pub struct DialogState {
    pub open: bool,
    pub default_open: bool,
}

/// Eindeutige ID für eine Dialog-Instanz.
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogId(pub Uuid);

impl DialogId {
    pub fn new_unique() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Marker für den **Wurzelknoten** eines Dialogs. Dieser Knoten wird sichtbar/unsichtbar geschaltet.
/// Enthält die eindeutige ID.
#[derive(Component, Debug, Clone)]
pub struct DialogRoot {
    pub id: DialogId,
    pub initially_open: bool, // Startet der Dialog offen?
    pub modal: bool,          // Blockiert er die UI dahinter?
}

/// Marker für das Overlay (der halbtransparente Hintergrund).
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogOverlay;

/// Marker für den sichtbaren Content-Bereich des Dialogs.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogContent;

/// Marker für einen Button, der den aktuellen Dialog schließen soll.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogCloseTrigger;

// Optional: Titel und Beschreibung könnten auch eigene Marker bekommen,
// aber oft reicht es, sie als Text im Content zu platzieren.
// #[derive(Component, Default)] pub struct DialogTitle;
// #[derive(Component, Default)] pub struct DialogDescription;

// --- Events ---

/// Event zum Öffnen eines Dialogs über seine ID.
#[derive(Event, Debug, Clone)]
pub struct OpenDialogEvent(pub DialogId);

/// Event zum Schließen des aktuell offenen modalen Dialogs (oder eines spezifischen?).
#[derive(Event, Debug, Clone)]
pub struct CloseDialogEvent {
    // Optional: ID zum gezielten Schließen (falls nicht-modal erlaubt wird)
    // pub id_to_close: Option<DialogId>
}

// --- Builder ---

// Typ für die Funktion, die den Dialog-Inhalt baut
type DialogContentBuilderFn =
    Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>;

pub struct DialogBuilder {
    id: DialogId,
    modal: bool,
    initially_open: bool,
    controlled_open: Option<bool>,
    default_open: bool,
    portal_container: Option<Entity>,
    animate: bool,
    on_open_change: Option<Box<dyn Fn(DialogState) + Send + Sync>>,
    content_builder: Option<DialogContentBuilderFn>,
    // Styling-Optionen?
    width: Option<Val>,
    height: Option<Val>,
    overlay_color: Option<Color>,
    // Titel/Beschreibung optional als direkte Strings im Builder?
    title: Option<String>,
    description: Option<String>,
    // Standardmäßig einen Schließen-Button hinzufügen?
    show_default_close_button: bool,
}

impl DialogBuilder {
    pub fn new(id: DialogId) -> Self {
        Self {
            id,
            modal: true, // Standardmäßig modal
            initially_open: false,
            controlled_open: None,
            default_open: false,
            portal_container: None,
            animate: false,
            on_open_change: None,
            content_builder: None,
            width: Some(Val::Px(400.0)), // Standardbreite
            height: None,                // Auto-Höhe
            overlay_color: None,         // Wird vom Theme geholt
            title: None,
            description: None,
            show_default_close_button: true, // Standard: X oben rechts
        }
    }

    pub fn non_modal(mut self) -> Self {
        self.modal = false;
        self
    }

    pub fn initially_open(mut self) -> Self {
        self.initially_open = true;
        self
    }

    /// Definiert den Inhalt des Dialogs über eine Closure.
    pub fn with_content(
        mut self,
        builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    ) -> Self {
        self.content_builder = Some(Box::new(builder));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn hide_default_close_button(mut self) -> Self {
        self.show_default_close_button = false;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.controlled_open = Some(open);
        self
    }

    /// steuert Öffnen/Schließen von außen (controlled)
    pub fn controlled_open(mut self, open: bool) -> Self {
        self.controlled_open = Some(open);
        self
    }

    /// setzt den Default-Open-Status (uncontrolled)
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Ziel-Container für das Portal
    pub fn with_portal(mut self, container: Entity) -> Self {
        self.portal_container = Some(container);
        self
    }

    /// Fade-In/Fade-Out aktivieren
    pub fn with_animation(mut self, enable: bool) -> Self {
        self.animate = enable;
        self
    }

    /// Callback, wenn sich open ändert
    pub fn on_open_change(mut self, f: impl Fn(DialogState) + Send + Sync + 'static) -> Self {
        self.on_open_change = Some(Box::new(f));
        self
    }

    /// Spawnt den kompletten Dialog (Overlay + Content).
    /// Der Dialog wird normalerweise **nicht** direkt als Kind eines anderen UI-Elements
    /// gespawnt, sondern auf oberster Ebene (z.B. als Kind der Root-UI-Node oder
    /// einer dedizierten Overlay-Node), um das Z-Index-Problem zu umgehen.
    #[must_use]
    pub fn spawn<'w, 'a>(
        self,
        commands: &'a mut Commands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
        // Optional: Icons für Default Close Button
        close_icon: Option<&Handle<Image>>,
        // Optional: Eine feste Parent-Entity für alle Dialoge (Portal-Container)
        dialog_container: Option<Entity>,
        portal_container: Option<Res<DialogPortalContainer>>,
    ) -> Entity {
        let target_container = dialog_container.or_else(|| portal_container.map(|r| r.0));
        let overlay_bg = self.overlay_color.unwrap_or(Color::BLACK.with_alpha(0.6));

        // Styling für den Content-Bereich
        let mut content_style = DialogContentStyle {
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.),
                top: Val::Percent(50.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                width: self.width.unwrap_or(Val::Px(400.0)),
                height: self.height.unwrap_or_default(),
                row_gap: Val::Px(12.0),
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
            background_color: BackgroundColor(theme.color.gray.step02),
            transform: Transform::from_translation(Vec3::new(-50., -50., 0.)),
        };

        if let Some(w) = self.width {
            content_style.node.width = w;
        }
        if let Some(h) = self.height {
            content_style.node.height = h;
        }

        // Initiale Sichtbarkeit anhand controlled/uncontrolled
        let is_open = self.controlled_open.unwrap_or(self.default_open);
        let initial_visibility = if is_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        // --- Spawnen des Dialog-Wurzelknotens ---
        let root_entity = commands
            .spawn((
                DialogRoot {
                    id: self.id.clone(),
                    initially_open: self.default_open,
                    modal: self.modal,
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                initial_visibility,
                GlobalZIndex(10),
                DialogState {
                    open: is_open,
                    default_open: self.default_open,
                },
            ))
            .id();
        // Content-Entity schon vorab erzeugen!
        let content_entity = commands.spawn((DialogContent, content_style.clone())).id();

        if let Some(container) = target_container {
            commands.entity(container).add_child(root_entity);
        }
        // Animation: Fade-In / Scale
        if self.animate {
            commands
                .entity(content_entity)
                .insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(200),
                    TransformScaleLens {
                        start: Vec3::ZERO,
                        end: Vec3::ONE,
                    },
                )));
            // Keep mounted während Ausblendung
            commands.entity(root_entity).insert(KeepMounted(true));
        }

        // Kinder: Overlay & Content
        commands.entity(root_entity).with_children(|builder| {
            // Overlay nur bei modal
            if self.modal {
                builder.spawn((
                    DialogOverlay,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        top: Val::Px(0.),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(overlay_bg),
                    GlobalZIndex(9),
                    FocusPolicy::Block,
                ));
            }
        });

        commands.entity(content_entity).with_children(|content| {
            if let Some(title_text) = self.title {
                content.spawn((
                    Text::new(title_text),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.color.white.step11),
                ));
            }
            if let Some(desc_text) = self.description {
                content.spawn((
                    Text::new(desc_text),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.color.white.step09),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
            }
            if let Some(builder_fn) = self.content_builder {
                (builder_fn)(content, theme, font_handle);
            }
            if self.show_default_close_button {
                content
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(theme.layout.padding.base),
                        right: Val::Px(theme.layout.padding.base),
                        ..default()
                    })
                    .with_children(|btn_parent| {
                        let mut close_btn = ButtonBuilder::new()
                            .size(ButtonSize::Icon)
                            .variant(ButtonVariant::Ghost)
                            .add_marker(|cmd| {
                                cmd.insert(DialogCloseTrigger);
                            });
                        if let Some(icon) = close_icon {
                            close_btn = close_btn.with_icon(icon.clone());
                        } else {
                            close_btn = close_btn.with_text("X");
                        }
                        let _ = close_btn.spawn(btn_parent, theme, font_handle);
                    });
            }
        }); // Inhalt des Dialogs (Title, Description, Custom Content, Close-Button)

        root_entity
    }
}

// --- Systeme ---

/// System zum Öffnen eines Dialogs über Event.
pub fn open_dialog_system(
    mut ev_open: EventReader<OpenDialogEvent>,
    mut q_dialogs: Query<(Entity, &DialogRoot, &mut Visibility, &mut DialogState)>,
    mut active: ResMut<ActiveDialogs>,
) {
    for OpenDialogEvent(id) in ev_open.read() {
        for (entity, root, mut vis, mut state) in q_dialogs.iter_mut() {
            if root.id == *id {
                if !state.open {
                    *vis = Visibility::Inherited;
                    state.open = true;
                    // wenn modal, schließe alle anderen
                    if root.modal {
                        for &old in active.modals.iter() {
                            if old != entity {
                                if let Ok((_, _, mut old_vis, mut old_state)) =
                                    q_dialogs.get_mut(old)
                                {
                                    *old_vis = Visibility::Hidden;
                                    old_state.open = false;
                                }
                            }
                        }
                        active.modals.insert(entity);
                    }
                }
                break;
            }
        }
    }
}

/// System to close modal dialogs via event, button click, or ESC key.
///
/// This system listens for three ways to request a dialog close:
/// 1. A `CloseDialogEvent` is sent.
/// 2. A click on any entity with `DialogCloseTrigger` and `Button` when its `Interaction` changes to `Pressed`.
/// 3. The user presses the ESC key (`KeyCode::Escape`).
///
/// # Run Condition
/// This system should only run when a dialog is currently open. To enforce this, register it with:
///
/// ```rust,ignore
/// app.add_systems(
///     Update,
///     close_dialog_system.run_if(|active_dialogs: Res<ActiveDialogs>| {
///         active_dialogs.current_modal.is_some()
///     }),
/// );
/// ```
///
/// # Parameters
/// - `ev_close`: Reader for `CloseDialogEvent`. Cleared after processing.
/// - `q_close_triggers`: Query for `(Changed<Interaction>, With<DialogCloseTrigger>, With<Button>)`. Detects Pressed interactions.
/// - `keyboard`: Resource providing key input via `ButtonInput<KeyCode>`, used to detect ESC.
/// - `active_dialogs`: Mutable resource tracking the currently active modal dialog, if any.
/// - `q_dialogs`: Query for `(Entity, &mut Visibility, &DialogRoot)` to locate and hide the open dialog.
///
/// # Behavior
/// 1. Check for close request via event, button press, or ESC key.
/// 2. If a close is requested and `active_dialogs.current_modal` has an `Entity`:
///    - Hide that dialog by setting its `Visibility` to `Hidden`.
///    - Log an info message.
/// 3. If close is requested but no modal is active, log a warning.
///
/// # Example
///
/// ```rust,ignore
/// // Register the system with a run condition so it only executes when a dialog is open
/// app.add_systems(
///     Update,
///     close_dialog_system.run_if(|active_dialogs: Res<ActiveDialogs>| {
///         active_dialogs.current_modal.is_some()
///     }),
/// );
/// ```
pub fn close_dialog_system(
    ev_close: EventReader<CloseDialogEvent>,
    q_close_triggers: Query<&Interaction, (Changed<Interaction>, With<DialogCloseTrigger>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActiveDialogs>,
    mut q_dialogs: Query<(Entity, &mut Visibility, &DialogRoot, &mut DialogState)>,
) {
    let should = !ev_close.is_empty()
        || q_close_triggers.iter().any(|i| *i == Interaction::Pressed)
        || keyboard.just_pressed(KeyCode::Escape);

    if should {
        // alle offenen modalen Dialoge schließen
        for &entity in active.modals.iter() {
            if let Ok((_, mut vis, _, mut state)) = q_dialogs.get_mut(entity) {
                *vis = Visibility::Hidden;
                state.open = false;
            }
        }
        active.modals.clear();
    }
}

/// System, das Klicks auf das Overlay abfängt (nur wenn modal).
/// Wenn ein modaler Dialog offen ist, schließt ein Klick auf das Overlay den Dialog.
pub fn handle_overlay_click_system(
    q_overlays: Query<
        (Entity, &Interaction, &ChildOf),
        (Changed<Interaction>, With<DialogOverlay>),
    >,
    q_dialog_root: Query<&DialogRoot>, // Um Modalität zu prüfen
    mut ev_close: EventWriter<CloseDialogEvent>,
) {
    for (_overlay_entity, interaction, child_of) in q_overlays.iter() {
        if *interaction == Interaction::Pressed {
            // Oder Clicked, je nach Bevy Version/Verhalten
            // Finde den DialogRoot des Overlays
            if let Ok(dialog_root) = q_dialog_root.get(child_of.parent()) {
                // Nur schließen, wenn modal und geklickt
                if dialog_root.modal {
                    info!("Overlay clicked, sending CloseDialogEvent.");
                    ev_close.write(CloseDialogEvent {});
                }
            }
        }
    }
}

// Optional: System, das verhindert, dass hinter einem modalen Dialog geklickt wird
// (kann komplex sein, einfacher ist oft, Klicks auf Overlay zum Schließen zu nutzen)

// --- System zum initialen Registrieren von `initially_open` Dialogen ---
pub fn register_initially_open_dialogs(
    mut active_dialogs: ResMut<ActiveDialogs>,
    query: Query<(Entity, &DialogRoot), Added<DialogRoot>>, // Nur neue Dialoge
) {
    for (entity, root) in query.iter() {
        if root.initially_open && root.modal {
            if let Some(old_modal) = active_dialogs.modals.get(&entity) {
                warn!("Multiple initially open modal dialogs detected! Only the last one registered ({:?}) will be active. Previous: {:?}", entity, old_modal);
            }
            active_dialogs.modals.clear();
            active_dialogs.modals.insert(entity);
            info!("Registered initially open modal dialog: {:?}", root.id);
        }
        // TODO: Handle initially open non-modal dialogs if needed
    }
}

pub fn setup_dialog_portal_container(mut commands: Commands) {
    let container = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.),
                top: Val::Percent(50.),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::NONE),
            Name::new("DialogPortalContainer"),
        ))
        .id();
    commands.insert_resource(DialogPortalContainer(container));
}
