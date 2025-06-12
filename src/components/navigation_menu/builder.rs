use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    NavigationMenuContentMarker, NavigationMenuContentStyle, NavigationMenuItemMarker,
    NavigationMenuItemState, NavigationMenuItemStyle, NavigationMenuLinkMarker,
    NavigationMenuLinkStyle, NavigationMenuMarker, NavigationMenuStyle,
    NavigationMenuTriggerMarker,
};

/// Builder for a navigation menu item with optional submenu links.
pub struct NavigationMenuItemBuilder {
    label: String,
    links: Vec<String>,
}

impl NavigationMenuItemBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            links: Vec::new(),
        }
    }

    /// Add a link label to this item.
    pub fn link(mut self, text: impl Into<String>) -> Self {
        self.links.push(text.into());
        self
    }
}

/// Builder for a navigation menu root element.
pub struct NavigationMenuBuilder {
    items: Vec<NavigationMenuItemBuilder>,
}

impl NavigationMenuBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item to the menu.
    pub fn item(mut self, item: NavigationMenuItemBuilder) -> Self {
        self.items.push(item);
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for NavigationMenuBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut root = parent.spawn((NavigationMenuMarker, NavigationMenuStyle::new(theme), Name::new("NavigationMenu")));
        root.with_children(|rc| {
            for item in self.items {
                let mut item_cmd = rc.spawn((NavigationMenuItemMarker, NavigationMenuItemState { open: false }, NavigationMenuItemStyle::new(theme)));
                item_cmd.with_children(|ic| {
                    ic.spawn((NavigationMenuTriggerMarker, Node { display: Display::Flex, align_items: AlignItems::Center, ..default() }, Interaction::default()))
                        .with_children(|tc| {
                            LabelBuilder::new(item.label.clone()).spawn(tc, theme, font);
                        });

                    let mut content_cmd = ic.spawn((NavigationMenuContentMarker, NavigationMenuContentStyle::new(theme), Visibility::Hidden));
                    content_cmd.with_children(|cc| {
                        for link in &item.links {
                            let mut link_cmd = cc.spawn((NavigationMenuLinkMarker, NavigationMenuLinkStyle::new(theme)));
                            link_cmd.with_children(|lc| {
                                LabelBuilder::new(link.clone()).spawn(lc, theme, font);
                            });
                        }
                    });
                });
            }
        });
        root.id()
    }
}
