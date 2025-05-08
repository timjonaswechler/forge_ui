// src/components/radio/components.rs
use bevy::prelude::*;

use crate::{RadioSize, RadioVariant};

#[derive(Component)]
pub struct RadioIndicator;

#[derive(Component)]
pub struct RadioMarker;

#[derive(Component)]
pub struct RadioState {
    pub checked: bool,
    pub disabled: bool,
    pub value: String,
    pub variant: RadioVariant,
    pub size: RadioSize,
}

#[derive(Component)]
pub struct OnSelect(pub Box<dyn Fn(String) + Send + Sync + 'static>);

#[derive(Component)]
pub struct RadioGroup(pub String);

#[derive(Component)]
pub struct OnSelectId(pub u32);
