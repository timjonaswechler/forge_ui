use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PasswordToggleFieldMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PasswordInputMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PasswordToggleMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PasswordVisibleTextMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PasswordHiddenTextMarker;

#[derive(Component, Debug, Clone)]
pub struct PasswordToggleFieldState {
    pub visible: bool,
    pub password: String,
}

