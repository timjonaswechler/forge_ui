use bevy::prelude::*;

/// Generic context provider component holding a value of type `T`.
#[derive(Component, Debug, Clone)]
pub struct ContextProvider<T: Clone + Send + Sync + 'static> {
    pub value: T,
}
