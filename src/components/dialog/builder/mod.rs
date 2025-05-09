use crate::{
    ButtonBuilder, ButtonSize, ButtonVariant, DialogCloseTrigger, DialogConfig, DialogContent,
    DialogContentBundle, DialogId, DialogOverlay, DialogRootMarker, DialogState, ForgeUiPortalRoot,
    Icon, KeepMounted, NoAction as ButtonNoAction, UiTheme,
};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_tweening::{lens::*, Animator, Tween};
use std::time::Duration;

pub mod body;
pub mod footer;
pub mod header;
pub use body::DialogBodyBuilder;
pub use footer::DialogFooterBuilder;
pub use header::DialogHeaderBuilder;

type SectionElementBuilderFn =
    Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static>;

pub struct DialogBuilder {
    id: DialogId,
    modal: bool,
    initially_open: bool,
    target_container: Option<Entity>,
    use_global_portal: bool,
    animate: bool,

    // Ersetze die alten Sektions-Funktionen durch die Builder-Instanzen
    header_builder: Option<DialogHeaderBuilder>,
    body_builder: Option<DialogBodyBuilder>,
    footer_builder: Option<DialogFooterBuilder>,

    width: Option<Val>,
    height: Option<Val>,
    overlay_color: Option<Color>,

    // `title` und `description` im Haupt-Builder sind jetzt redundant,
    // da der DialogHeaderBuilder diese Aufgaben übernimmt. Man könnte sie entfernen
    // oder als Schnellzugriff belassen, der intern einen DialogHeaderBuilder erstellt.
    // Um es klar zu halten, entfernen wir sie hier und zwingen die Nutzung des DialogHeaderBuilders.
    // title: Option<String>,
    // description: Option<String>,
    show_default_close_button: bool, // Dieser bleibt, da er global ist.
    z_index_offset: i32,
}

impl DialogBuilder {
    pub fn new(id: DialogId) -> Self {
        Self {
            id,
            modal: true,
            initially_open: false,
            target_container: None,
            use_global_portal: true,
            animate: true,

            header_builder: None,
            body_builder: None,
            footer_builder: None,

            width: Some(Val::Px(500.0)), // Oder dynamischer aus dem Theme
            height: None,                // Passt sich Inhalt an, es sei denn explizit gesetzt
            overlay_color: None,
            show_default_close_button: true,
            z_index_offset: 0,
        }
    }

    // ... (new_unique, modal, initially_open etc. bleiben) ...

    // Methoden zum Setzen der Sektions-Builder
    pub fn header(mut self, header_builder: DialogHeaderBuilder) -> Self {
        self.header_builder = Some(header_builder);
        self
    }

    pub fn body(mut self, body_builder: DialogBodyBuilder) -> Self {
        self.body_builder = Some(body_builder);
        self
    }

    pub fn footer(mut self, footer_builder: DialogFooterBuilder) -> Self {
        self.footer_builder = Some(footer_builder);
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

    pub fn hide_default_close_button(mut self) -> Self {
        self.show_default_close_button = false;
        self
    }

    /// Passt den Z-Index des Dialogs relativ zum Z-Index des Portal-Roots an (oder Basis-ZIndex).
    /// Ein positiver Wert hebt ihn höher, ein negativer niedriger.
    pub fn z_index_offset(mut self, offset: i32) -> Self {
        self.z_index_offset = offset;
        self
    }
    // hide_default_close_button, width, height, overlay_color, z_index_offset bleiben

    // --- Spawnen ---
    #[must_use]
    pub fn spawn(
        self,
        commands: &mut Commands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
        global_portal_root_opt: Option<Res<ForgeUiPortalRoot>>,
        close_icon: Option<Handle<Icon>>,
    ) -> Entity {
        let overlay_final_color = self.overlay_color.unwrap_or(theme.color.black.step07);
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
            Interaction::None,
        );
        let mut dialog_root_entity_commands = commands.spawn(dialog_root_bundle);
        let dialog_root_entity_id = dialog_root_entity_commands.id();

        if self.animate {
            dialog_root_entity_commands.insert(KeepMounted(true));
        }

        let mut parent_entity_for_dialog: Option<Entity> = None;
        if self.use_global_portal {
            if let Some(global_root) = global_portal_root_opt {
                parent_entity_for_dialog = Some(global_root.0);
            } else {
                warn!(
                    "Global portal root not found. Dialog will not be spawned in the global portal."
                );
            }
        } else if let Some(target_container_entity) = self.target_container {
            parent_entity_for_dialog = Some(target_container_entity);
        }
        if let Some(actual_parent_id) = parent_entity_for_dialog {
            dialog_root_entity_commands.insert(ChildOf(actual_parent_id));
        }

        let mut aniatable_content_entity_id: Option<Entity> = None;

        dialog_root_entity_commands.with_children(|root_builder| {
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
                    Interaction::default(),
                ));
            }

            let content_wrapper_node_style = Node {
                position_type: PositionType::Relative,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.layout.padding.base)),
                width: self.width.unwrap_or(Val::Px(500.0)),
                height: self.height.unwrap_or_default(),
                max_width: Val::Percent(90.0),
                max_height: Val::Percent(90.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            };

            let content_transform = if self.animate {
                Transform::from_scale(Vec3::ZERO)
            } else {
                Transform::default()
            };

            let mut content_wrapper_cmd = root_builder.spawn(DialogContentBundle {
                marker: DialogContent,
                node: content_wrapper_node_style,
                background_color: dialog_bg_color.into(),
                transform: content_transform,
                border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
                ..default()
            });
            aniatable_content_entity_id = Some(content_wrapper_cmd.id());

            content_wrapper_cmd.with_children(|dialog_content_parent| {
                // -- 1. Header --
                if let Some(header_builder_instance) = self.header_builder {
                    dialog_content_parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(theme.layout.gap.sm),
                            margin: UiRect::bottom(Val::Px(theme.layout.gap.base)),
                            width: Val::Percent(100.0),
                            ..default()
                        })
                        .with_children(|header_parent_container| {
                            header_builder_instance.spawn_into(
                                header_parent_container,
                                theme,
                                font_handle,
                            );
                        });
                }

                // -- 2. Body --
                if let Some(body_builder_instance) = self.body_builder {
                    dialog_content_parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(theme.layout.gap.base),
                            flex_grow: 1.0,
                            width: Val::Percent(100.0),
                            overflow: Overflow::clip_y(),
                            ..default()
                        })
                        .with_children(|body_parent_container| {
                            body_builder_instance.spawn_into(
                                body_parent_container,
                                theme,
                                font_handle,
                            );
                        });
                }

                // -- 3. Footer --
                if let Some(footer_builder_instance) = self.footer_builder {
                    let footer_justify_content = footer_builder_instance.justify_content;
                    dialog_content_parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: footer_justify_content,
                                column_gap: Val::Px(theme.layout.gap.sm),
                                margin: UiRect::top(Val::Px(theme.layout.gap.base)),
                                padding: UiRect::top(Val::Px(theme.layout.padding.sm)),
                                border: UiRect::top(Val::Px(1.0)),

                                width: Val::Percent(100.0),
                                ..default()
                            },
                            BorderColor(theme.color.gray.step05),
                        ))
                        .with_children(|footer_parent_container| {
                            footer_builder_instance.spawn_into(
                                footer_parent_container,
                                theme,
                                font_handle,
                            );
                        });
                }

                // -- Standard Schließen-Button --
                if self.show_default_close_button {
                    dialog_content_parent
                        .spawn(Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(theme.layout.padding.xs),
                            right: Val::Px(theme.layout.padding.xs),
                            ..default()
                        })
                        .with_children(|btn_area_parent| {
                            let mut close_btn_builder = ButtonBuilder::<ButtonNoAction>::new()
                                .size(ButtonSize::Icon)
                                .variant(ButtonVariant::Ghost)
                                .add_marker(|cmd| {
                                    cmd.insert(DialogCloseTrigger);
                                });

                            if let Some(icon_handle) = close_icon {
                                // close_btn_builder = close_btn_builder.icon(icon_handle); // TODO: Implement icon support
                            } else {
                                close_btn_builder = close_btn_builder.text("✕");
                            }
                            let _ = close_btn_builder.spawn(btn_area_parent, theme, font_handle);
                        });
                }
            });
        });

        // ... (Animation und Rückgabe von dialog_root_entity_id bleiben gleich) ...
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
