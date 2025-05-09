// crates/forge_ui/src/components/toggle_switch.rs

use crate::theme::UiTheme;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

// --- Komponenten ---
#[derive(Component, Clone, Copy)]
pub struct SwitchTrackColor(pub Option<Color>);

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Switch-Track
pub struct SwitchMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Daumen
pub struct SwitchThumbMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für die Overlay-Node bei disabled
pub struct SwitchOverlayMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
/// Zustand des Switches
pub struct SwitchState {
    pub checked: bool,
    pub disabled: bool,
}

// --- Events ---

#[derive(Event, Debug, Clone)]
pub struct SwitchChangedEvent {
    pub switch_entity: Entity,
    pub is_checked: bool,
}

// --- Builder ---

pub struct SwitchBuilder {
    checked: bool,
    disabled: bool,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
    radius: Option<f32>,
    track_color: Option<Color>,
}

impl Default for SwitchBuilder {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            markers: Vec::new(),
            radius: None,
            track_color: None,
        }
    }
}

///
///         // Anfangs aktivierter Switch
/// ```rust
/// let _ = SwitchBuilder::new()
/// .checked(true)
/// .with_color(theme.color.green.step10)
/// .spawn(parent, &theme /*, &font_handle*/);
/// ```
/// // Deaktivierter Switch
/// ```rust
/// let _ = SwitchBuilder::new()
/// .disabled(true)
/// .spawn(parent, &theme /*, &font_handle*/);
/// ```
/// // Deaktivierter und aktivierter Switch
/// ```rust
/// let _ = SwitchBuilder::new()
/// .checked(true)
/// .disabled(true)
/// .with_color(theme.color.red.step05) // Beispiel: Rote Farbe
/// .spawn(parent, &theme /*, &font_handle*/);
/// ```
impl SwitchBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    #[must_use]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
    ) -> EntityCommands<'a> {
        let track_h = Val::Px(24.0);
        let track_w = Val::Px(44.0);
        let thumb_sz = Val::Px(16.0);
        let margin = Val::Px((24.0 - 16.0) / 2.0);
        let track_radius = self.radius.unwrap_or(theme.layout.radius.xs);

        // Extract f32 values from Val::Px for calculation
        let track_h_px = if let Val::Px(v) = track_h { v } else { 24.0 };
        let thumb_sz_px = if let Val::Px(v) = thumb_sz { v } else { 16.0 };
        let thumb_radius = track_radius - (track_h_px - thumb_sz_px) / 2.0;

        let mut cmd = parent.spawn((
            SwitchMarker,
            Button,
            Node {
                width: track_w,
                height: track_h,
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: if self.checked {
                    JustifyContent::FlexEnd
                } else {
                    JustifyContent::FlexStart
                },
                padding: UiRect::horizontal(margin),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            SwitchTrackColor(self.track_color),
            BackgroundColor(if self.checked {
                if let Some(color) = self.track_color {
                    color
                } else {
                    theme.accent_color.step09
                }
            } else {
                theme.color.gray.step05
            }),
            BorderRadius::all(Val::Px(track_radius)),
            if self.disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            },
            SwitchState {
                checked: self.checked,
                disabled: self.disabled,
            },
        ));

        // Thumb
        cmd.with_children(|p| {
            p.spawn((
                SwitchThumbMarker,
                Node {
                    width: thumb_sz,
                    height: thumb_sz,

                    ..default()
                },
                BorderRadius::all(Val::Px(thumb_radius)),
                BackgroundColor(theme.color.white.step12.into()),
                FocusPolicy::Pass,
            ));

            // Overlay für disabled
            p.spawn((
                SwitchOverlayMarker,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                BorderRadius::all(Val::Px(track_radius)),
                BackgroundColor(theme.color.black.step08.into()),
                if self.disabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                FocusPolicy::Block,
            ));
        });

        for m in self.markers {
            m(&mut cmd);
        }
        cmd
    }
}

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
