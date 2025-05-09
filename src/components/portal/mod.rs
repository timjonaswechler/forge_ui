// src/components/portal/mod.rs

mod builder;
mod components;
mod systems;

pub(crate) use builder::PortalContentBuilder; // Öffentlich zugänglich
pub(crate) use components::ForgeUiPortalRoot; // Öffentlich zugänglich
pub(crate) use systems::setup_global_portal_root; // Öffentlich zugänglich
