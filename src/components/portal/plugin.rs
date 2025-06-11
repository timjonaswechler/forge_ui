use bevy::prelude::*;

use super::{components::ForgeUiPortalRoot, systems::setup_global_portal_root};
use crate::plugin::UiState;

/// Plugin for portal support.
///
/// This plugin initializes a [`ForgeUiPortalRoot`] resource and spawns
/// the global UI portal root once the UI enters the [`UiState::Ready`] state.
pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForgeUiPortalRoot::default())
            .add_systems(OnEnter(UiState::Ready), setup_global_portal_root);
    }
}
