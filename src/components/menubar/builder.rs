use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::components::label::LabelBuilder;
use crate::theme::UiTheme;

use super::{
    MenubarMarker, MenubarMenuContentMarker, MenubarMenuContentStyle, MenubarMenuItemMarker,
    MenubarMenuItemStyle, MenubarMenuMarker, MenubarMenuState, MenubarMenuStyle,
    MenubarMenuTriggerMarker, MenubarStyle,
};

/// Builder for an individual menu within the menubar.
pub struct MenubarMenuBuilder {
    label: String,
    items: Vec<String>,
}

impl MenubarMenuBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }

    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }
}

/// Builder for a simple menubar.
pub struct MenubarBuilder {
    menus: Vec<MenubarMenuBuilder>,
}

impl MenubarBuilder {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    pub fn menu(mut self, menu: MenubarMenuBuilder) -> Self {
        self.menus.push(menu);
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for MenubarBuilder {
    type Output = Entity;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output {
        let mut root = parent.spawn((
            MenubarMarker,
            MenubarStyle::new(theme),
            Name::new("Menubar"),
        ));
        root.with_children(|rc| {
            for menu in self.menus {
                let mut menu_cmd = rc.spawn((
                    MenubarMenuMarker,
                    MenubarMenuState { open: false },
                    MenubarMenuStyle::new(theme),
                ));
                menu_cmd.with_children(|mc| {
                    mc.spawn((
                        MenubarMenuTriggerMarker,
                        Node {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Interaction::default(),
                    ))
                    .with_children(|tc| {
                        let _ = LabelBuilder::new(menu.label.clone()).spawn(tc, theme, font);
                    });

                    let mut content_cmd = mc.spawn((
                        MenubarMenuContentMarker,
                        MenubarMenuContentStyle::new(theme),
                        Visibility::Hidden,
                    ));
                    content_cmd.with_children(|cc| {
                        for item in &menu.items {
                            let mut item_cmd =
                                cc.spawn((MenubarMenuItemMarker, MenubarMenuItemStyle::new(theme)));
                            item_cmd.with_children(|icb| {
                                let _ = LabelBuilder::new(item.clone()).spawn(icb, theme, font);
                            });
                        }
                    });
                });
            }
        });
        root.id()
    }
}
