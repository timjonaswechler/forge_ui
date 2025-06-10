//! Asset collections used by Forge UI.
//!
//! The [`ForgeUiPlugin`](crate::plugin::ForgeUiPlugin) loads both fonts and
//! icons at startup using `bevy_asset_loader`. Icons can then be accessed via
//! [`IconAssets`], e.g. `icons.0.get("check")`.
//!
//! ```rust
//! use forge_ui::prelude::*;
//!
//! fn system(icons: Res<IconAssets>) {
//!     if let Some(handle) = icons.0.get("cross_1") {
//!         // use `handle` with a builder
//!     }
//! }
//! ```
//
mod fonts;
mod icon_assets;
pub use fonts::FontAssets;
pub use icon_assets::IconAssets;
