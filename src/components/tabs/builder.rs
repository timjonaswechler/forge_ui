use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{
    TabContentMarker, TabContentStyle, TabTriggerMarker, TabTriggerStyle, TabsMarker,
    TabsOrientation, TabsRootStyle, TabsState,
};

pub struct TabsBuilder {
    tabs: Vec<(
        String,
        Box<dyn FnMut(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>,
    )>,
    orientation: TabsOrientation,
    selected: usize,
}

impl TabsBuilder {
    pub fn new() -> Self {
        Self { tabs: Vec::new(), orientation: TabsOrientation::Horizontal, selected: 0 }
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn tab<F>(mut self, label: impl Into<String>, content: F) -> Self
    where
        F: FnMut(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.tabs.push((label.into(), Box::new(content)));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for TabsBuilder {
    type Output = Entity;

    fn spawn(mut self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let selected_idx = self.selected;
        let mut root = parent.spawn((
            TabsMarker,
            TabsState { selected: selected_idx },
            TabsRootStyle::new(theme, self.orientation),
            Name::new("Tabs"),
        ));
        let root_entity = root.id();

        root.with_children(|rc| {
            for (i, (label, _)) in self.tabs.iter().enumerate() {
                rc.spawn((
                    TabTriggerMarker { index: i, tabs_entity: root_entity },
                    Button,
                    Interaction::default(),
                    TabTriggerStyle::new(theme, i == selected_idx),
                    Name::new(format!("TabTrigger{}", i)),
                ))
                .with_children(|tc| {
                    tc.spawn((
                        Text::new(label.clone()),
                        TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                        TextColor(theme.color.slate.step12),
                    ));
                });
            }
            for (i, (_label, mut content_fn)) in self.tabs.into_iter().enumerate() {
                let mut cmd = rc.spawn((
                    TabContentMarker { index: i, tabs_entity: root_entity },
                    TabContentStyle::new(theme),
                    if i == selected_idx { Visibility::Inherited } else { Visibility::Hidden },
                ));
                cmd.with_children(|cb| {
                    (content_fn)(cb, theme, font);
                });
            }
        });

        root.id()
    }
}
