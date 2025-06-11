use bevy::prelude::*;

/// Plugin for the Radio Card component.
///
/// Currently this component does not require runtime systems but we provide
/// the plugin for future extensibility and consistency.
pub struct RadioCardsPlugin;

impl Plugin for RadioCardsPlugin {
    fn build(&self, _app: &mut App) {}
}
