use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{
    BaseDialogContentMarker, BaseDialogContentStyle, BaseDialogOverlayMarker,
    BaseDialogOverlayStyle, BaseDialogScrollMarker, BaseDialogScrollStyle,
    BaseDialogScrollPaddingMarker, BaseDialogScrollPaddingStyle,
};

/// Builder for a simple dialog overlay with content container.
pub struct BaseDialogBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl BaseDialogBuilder {
    /// Creates a new dialog builder.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the dialog content.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for BaseDialogBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut overlay = parent.spawn((BaseDialogOverlayMarker, BaseDialogOverlayStyle::default()));
        overlay.with_children(|overlay_cb| {
            let mut scroll = overlay_cb.spawn((BaseDialogScrollMarker, BaseDialogScrollStyle::default()));
            scroll.with_children(|scroll_cb| {
                let mut padding = scroll_cb.spawn((BaseDialogScrollPaddingMarker, BaseDialogScrollPaddingStyle::default()));
                padding.with_children(|content_cb| {
                    let mut content_cmd = content_cb.spawn((BaseDialogContentMarker, BaseDialogContentStyle::default()));
                    if let Some(content_fn) = self.content {
                        content_cmd.with_children(|cc| {
                            content_fn(cc, theme, font);
                        });
                    }
                });
            });
        });
        overlay.id()
    }
}
