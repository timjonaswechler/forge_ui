use bevy::prelude::*;

use crate::plugin::UiState;

use super::systems::handle_navigation_menu_interaction;

pub struct NavigationMenuPlugin;

impl Default for NavigationMenuPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for NavigationMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_navigation_menu_interaction.run_if(in_state(UiState::Ready)));
    }
}
