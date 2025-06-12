use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;
use crate::components::visually_hidden::VisuallyHiddenBuilder;

use super::AccessibleIconMarker;

/// Builder spawning an icon with an accessible label hidden from view.
pub struct AccessibleIconBuilder {
    icon: Handle<Image>,
    label: String,
}

impl AccessibleIconBuilder {
    /// Creates a new accessible icon builder.
    pub fn new(icon: Handle<Image>, label: impl Into<String>) -> Self {
        Self { icon, label: label.into() }
    }
}

impl<'w, 's> UiBuilder<'w, 's> for AccessibleIconBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((AccessibleIconMarker, Node::default()));
        cmd.with_children(|cb| {
            cb.spawn(ImageNode::new(self.icon.clone()));
            VisuallyHiddenBuilder::new(self.label).spawn(cb, theme, font);
        });
        cmd.id()
    }
}
