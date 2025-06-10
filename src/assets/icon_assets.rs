//! Collection of icon textures loaded at startup.
//!
//! All files inside the `assets/icons` directory are loaded via
//! [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader) and
//! stored in a simple `HashMap`. Builders which support icons receive a
//! [`Res<IconAssets>`](IconAssets) parameter so they can fetch handles by name:
//!
//! ```rust
//! use forge_ui::prelude::*;
//!
//! fn ui_system(mut commands: Commands, theme: Res<UiTheme>, icons: Res<IconAssets>) {
//!     let check = icons.0.get("check").expect("missing 'check' icon").clone();
//!     commands.spawn(NodeBundle::default()).with_children(|parent| {
//!         BadgeBuilder::new("Checked")
//!             .leading_icon(check)
//!             .spawn(parent, &theme, &theme.font.family.default);
//!     });
//! }
//! ```
//!
//! See the showcase modules for more examples.

use bevy::asset::UntypedHandle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use std::collections::HashMap;

#[derive(Resource)]
pub struct IconAssets(pub HashMap<String, Handle<Image>>);

impl AssetCollection for IconAssets {
    fn create(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("AssetServer missing");
        let mut map = HashMap::new();
        if let Ok(handles) = asset_server.load_folder("icons") {
            for handle in handles {
                if let Some(path) = asset_server.get_handle_path(&handle) {
                    if let Some(stem) = path.path().file_stem().and_then(|s| s.to_str()) {
                        map.insert(stem.to_string(), handle.clone().typed());
                    }
                }
            }
        }
        IconAssets(map)
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("AssetServer missing");
        asset_server.load_folder("icons").unwrap_or_default()
    }
}
