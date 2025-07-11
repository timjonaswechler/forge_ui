// src/components/radio/builder.rs
//! Fluent builder for individual radio buttons.
//!
//! This module provides [`RadioBuilder`], which mirrors the appearance and
//! behaviour of the Radix UI radio component. It allows creating standalone
//! radio buttons that can optionally be grouped via [`RadioGroup`] and react to
//! selection changes through callbacks.

use bevy::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

use super::*;
use crate::theme::UiTheme;

/// Global registry for selection callbacks used by radio buttons.
#[derive(Default, Resource)]
pub struct OnSelectRegistry {
    next_id: AtomicU32,
    callbacks: std::collections::HashMap<u32, Box<dyn Fn(String) + Send + Sync>>,
}

impl OnSelectRegistry {
    /// Registers a callback and returns its unique ID.
    pub fn register<F>(&mut self, callback: F) -> u32
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.callbacks.insert(id, Box::new(callback));
        id
    }

    /// Invokes the callback registered under `id` with the provided value.
    pub fn call(&self, id: u32, value: String) {
        if let Some(cb) = self.callbacks.get(&id) {
            cb(value);
        }
    }
}

/// Builder for an interactive radio button.
///
/// The builder exposes a fluent API to configure variant, size, group and
/// checked state. Example:
/// ```rust
/// use forge_ui::components::radio::{RadioBuilder, RadioVariant};
/// # use bevy::prelude::*;
/// # fn demo(mut commands: Commands, theme: Res<UiTheme>, font: Res<Handle<Font>>) {
/// commands.spawn(NodeBundle::default()).with_children(|p| {
///     let _ = RadioBuilder::new("a")
///         .variant(RadioVariant::Primary)
///         .group("example")
///         .spawn(p, &theme, &font);
/// });
/// # }
/// ```
pub struct RadioBuilder {
    variant: RadioVariant,
    size: RadioSize,
    disabled: bool,
    value: String,
    group: Option<String>,
    checked: bool,
    on_select_id: Option<u32>,
}

impl RadioBuilder {
    /// Creates a new builder with the given value used as the radio's payload.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            variant: RadioVariant::Primary,
            size: RadioSize::Medium,
            disabled: false,
            value: value.into(),
            group: None,
            checked: false,
            on_select_id: None,
        }
    }

    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = v.into();
        self
    }

    pub fn variant(mut self, v: RadioVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn size(mut self, s: RadioSize) -> Self {
        self.size = s;
        self
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn group(mut self, name: impl Into<String>) -> Self {
        self.group = Some(name.into());
        self
    }
    /// Registers a callback that fires when this radio is selected.
    ///
    /// The callback receives the radio's value and will be stored in a global
    /// [`OnSelectRegistry`].
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        // Wir holen uns zur Build-Zeit die Ressource und registrieren die Closure.
        // Achtung: dieser Schritt muss *vor* `App::run()` gemacht werden!
        // Besser: man sammelt die Closure hier und registriert sie später in einem Setup-System.
        let mut registry = OnSelectRegistry::default();
        let id = registry.register(callback);
        self.on_select_id = Some(id);
        self
    }
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        _font: &Handle<Font>,
    ) -> EntityCommands<'s> {
        let style = get_radio_style_def(self.variant, self.size, theme);

        let background_color = if self.disabled {
            style.disabled
        } else {
            style.background
        };
        let border_color = if self.disabled {
            style.border.with_alpha(0.5)
        } else {
            style.border
        };

        let mut entity = parent.spawn((
            // Layout & Styling
            Node {
                width: Val::Px(style.size),
                height: Val::Px(style.size),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(background_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Percent(50.0)),
            Interaction::default(), // für Klick-Erkennung
            RadioMarker,            // unser Marker
            RadioState {
                checked: self.checked,
                disabled: self.disabled,
                value: self.value.clone(),
                variant: self.variant,
                size: self.size,
            },
        ));

        if let Some(id) = self.on_select_id {
            entity.insert(OnSelectId(id));
        }

        // Radio gehört (optional) zu einer Gruppe
        if let Some(name) = self.group {
            entity.insert(RadioGroup(name));
        }

        // Innerer Indikator
        entity.with_children(|parent| {
            let indicator_size = style.size / 2.0;
            parent.spawn((
                Node {
                    width: Val::Px(indicator_size),
                    height: Val::Px(indicator_size),
                    ..default()
                },
                BackgroundColor(style.indicator),
                BorderRadius::all(Val::Percent(50.0)),
                if self.checked {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                RadioIndicator,
            ));
        });

        entity
    }
}
