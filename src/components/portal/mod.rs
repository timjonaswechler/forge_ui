// src/components/portal/mod.rs

mod builder;
mod components;
mod plugin;
mod systems;

pub(crate) use builder::PortalContentBuilder; // Öffentlich zugänglich
pub use components::ForgeUiPortalRoot; // Öffentlich zugänglich
pub use plugin::PortalPlugin;
