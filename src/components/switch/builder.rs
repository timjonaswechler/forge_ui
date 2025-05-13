use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

use super::*;
use crate::theme::UiTheme;

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
/// // Anfangs aktivierter Switch
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
                    theme.accent.step09
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
