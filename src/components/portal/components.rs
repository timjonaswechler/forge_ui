// src/components/portal/components.rs
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ForgeUiPortalRoot(pub Entity);

impl Default for ForgeUiPortalRoot {
    fn default() -> Self {
        // Platzhalter, bis sie durch das Setup-System initialisiert wird
        ForgeUiPortalRoot(Entity::from_raw(u32::MAX)) // Oder eine andere ungültige Standard-Entity
    }
}
