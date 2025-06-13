use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    TooltipContentMarker, TooltipContentStyle, TooltipMarker, TooltipState,
    TooltipTriggerMarker,
};

/// Builder for a simple tooltip with text content.
pub struct TooltipBuilder {
    trigger: String,
    text: String,
}

impl TooltipBuilder {
    /// Create a new tooltip with a trigger label and text content.
    pub fn new(trigger: impl Into<String>, text: impl Into<String>) -> Self {
        Self { trigger: trigger.into(), text: text.into() }
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
        let tooltip_text = self.text.clone();
        let mut cmd = parent.spawn((
            TooltipMarker,
            TooltipState { open: false },
            Name::new("Tooltip"),
        ));

        cmd.with_children(|cb| {
            cb.spawn((
                TooltipTriggerMarker,
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Interaction::default(),
                FocusPolicy::Block,
            ))
            .with_children(|tc| {
                LabelBuilder::new(self.trigger.clone()).spawn(tc, theme, font);
            });

            let mut content = cb.spawn((
                TooltipContentMarker,
                TooltipContentStyle::new(theme),
                Visibility::Hidden,
            ));

            content.with_children(|cc| {
                LabelBuilder::new(tooltip_text).spawn(cc, theme, font);
            });
        });

        cmd.id()
    }
}
