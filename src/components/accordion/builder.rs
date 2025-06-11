use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::components::*;
use crate::theme::UiTheme;

/// Builder for a simple accordion item consisting of a header and body.
/// The body visibility is controlled by [`AccordionState`].
pub struct AccordionBuilder {
    title: String,
    open: bool,
    disabled: bool,
    content:
        Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl AccordionBuilder {
    /// Creates a new accordion builder with the given header text.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            open: false,
            disabled: false,
            content: None,
            markers: Vec::new(),
        }
    }

    /// Whether the accordion starts opened.
    pub fn open(mut self, flag: bool) -> Self {
        self.open = flag;
        self
    }

    /// Disable interaction on the accordion.
    pub fn disabled(mut self, flag: bool) -> Self {
        self.disabled = flag;
        self
    }

    /// Provide custom body content.
    pub fn content<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(func));
        self
    }

    /// Add custom markers to the root entity.
    pub fn add_marker<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    {
        self.markers.push(Box::new(func));
        self
    }

    /// Spawn the accordion into the given parent.
    #[must_use]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> EntityCommands<'a> {
        let mut cmd = parent.spawn((
            AccordionMarker,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..default()
            },
            AccordionState {
                open: self.open,
                disabled: self.disabled,
            },
        ));

        // Apply custom markers
        for m in self.markers {
            m(&mut cmd);
        }

        cmd.with_children(|cb| {
            // Header
            cb.spawn((
                AccordionHeaderMarker,
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(theme.color.red.step10),
                if self.disabled {
                    FocusPolicy::Pass
                } else {
                    FocusPolicy::Block
                },
                Interaction::default(),
            ))
            .with_children(|h| {
                h.spawn((
                    Text::new(self.title.clone()),
                    TextFont {
                        font: font.clone(),
                        font_size: theme.font.size.base,
                        ..default()
                    },
                    TextColor(theme.color.gray.step12),
                ));
            });

            // Body
            let body_visibility = if self.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            let mut body_cmd = cb.spawn((
                AccordionBodyMarker,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(theme.color.gray.step01),
                body_visibility,
            ));

            if let Some(content_fn) = self.content {
                body_cmd.with_children(|c| {
                    content_fn(c, theme, font);
                });
            }
        });

        if self.disabled {
            spawn_disabled_overlay(&mut cmd, theme);
        }

        cmd
    }
}

fn spawn_disabled_overlay<'w>(cmd: &mut EntityCommands<'w>, theme: &UiTheme) {
    cmd.with_children(|parent| {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.color.black.step08),
            FocusPolicy::Block,
        ));
    });
}
