use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::components::{
    TabContent, TabTrigger, TabsContentMarker, TabsListMarker, TabsMarker, TabsOrientation,
    TabsState, TabsTriggerMarker,
};

pub struct TabDefinition {
    pub label: String,
    pub value: String,
    pub content:
        Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl TabDefinition {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            content: None,
        }
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

pub struct TabsBuilder {
    orientation: TabsOrientation,
    default_value: Option<String>,
    tabs: Vec<TabDefinition>,
}

impl TabsBuilder {
    pub fn new() -> Self {
        Self {
            orientation: TabsOrientation::Horizontal,
            default_value: None,
            tabs: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn add_tab(mut self, tab: TabDefinition) -> Self {
        self.tabs.push(tab);
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for TabsBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let initial_value = self
            .default_value
            .clone()
            .or_else(|| self.tabs.first().map(|t| t.value.clone()));

        let mut root = parent.spawn((
            TabsMarker,
            TabsState {
                active: initial_value.clone(),
                orientation: self.orientation,
            },
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ));

        // Tab list
        root.with_children(|p| {
            p.spawn((
                TabsListMarker,
                Node {
                    display: Display::Flex,
                    flex_direction: match self.orientation {
                        TabsOrientation::Horizontal => FlexDirection::Row,
                        TabsOrientation::Vertical => FlexDirection::Column,
                    },
                    ..default()
                },
            ))
            .with_children(|list| {
                for tab in &self.tabs {
                    list.spawn((
                        TabsTriggerMarker,
                        Button,
                        TabTrigger {
                            value: tab.value.clone(),
                        },
                        Node {
                            padding: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        Interaction::default(),
                    ))
                    .with_children(|c| {
                        c.spawn((
                            Text::new(tab.label.clone()),
                            TextFont {
                                font: font.clone(),
                                font_size: theme.font.size.base,
                                ..default()
                            },
                            TextColor(theme.color.slate.step12),
                        ));
                    });
                }
            });
        });

        // Content nodes
        root.with_children(|p| {
            for tab in self.tabs {
                let vis = if Some(&tab.value) == initial_value.as_ref() {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };

                let mut cmd = p.spawn((
                    TabsContentMarker,
                    TabContent {
                        value: tab.value.clone(),
                    },
                    Node {
                        display: Display::Flex,
                        ..default()
                    },
                    vis,
                ));

                if let Some(content_fn) = tab.content {
                    cmd.with_children(|cb| content_fn(cb, theme, font));
                }
            }
        });

        root.id()
    }
}
