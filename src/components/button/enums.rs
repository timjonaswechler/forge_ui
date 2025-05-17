use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    #[default]
    Solid,
    Soft,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    Large,
}

pub enum ButtonChild {
    /// Text-Inhalt
    Text(String),
    /// Beliebige Closure für eigene Child-Elemente
    Custom(Box<dyn FnOnce(&mut ChildSpawnerCommands) + Send + Sync>),
}
