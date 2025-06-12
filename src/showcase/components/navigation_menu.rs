use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_navigation_menu_example(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut section = create_variant_section(parent, "NavigationMenu", theme, font);
    section.with_children(|vc| {
        NavigationMenuBuilder::new()
            .item(
                NavigationMenuItemBuilder::new("Products")
                    .link("Item 1")
                    .link("Item 2"),
            )
            .item(
                NavigationMenuItemBuilder::new("About")
                    .link("Team")
                    .link("Contact"),
            )
            .spawn(vc, theme, font);
    });
}
