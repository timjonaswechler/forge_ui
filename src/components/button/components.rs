//! Button components module
//!
//! This module defines the core components used for button entities in the UI system.
//! These components store the state and configuration of buttons and are used by
//! the button systems to determine visual appearance and behavior.

use super::enums::{ButtonSize, ButtonVariant};
use crate::theme::UiColorPalette;
use bevy::prelude::*;
// PhantomData is no longer needed for OnClick

/// Marker component for all buttons created by `ButtonBuilder`.
///
/// This component is automatically added to all buttons created through the button builder
/// and serves as a simple tag for filtering in queries. It allows systems to easily
/// identify and target button entities without needing to check multiple components.
#[derive(Component, Default, Debug)]
pub struct ButtonMarker;

/// Marker component for the disabled overlay of a button.
#[derive(Component, Default, Debug)]
pub struct ButtonDisabledOverlayMarker;

/// Stores the configured state of a button, such as variant, size, and disabled status.
///
/// This component holds all the configurable properties that determine a button's
/// appearance and behavior. It is used by the `update_button_visuals` system to
/// adjust the button's visual representation based on these properties and the
/// current interaction state.
///
/// # Fields
/// * `variant` - The visual style variant of the button (Solid, Soft, Outline)
/// * `size` - The size variant of the button (Default, Small, Large)
/// * `color_palette` - The color palette used for the button's visual elements
/// * `disabled` - Whether the button is currently disabled and should not respond to interactions
#[derive(Component, Debug, Clone)]
pub struct ButtonState {
    /// The visual style variant of the button
    pub variant: ButtonVariant,
    /// The size variant of the button
    pub size: ButtonSize,
    /// The color palette used for the button's visual elements
    pub color_palette: UiColorPalette,
    /// Whether the button is currently disabled
    pub disabled: bool,
}
