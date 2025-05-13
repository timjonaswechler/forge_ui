use super::enums::{ToggleGroupOrientation, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant};
use bevy::prelude::*;
use std::collections::HashSet;
// Simple focus component and event for keyboard navigation
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FocusState;

/// Marker for the main entity of a ToggleGroup
#[derive(Component, Debug, Clone, Copy)]
pub struct ToggleGroupMarker;

/// Marker for items in a ToggleGroup
#[derive(Component, Debug, Clone, Copy)]
pub struct ToggleGroupItemMarker;

/// Runtime state of a ToggleGroup
#[derive(Component, Debug, Clone)]
pub struct ToggleGroupState {
    /// Selection type (single or multiple)
    pub group_type: ToggleGroupType,
    /// Orientation (horizontal or vertical)
    pub orientation: ToggleGroupOrientation,
    /// Whether user interaction is enabled
    pub disabled: bool,
    /// Visual variant of the group
    pub variant: ToggleGroupVariant,
    /// Size variant of the group
    pub size: ToggleGroupSize,
    /// Currently active values (single string or multiple strings)
    pub active_values: HashSet<String>,
    /// Loop when navigating with keyboard
    pub loop_navigation: bool,
    /// Support for roving tabindex accessibility pattern
    pub roving_focus: bool,
}

impl Default for ToggleGroupState {
    fn default() -> Self {
        Self {
            group_type: ToggleGroupType::default(),
            orientation: ToggleGroupOrientation::default(),
            disabled: false,
            variant: ToggleGroupVariant::default(),
            size: ToggleGroupSize::default(),
            active_values: HashSet::new(),
            loop_navigation: true,
            roving_focus: true,
        }
    }
}

/// Runtime state of a ToggleGroupItem
#[derive(Component, Debug, Clone)]
pub struct ToggleGroupItemState {
    /// The string value of this item
    pub value: String,
    /// Is this item pressed/active
    pub pressed: bool,
    /// Is this item disabled (independent from group)
    pub disabled: bool,
}

impl ToggleGroupItemState {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            pressed: false,
            disabled: false,
        }
    }
}
