//! Button enumerations module
//!
//! This module defines various enumerations used for button configuration,
//! including visual variants, sizes, and child element types.

use bevy::prelude::*;

/// Defines the visual style variant of a button
///
/// Buttons can have different visual styles which affect their background,
/// border, and text colors. Each variant provides a different level of
/// visual prominence and purpose.
///
/// # Variants
/// * `Solid` - Full background color with high contrast text (default)
/// * `Soft` - Light background color with darker text
/// * `Outline` - Transparent background with border and dark text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonVariant {
    /// Full background color with high contrast text (default)
    #[default]
    Solid,
    /// Light background color with darker text
    Soft,
    /// Transparent background with border and dark text
    Outline,
}

/// Defines the size variant of a button
///
/// Buttons can have different size presets that affect their padding,
/// text size, and overall dimensions.
///
/// # Variants
/// * `Default` - Standard size for most scenarios (default)
/// * `Small` - Compact size for space-constrained areas
/// * `Large` - Expanded size for emphasis or improved touch targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonSize {
    /// Standard size for most scenarios (default)
    #[default]
    Default,
    /// Compact size for space-constrained areas
    Small,
    /// Expanded size for emphasis or improved touch targets
    Large,
}

/// Defines the child element types for a button
///
/// Buttons can contain different types of child elements, such as
/// simple text or custom-spawned entities.
///
/// # Variants
/// * `Text` - Simple text content
/// * `Custom` - Custom closure for spawning arbitrary child elements
pub enum ButtonChild {
    /// Text content
    Text(String),
    /// Custom closure for spawning arbitrary child elements
    Custom(Box<dyn FnOnce(&mut ChildSpawnerCommands) + Send + Sync>),
}
