use bevy::prelude::*;

use crate::components::base_button::BaseButtonBuilder;
use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{DropdownMenuItemMarker, DropdownMenuItemStyle, DropdownMenuMarker, DropdownMenuStyle};

/// Builder for a simple dropdown menu.
pub struct DropdownMenuBuilder {
    trigger: String,
    items: Vec<String>,
}

impl DropdownMenuBuilder {
    /// Create a new dropdown menu with the given trigger label.
    pub fn new(trigger: impl Into<String>) -> Self {
        Self {
            trigger: trigger.into(),
            items: Vec::new(),
        }
    }

    /// Add a text item to the menu.
    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for DropdownMenuBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let items = self.items;
        let trigger = self.trigger;
        let mut cmd = parent.spawn((
            DropdownMenuMarker,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ));

        cmd.with_children(|cb| {
            BaseButtonBuilder::new()
                .content(|b, theme, font| {
                    let _ = LabelBuilder::new(trigger).spawn(b, theme, font);
                })
                .spawn(cb, theme, font);

            let style = DropdownMenuStyle::new(theme);
            let mut menu_cmd = cb.spawn((style,));
            menu_cmd.with_children(|menu_cb| {
                for item in items {
                    let item_style = DropdownMenuItemStyle::new(theme);
                    let mut item_cmd = menu_cb.spawn((DropdownMenuItemMarker, item_style));
                    item_cmd.with_children(|icb| {
                        let _ = LabelBuilder::new(item.clone()).spawn(icb, theme, font);
                    });
                }
            });
        });

        cmd.id()
    }
}
