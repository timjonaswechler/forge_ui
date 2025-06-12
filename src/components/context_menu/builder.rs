use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{ContextMenuItemMarker, ContextMenuItemStyle, ContextMenuMarker, ContextMenuStyle};

/// Builder for a simple context menu.
pub struct ContextMenuBuilder {
    items: Vec<String>,
}

impl ContextMenuBuilder {
    /// Create a new empty context menu.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a text item to the context menu.
    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ContextMenuBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let style = ContextMenuStyle::new(theme);
        let mut cmd = parent.spawn((ContextMenuMarker, style));
        cmd.with_children(|cb| {
            for item in self.items {
                let item_style = ContextMenuItemStyle::new(theme);
                let mut item_cmd = cb.spawn((ContextMenuItemMarker, item_style));
                item_cmd.with_children(|icb| {
                    let _ = LabelBuilder::new(item).spawn(icb, theme, font);
                });
            }
        });
        cmd.id()
    }
}
