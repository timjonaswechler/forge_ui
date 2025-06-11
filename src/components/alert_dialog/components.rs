use bevy::prelude::*;
use uuid::Uuid;

/// Identifier for an AlertDialog instance.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlertDialogId(pub Uuid);

impl AlertDialogId {
    pub fn new_unique() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AlertDialogId {
    fn default() -> Self {
        Self::new_unique()
    }
}

/// Marker component for the root entity of an alert dialog.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct AlertDialogMarker;

/// Tracks whether the dialog is currently open.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct AlertDialogState {
    pub open: bool,
}

/// Internal marker for the confirm button.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ConfirmButtonMarker;

/// Internal marker for the cancel button.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CancelButtonMarker;
