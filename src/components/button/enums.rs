use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    Large,
    Icon,
}

#[derive(Clone, Debug)]
pub enum ButtonChild {
    Text(String),
    Icon(Handle<Image>),
}
