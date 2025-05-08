// src/components/radio/builder.rs

use crate::components::radio::components::{OnSelect, Radio, RadioState};
use crate::components::radio::enums::{RadioSize, RadioVariant};
use crate::components::radio::events::RadioSelectedEvent;
use crate::components::radio::style::get_radio_style_def;
use crate::components::UiTheme;
use bevy::prelude::*;

pub struct RadioBuilder<F: Fn(String) + Send + Sync + 'static = fn(_)> {
    variant: RadioVariant,
    size: RadioSize,
    disabled: bool,
    value: String,
    on_select: Option<F>,
}

impl<F: Fn(String) + Send + Sync + 'static> RadioBuilder<F> {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            variant: RadioVariant::Primary,
            size: RadioSize::Medium,
            disabled: false,
            value: value.into(),
            on_select: None,
        }
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

    pub fn on_select(mut self, callback: F) -> Self {
        self.on_select = Some(callback);
        self
    }

    #[must_use]
    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut ChildBuilder<'w, 's, '_, '_>,
        theme: &UiTheme,
        asset_server: &Res<AssetServer>,
    ) -> EntityCommands<'a> {
        let style = get_radio_style_def(self.variant, self.size, theme);
        let cmd = commands
            .spawn_bundle(
                // setup sprite/material/text
            )
            .insert(Radio)
            .insert(RadioState { checked: false, disabled: self.disabled })
            .insert(OnSelect(Box::new(self.on_select.unwrap_or(|_| {}))));
        cmd
    }
}
