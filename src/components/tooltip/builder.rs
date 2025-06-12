use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    TooltipContentMarker, TooltipContentStyle, TooltipMarker, TooltipState, TooltipTriggerMarker,
};

/// Builder for a basic tooltip shown on hover.
pub struct TooltipBuilder {
    trigger: String,
    text: String,
}

impl TooltipBuilder {
    /// Create a new tooltip with the given trigger label and message.
    pub fn new(trigger: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            trigger: trigger.into(),
            text: text.into(),
        }
    }
}

impl<'w, 's> UiBuilder<'w, 's> for TooltipBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let mut root = parent.spawn((
            TooltipMarker,
            TooltipState { open: false },
            Name::new("Tooltip"),
        ));
        root.with_children(|rc| {
            rc.spawn((
                TooltipTriggerMarker,
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Interaction::default(),
            ))
            .with_children(|tc| {
                LabelBuilder::new(self.trigger.clone()).spawn(tc, theme, font);
            });

            rc.spawn((
                TooltipContentMarker,
                TooltipContentStyle::new(theme),
                Visibility::Hidden,
            ))
            .with_children(|cc| {
                LabelBuilder::new(self.text.clone()).spawn(cc, theme, font);
            });
        });
        root.id()
    }
}
