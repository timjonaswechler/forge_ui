use bevy::prelude::*;
use std::collections::HashSet;

/// Marker component for the root entity of a checkbox group.
#[derive(Component)]
pub struct CheckboxGroupMarker;

/// Holds runtime state for a checkbox group.
#[derive(Component, Debug, Clone)]
pub struct CheckboxGroupState {
    /// Orientation of the checkboxes inside the group.
    pub orientation: CheckboxGroupOrientation,
    /// Whether all checkboxes are disabled.
    pub disabled: bool,
    /// Set of currently checked values.
    pub checked_values: HashSet<String>,
}

impl Default for CheckboxGroupState {
    fn default() -> Self {
        Self {
            orientation: CheckboxGroupOrientation::Vertical,
            disabled: false,
            checked_values: HashSet::new(),
        }
    }
}

/// Tag component added to each checkbox spawned by the group storing its value.
#[derive(Component, Clone)]
pub struct CheckboxGroupItem {
    pub value: String,
}

/// Orientation of the group container.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxGroupOrientation {
    Horizontal,
    Vertical,
}

impl Default for CheckboxGroupOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}
