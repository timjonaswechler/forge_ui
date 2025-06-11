use bevy::prelude::*;

/// Plugin for the Checkbox Card component.
///
/// Currently, the checkbox card is a simple composition of a [`CheckboxBuilder`]
/// and a [`LabelBuilder`] and does not require any runtime systems or events.
/// The plugin is kept for consistency and future extensibility.
pub struct CheckboxCardsPlugin;

impl Plugin for CheckboxCardsPlugin {
    fn build(&self, _app: &mut App) {
        // No systems or events are required at the moment
    }
}

