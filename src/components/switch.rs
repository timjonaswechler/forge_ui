// crates/forge_ui/src/components/toggle_switch.rs

use crate::theme::UiTheme;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

// --- Komponenten ---

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Switch-Track
pub struct ToggleSwitchMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Daumen
pub struct ToggleSwitchThumbMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für die Overlay-Node bei disabled
pub struct ToggleSwitchOverlayMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
/// Zustand des Switches
pub struct ToggleSwitchState {
    pub checked: bool,
    pub disabled: bool,
}

// --- Events ---

#[derive(Event, Debug, Clone)]
pub struct ToggleSwitchChangedEvent {
    pub switch_entity: Entity,
    pub is_checked: bool,
}

// --- Builder ---

pub struct ToggleSwitchBuilder {
    checked: bool,
    disabled: bool,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
    radius: Option<f32>,
}

impl Default for ToggleSwitchBuilder {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            markers: Vec::new(),
            radius: None,
        }
    }
}

impl ToggleSwitchBuilder {
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
            ToggleSwitchMarker,
            Button,
            Node {
                width: track_w,
                height: track_h,
                display: Display::Flex,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(margin),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(if self.checked {
                theme.accent_color.step09
            } else {
                theme.color.gray.step05
            }),
            BorderRadius::all(Val::Px(track_radius)),
            if self.disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            },
            ToggleSwitchState {
                checked: self.checked,
                disabled: self.disabled,
            },
        ));

        // Thumb
        cmd.with_children(|p| {
            p.spawn((
                ToggleSwitchThumbMarker,
                Node {
                    width: thumb_sz,
                    height: thumb_sz,
                    justify_self: if self.checked {
                        JustifySelf::End
                    } else {
                        JustifySelf::Start
                    },
                    ..default()
                },
                BorderRadius::all(Val::Px(thumb_radius)),
                BackgroundColor(theme.color.white.step12.into()),
                FocusPolicy::Pass,
            ));

            // Overlay für disabled
            p.spawn((
                ToggleSwitchOverlayMarker,
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

/// Farben und Position für Track & Thumb
fn get_switch_colors_and_pos(checked: bool, theme: &UiTheme) -> (Color, Color, JustifySelf) {
    let track = if checked {
        theme.accent_color.step09
    } else {
        theme.color.gray.step05
    };
    let thumb = theme.color.white.step12;

    let pos = if checked {
        JustifySelf::End
    } else {
        JustifySelf::Start
    };
    (track, thumb, pos)
}

/// Update-System passt Thumb & Overlay an
pub fn update_toggle_switch_visuals(
    theme_opt: Option<Res<UiTheme>>,
    mut params: ParamSet<(
        Query<
            (
                Entity,
                &ToggleSwitchState,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
            ),
            (Changed<ToggleSwitchState>, With<ToggleSwitchMarker>),
        >,
        Query<(&mut Node, &mut BackgroundColor, &mut Visibility), With<ToggleSwitchOverlayMarker>>,
        Query<(&mut Node, &mut BackgroundColor), With<ToggleSwitchThumbMarker>>,
    )>,
) {
    let Some(theme) = theme_opt else {
        return;
    };
    let mut thumbs = Vec::new();
    let mut overlays = Vec::new();

    // First, collect all necessary data to avoid multiple mutable borrows
    for (e, state, _, _, children) in params.p0().iter_mut() {
        let (tb, th, pos) = get_switch_colors_and_pos(state.checked, &theme);
        thumbs.extend(children.iter().map(|c| (c, pos, th)));
        overlays.push((e, state.disabled, tb));
    }

    // Now, update the visuals with separate mutable borrows
    for (e, disabled, tb) in overlays {
        if let Ok((_, _, mut vis)) = params.p1().get_mut(e) {
            *vis = if disabled {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        if let Ok((_, _, mut bg, mut border, _)) = params.p0().get_mut(e) {
            *bg = tb.into();
            *border = if disabled {
                theme.color.gray.step03.into()
            } else {
                theme.color.gray.step06.into()
            };
        }
    }

    for (child, pos, color) in thumbs {
        if let Ok((mut node, mut bg)) = params.p2().get_mut(child) {
            node.justify_self = pos;
            *bg = color.into();
        }
    }
}

/// Klick-Handler toggelt Zustand
pub fn handle_toggle_switch_clicks(
    mut q: Query<
        (&Interaction, &mut ToggleSwitchState),
        (Changed<Interaction>, With<ToggleSwitchMarker>),
    >,
    mut ev: EventWriter<ToggleSwitchChangedEvent>,
) {
    for (int, mut state) in q.iter_mut() {
        if *int == Interaction::Pressed && !state.disabled {
            state.checked = !state.checked;
            ev.write(ToggleSwitchChangedEvent {
                switch_entity: Entity::from_raw(0),
                is_checked: state.checked,
            });
        }
    }
}
