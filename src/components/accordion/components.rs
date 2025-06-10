use bevy::prelude::*;

/// Marker for the root entity of an accordion item.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AccordionMarker;

/// Marker for the clickable header of an accordion item.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AccordionHeaderMarker;

/// Marker for the expandable body of an accordion item.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AccordionBodyMarker;

/// Runtime state of an accordion item.
#[derive(Component, Debug, Clone, Copy)]
pub struct AccordionState {
    pub open: bool,
    pub disabled: bool,
}

impl Default for AccordionState {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
        }
    }
}
