// src/components/dialog/builder/builder.rs
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::super::*;
use super::*;
use crate::components::portal::*;
use crate::theme::UiTheme;

/// Builder für einen gesamten Dialog (Overlay + Content).
/// Alle konkreten Abschnitts‑Details (Header/Body/Footer) werden über
/// `DialogContentBuilder` injiziert.
pub struct DialogBuilder {
    // Identität
    id: DialogId,
    // Anfangszustand
    initially_open: bool,

    // Wo soll der Dialog angehängt werden?
    target_container: Option<Entity>,
    use_global_portal: bool,

    // --- Inhalt ---
    content_builder: Option<DialogContentBuilder>,

    // --- Layout ---
    width: Option<Val>,
    height: Option<Val>,
    overlay_color: Option<Color>,
    z_index_offset: i32,

    // --- Komfortoptionen ---
    show_default_close_button: bool,
}

impl DialogBuilder {
    // ---------------------------------------------------------------------
    // Konstruktoren
    // ---------------------------------------------------------------------
    pub fn new(id: DialogId) -> Self {
        Self {
            id,
            initially_open: false,
            target_container: None,
            use_global_portal: true,
            content_builder: None,
            width: Some(Val::Px(500.0)),
            height: None,
            overlay_color: None,
            z_index_offset: 0,
            show_default_close_button: true,
        }
    }

    pub fn new_unique() -> Self {
        Self::new(DialogId::new_unique())
    }

    // ---------------------------------------------------------------------
    // Konfig‑API (chainable)
    // ---------------------------------------------------------------------
    pub fn initially_open(mut self, flag: bool) -> Self {
        self.initially_open = flag;
        self
    }

    pub fn content(mut self, builder: DialogContentBuilder) -> Self {
        self.content_builder = Some(builder);
        self
    }

    pub fn width(mut self, w: Val) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: Val) -> Self {
        self.height = Some(h);
        self
    }

    pub fn overlay_color(mut self, c: Color) -> Self {
        self.overlay_color = Some(c);
        self
    }

    pub fn z_index_offset(mut self, offset: i32) -> Self {
        self.z_index_offset = offset;
        self
    }

    pub fn hide_default_close_button(mut self) -> Self {
        self.show_default_close_button = false;
        self
    }

    // ---------------------------------------------------------------------
    // SPAWN – Erzeugt alle Entities des Dialogs
    // ---------------------------------------------------------------------
    #[must_use]
    pub fn spawn(
        self,
        commands: &mut Commands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
        global_portal_root: Option<Res<ForgeUiPortalRoot>>, // kann None sein
    ) -> Entity {
        // ---------- Farben & Sichtbarkeit ----------
        let overlay_color = self.overlay_color.unwrap_or(theme.color.black.step07);
        let bg_color = theme.color.gray.step02;
        let visibility = if self.initially_open {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        // ---------- Root‑Entity ----------
        let root_id = commands
            .spawn((
                DialogRootMarker,
                DialogConfig { id: self.id },
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
                visibility,
                ZIndex(theme.layout.z_index.modal_base + self.z_index_offset),
                DialogState {
                    open: self.initially_open,
                },
                Interaction::None,
            ))
            .id();

        // ---------- Optionale Parent‑Zuweisung ----------
        if self.use_global_portal {
            if let Some(root) = global_portal_root {
                commands.entity(root_id).insert(ChildOf(root.0));
            }
        } else if let Some(container) = self.target_container {
            commands.entity(root_id).insert(ChildOf(container));
        }

        // -----------------------------------------------------------------
        // Kinder anlegen
        // -----------------------------------------------------------------
        commands.entity(root_id).with_children(|root| {
            // 1) Overlay
            root.spawn((
                DialogOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(overlay_color.into()),
                FocusPolicy::Block,
                Interaction::default(),
            ));

            // 2) Content‑Wrapper
            let wrapper_style = Node {
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

            root.spawn((DialogContentBundle {
                marker: DialogContent,
                node: wrapper_style,
                background_color: bg_color.into(),
                border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
            },))
                .with_children(|content_parent| {
                    // ---------- Header / Body / Footer per Content‑Builder ----------
                    if let Some(content) = self.content_builder {
                        content.spawn_into(content_parent, theme, font_handle);
                    }
                });
        });

        root_id
    }
}
