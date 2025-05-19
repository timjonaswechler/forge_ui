use bevy::prelude::*;

/// Marker component for the Radio‑Group root container entity.
#[derive(Component)]
pub struct RadioGroupMarker;

/// Runtime state of a Radio‑Group. Updated automatically whenever one of its
/// radios becomes checked.
#[derive(Component, Clone)]
pub struct RadioGroupState {
    /// Unique group name (shared with child radios via `RadioGroup(name)`).
    pub name: String,
    /// Currently selected value (`None` if nothing is selected yet).
    pub selected_value: Option<String>,
    /// Whether the entire group is disabled (forwarded to child radios).
    pub disabled: bool,
}

/// Internal marker put on the label entities spawned beside every radio item so
/// we can colour them for `disabled`/`hover` states if desired.
#[derive(Component)]
pub struct RadioGroupLabelMarker;
