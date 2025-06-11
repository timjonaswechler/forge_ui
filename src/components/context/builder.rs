use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::ContextProvider;

/// Builder for a [`ContextProvider`] component.
pub struct ContextProviderBuilder<T: Clone + Send + Sync + 'static> {
    value: T,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &T, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl<T: Clone + Send + Sync + 'static> ContextProviderBuilder<T> {
    /// Create a new context provider with the given value.
    pub fn new(value: T) -> Self {
        Self { value, content: None }
    }

    /// Spawn children inside this context provider.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &T, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's, T: Clone + Send + Sync + 'static> UiBuilder<'w, 's> for ContextProviderBuilder<T> {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let stored_value = self.value.clone();
        let mut cmd = parent.spawn(ContextProvider { value: self.value });
        if let Some(func) = self.content {
            cmd.with_children(|cb| {
                func(cb, &stored_value, theme, font);
            });
        }
        cmd.id()
    }
}
