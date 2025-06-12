use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{HoverCardContentMarker, HoverCardContentStyle, HoverCardMarker, HoverCardState, HoverCardTriggerMarker};

/// Builder for a simple hover card.
pub struct HoverCardBuilder {
    trigger: String,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl HoverCardBuilder {
    /// Create a new hover card with the given trigger label.
    pub fn new(trigger: impl Into<String>) -> Self {
        Self { trigger: trigger.into(), content: None }
    }

    /// Provide custom content for the hover card.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }

    #[must_use]
    pub fn spawn<'w, 's>(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Entity {
        let trigger_label = self.trigger.clone();
        let mut cmd = parent.spawn((HoverCardMarker, HoverCardState { open: false }, Name::new("HoverCard")));

        cmd.with_children(|cb| {
            cb.spawn((
                HoverCardTriggerMarker,
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Interaction::default(),
                FocusPolicy::Block,
            ))
            .with_children(|tc| {
                LabelBuilder::new(trigger_label).spawn(tc, theme, font);
            });

            let style = HoverCardContentStyle::new(theme);
            let mut content_cmd = cb.spawn((HoverCardContentMarker, style, Visibility::Hidden));
            if let Some(f) = self.content {
                content_cmd.with_children(|c| {
                    f(c, theme, font);
                });
            }
        });

        cmd.id()
    }
}
