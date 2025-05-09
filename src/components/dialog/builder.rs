// components/dialog/builder.rs
use bevy::prelude::*;
use bevy::ui::FocusPolicy; // Import für FocusPolicy
use bevy_tweening::{lens::*, Animator, Tween};
use std::time::Duration; // Für Animationen // Für Animationen

use crate::components::button::{
    ButtonBuilder, ButtonSize, ButtonVariant, NoAction as ButtonNoAction,
}; // NoAction für Buttons importieren
use crate::components::portal::ForgeUiPortalRoot;
use crate::theme::UiTheme; // Unser globaler Portal-Root

use super::components::*; // Dialog-spezifische Komponenten
                          // Dialog-spezifische Events

// Typ für die Funktion, die den Dialog-Inhalt baut
type DialogContentBuilderFn =
    Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static>;

pub struct DialogBuilder {
    id: DialogId,
    modal: bool,
    initially_open: bool,
    // controlled_open: Option<bool>, // Wird über `initially_open` und Events gesteuert
    // default_open: bool,          // Teil von `initially_open`
    target_container: Option<Entity>,
    use_global_portal: bool,
    animate: bool,
    // on_open_change: Option<Box<dyn Fn(DialogState) + Send + Sync>>, // Wird durch Events ersetzt
    content_builder: Option<DialogContentBuilderFn>,
    width: Option<Val>,
    height: Option<Val>,
    overlay_color: Option<Color>,
    title: Option<String>,
    description: Option<String>,
    show_default_close_button: bool,
    z_index_offset: i32, // Erlaubt Anpassung des ZIndex relativ zum Portal
}

impl DialogBuilder {
    pub fn new(id: DialogId) -> Self {
        Self {
            id,
            modal: true,
            initially_open: false,
            target_container: None,
            use_global_portal: true, // Standardmäßig den globalen Portal-Root verwenden
            animate: true,           // Animation standardmäßig an für ein besseres Gefühl
            content_builder: None,
            width: Some(Val::Px(500.0)), // Angepasste Standardbreite
            height: None,
            overlay_color: None,
            title: None,
            description: None,
            show_default_close_button: true,
            z_index_offset: 0, // Kein Offset standardmäßig
        }
    }

    pub fn new_unique() -> Self {
        Self::new(DialogId::new_unique())
    }

    // --- Konfigurationsmethoden ---
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn initially_open(mut self, initially_open: bool) -> Self {
        self.initially_open = initially_open;
        self
    }

    pub fn spawn_in_container(mut self, container: Entity) -> Self {
        self.target_container = Some(container);
        self.use_global_portal = false;
        self
    }

    pub fn spawn_in_place(mut self) -> Self {
        self.use_global_portal = false;
        self.target_container = None;
        self
    }

    pub fn with_animation(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

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

    pub fn overlay_color(mut self, color: Color) -> Self {
        self.overlay_color = Some(color);
        self
    }

    /// Passt den Z-Index des Dialogs relativ zum Z-Index des Portal-Roots an (oder Basis-ZIndex).
    /// Ein positiver Wert hebt ihn höher, ein negativer niedriger.
    pub fn z_index_offset(mut self, offset: i32) -> Self {
        self.z_index_offset = offset;
        self
    }

    // --- Spawnen ---
    #[must_use]
    pub fn spawn(
        self,
        commands: &mut Commands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
        global_portal_root_opt: Option<Res<ForgeUiPortalRoot>>,
        close_icon: Option<Handle<Image>>,
    ) -> Entity {
        // Wir geben jetzt direkt die Entity ID zurück
        let overlay_final_color = self.overlay_color.unwrap_or(theme.color.black.step07); // black.step07
        let dialog_bg_color = theme.color.gray.step02;

        let is_open = self.initially_open;
        let initial_visibility = if is_open {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        let dialog_root_bundle = (
            DialogRootMarker,
            DialogConfig {
                id: self.id.clone(),
                initially_open: self.initially_open,
                modal: self.modal,
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            initial_visibility,
            ZIndex(theme.layout.z_index.modal_base + self.z_index_offset),
            DialogState { open: is_open },
            Interaction::None, // Dialog-Root fängt keine Klicks ab
        );

        // === Dialog-Root-Entität erstellen und Parent setzen ===
        let mut dialog_root_entity_commands = commands.spawn(dialog_root_bundle);
        let dialog_root_entity_id = dialog_root_entity_commands.id(); // ID merken

        if self.animate {
            dialog_root_entity_commands.insert(KeepMounted(true));
        }

        // Ziel-Parent bestimmen
        let mut parent_entity_for_dialog: Option<Entity> = None;
        if self.use_global_portal {
            if let Some(global_root) = global_portal_root_opt {
                parent_entity_for_dialog = Some(global_root.0);
            } else {
                warn!(
                    "Dialog {:?}: use_global_portal ist true, aber ForgeUiPortalRoot wurde nicht bereitgestellt. Dialog wird Top-Level.",
                    self.id
                );
            }
        } else if let Some(target_container_entity) = self.target_container {
            parent_entity_for_dialog = Some(target_container_entity);
        }
        // Wenn spawn_in_place gewählt wurde, bleibt parent_entity_for_dialog None (Top-Level).

        // Parent setzen, FALLS einer bestimmt wurde
        if let Some(actual_parent_id) = parent_entity_for_dialog {
            // WICHTIG: .set_parent() auf die EntityCommands des *Kindes* anwenden.
            dialog_root_entity_commands.insert(ChildOf(actual_parent_id));
        }
        // `dialog_root_entity_commands` ist jetzt konfiguriert und (potenziell) geparentet.

        // Variable, um die content_wrapper_id zu speichern für die Animation
        let mut aniatable_content_entity_id: Option<Entity> = None;

        // Nutze `commands.entity(dialog_root_entity_id).with_children()`
        dialog_root_entity_commands.with_children(|root_builder| {
            // 1. Overlay (wenn modal)
            if self.modal {
                root_builder.spawn((
                    DialogOverlay,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(overlay_final_color.into()),
                    FocusPolicy::Block,
                    Interaction::default(), // Um Klicks auf Overlay zu erkennen
                ));
            }

            // 2. Content-Bereich (das eigentliche Dialogfenster)
            let content_wrapper_node = Node {
                position_type: PositionType::Relative,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                width: self.width.unwrap_or(Val::Px(300.0)), //TODO
                height: self.height.unwrap_or_default(),
                max_width: Val::Percent(90.0),
                max_height: Val::Percent(90.0),
                row_gap: Val::Px(theme.layout.gap.base),
                align_self: AlignSelf::Center, // Stellt sicher, dass der Content-Wrapper sich im Root zentriert
                justify_self: JustifySelf::Center, // Stellt sicher, dass der Content-Wrapper sich im Root zentriert
                ..default()
            };
            let content_transform = if self.animate {
                Transform::from_scale(Vec3::ZERO)
            } else {
                Transform::default()
            };

            // Spawne den Content-Wrapper und hole seine ID für die Animation
            let mut content_wrapper_cmd = root_builder.spawn((DialogContentBundle {
                marker: DialogContent, // Sicherstellen, dass der Marker gesetzt wird
                node: content_wrapper_node,
                background_color: dialog_bg_color.into(),
                transform: content_transform,
                border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
                ..default()
            },));
            aniatable_content_entity_id = Some(content_wrapper_cmd.id()); // ID speichern

            // Inhalt des Dialogfensters
            content_wrapper_cmd.with_children(|content_parent| {
                // Titel
                if let Some(title_text) = &self.title {
                    // self wurde noch nicht konsumiert
                    content_parent.spawn((
                        Text::new(title_text.clone()),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: theme.font.font_size.h4,

                            ..default()
                        },
                        TextColor(theme.color.gray.step12),
                        Node {
                            margin: UiRect::bottom(Val::Px(theme.layout.gap.sm)),
                            ..default()
                        },
                    ));
                }
                // Beschreibung
                if let Some(desc_text) = &self.description {
                    content_parent.spawn((
                        Text::new(desc_text.clone()),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: theme.font.font_size.base,
                            ..default()
                        },
                        TextColor(theme.color.gray.step11),
                        Node {
                            margin: UiRect::bottom(Val::Px(theme.layout.gap.base)),
                            ..default()
                        },
                    ));
                }

                // Benutzerdefinierter Inhalt
                // Hier wird `self.content_builder` konsumiert, wenn `self` `mut` ist.
                // Wenn `self` nicht `mut` ist und per Wert übergeben wurde, ist es okay.
                // Der `content_builder` muss aus `self` herausbewegt werden, bevor `self` (implizit) für was anderes geborgt wird.
                let local_content_builder = self.content_builder; // Bewege es hierher, wenn self nicht mut ist
                if let Some(builder_fn) = local_content_builder {
                    // oder self.content_builder.take()
                    content_parent
                        .spawn(Node {
                            flex_grow: 1.0,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(theme.layout.gap.sm),
                            ..default()
                        })
                        .with_children(|custom_content_parent| {
                            (builder_fn)(custom_content_parent, theme, font_handle);
                        });
                }

                // Standard Schließen-Button
                if self.show_default_close_button {
                    content_parent
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(theme.layout.padding.xs),
                            right: Val::Px(theme.layout.padding.xs),
                            ..default()
                        })
                        .with_children(|btn_area_parent| {
                            let mut close_btn_builder =
                                ButtonBuilder::<ButtonNoAction>::new() // Typ explizit
                                    .size(ButtonSize::Icon)
                                    .variant(ButtonVariant::Ghost)
                                    .add_marker(|cmd| {
                                        cmd.insert(DialogCloseTrigger);
                                    });

                            if let Some(icon_handle) = close_icon.clone() {
                                // Clone, da close_icon ein Option<Handle> ist
                                close_btn_builder = close_btn_builder.icon(icon_handle);
                            } else {
                                close_btn_builder = close_btn_builder.text("✕");
                            }
                            // Verwende die .spawn() Methode des ButtonBuilders. ChildSpawnerCommands ist hier der `btn_area_parent`
                            let _ = close_btn_builder.spawn(btn_area_parent, theme, font_handle);
                        });
                }
            });
        });

        // === SCHRITT 4: Animation für den Content-Teil anfügen, wenn nötig ===
        if self.animate {
            if let Some(content_id_for_anim) = aniatable_content_entity_id {
                commands
                    .entity(content_id_for_anim)
                    .insert(Animator::new(Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(250),
                        TransformScaleLens {
                            start: Vec3::ZERO, // Startet bei 0 für den Einblendeffekt
                            end: Vec3::ONE,
                        },
                    )));
            }
        }

        dialog_root_entity_id
    }
}
