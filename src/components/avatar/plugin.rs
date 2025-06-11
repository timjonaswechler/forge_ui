use bevy::prelude::*;

/// Avatar component plugin.
///
/// Currently no runtime systems are needed but the plugin is
/// provided for consistency and future extensions.
pub struct AvatarPlugin;

impl Plugin for AvatarPlugin {
    fn build(&self, _app: &mut App) {}
}
