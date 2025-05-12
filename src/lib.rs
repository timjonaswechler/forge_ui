// forge_ui/src/lib.rs
mod showcase;
pub use showcase::*;

mod components;
pub use components::*;

mod layout;
pub use layout::*;
mod theme;
pub use theme::*;

mod assets;
pub use assets::*;
mod camera;
use camera::CameraPlugin;
// Haupt-Plugin-Auslagerung
mod plugin;
pub use plugin::{ForgeUiPlugin, UiConfig, UiState}; // UiConfig auch exportieren
