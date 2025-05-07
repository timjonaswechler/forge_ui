// crates/forge_ui/src/plugin.rs
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::assets::{FontAssets, Icon, IconAssets, IconLoader};
use crate::theme::{
    data::UiThemeData,
    systems::{
        check_theme_asset_readiness, hot_reload_theme_system, load_theme_asset, save_theme_system,
    },
    UiTheme,
};
use crate::{
    close_dialog_system, handle_button_clicks_event, handle_button_clicks_fn,
    handle_checkbox_clicks, handle_overlay_click_system, open_dialog_system,
    register_initially_open_dialogs, update_button_visuals, update_checkbox_visuals,
    update_checkmark_visibility_on_state_change, ActiveDialogs, ButtonClickedEvent,
    CheckboxChangedEvent, CloseDialogEvent, OpenDialogEvent,
};

// UI lifecycle phases for Bevy 0.16
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UiState {
    #[default]
    LoadingAssets, // Fonts & Icons laden
    LoadingTheme, // RON-Themes laden/prüfen
    Ready,        // Normale UI-Systeme laufen
    HotReload,    // Theme-HotReload Phase
}

pub struct ForgeUiPlugin;

impl Plugin for ForgeUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // 1) Initialize UiState and set starting variant
            .init_state::<UiState>()
            .insert_state(UiState::LoadingAssets)
            // 2) Asset-Loading: load FontAssets and IconAssets, then go to LoadingTheme
            .add_loading_state(
                LoadingState::new(UiState::LoadingAssets)
                    .continue_to_state(UiState::LoadingTheme)
                    .load_collection::<FontAssets>()
                    .load_collection::<IconAssets>(),
            )
            // 3) Register RON asset type
            .register_asset_reflect::<UiThemeData>()
            .init_asset_loader::<IconLoader>()
            .init_asset::<Icon>()
            .init_asset::<UiThemeData>()
            .add_plugins(RonAssetPlugin::<UiThemeData>::new(&["theme.ron", "theme"]))
            // 4) Theme loading systems in LoadingTheme
            .add_systems(OnEnter(UiState::LoadingTheme), load_theme_asset)
            .add_systems(
                Update,
                check_theme_asset_readiness.run_if(in_state(UiState::LoadingTheme)),
            )
            .add_systems(
                Update,
                (|theme: Option<Res<UiTheme>>, mut next: ResMut<NextState<UiState>>| {
                    if theme.is_some() {
                        next.set(UiState::Ready);
                    }
                })
                .run_if(in_state(UiState::LoadingTheme)),
            )
            // 5) Normal UI systems in Ready
            .add_event::<ButtonClickedEvent>()
            .add_event::<CheckboxChangedEvent>()
            .add_event::<OpenDialogEvent>()
            .add_event::<CloseDialogEvent>()
            .insert_resource(ActiveDialogs::default())
            .add_systems(
                Update,
                (
                    update_button_visuals.run_if(in_state(UiState::Ready)),
                    handle_button_clicks_event.run_if(in_state(UiState::Ready)),
                    update_button_visuals.run_if(in_state(UiState::Ready)),
                    handle_button_clicks_event.run_if(in_state(UiState::Ready)),
                    handle_button_clicks_fn.run_if(in_state(UiState::Ready)),
                    update_checkbox_visuals.run_if(in_state(UiState::Ready)),
                    handle_checkbox_clicks.run_if(in_state(UiState::Ready)),
                    update_checkmark_visibility_on_state_change.run_if(in_state(UiState::Ready)),
                    open_dialog_system.run_if(in_state(UiState::Ready)),
                    handle_overlay_click_system.run_if(in_state(UiState::Ready)),
                    close_dialog_system
                        .run_if(in_state(UiState::Ready))
                        .run_if(|active: Res<ActiveDialogs>| active.current_modal.is_some()),
                    register_initially_open_dialogs.run_if(in_state(UiState::Ready)),
                ),
            )
            // Debug: Save theme on S key
            .add_systems(
                Update,
                save_theme_system.run_if(in_state(UiState::Ready)).run_if(
                    bevy::input::common_conditions::input_just_pressed(KeyCode::KeyS),
                ),
            )
            // 6) HotReload cycle: detect & trigger in Ready, process in HotReload
            .add_systems(
                Update,
                // detect file change and switch to HotReload
                (|mut _next: ResMut<NextState<UiState>>| {
                    // insert file watcher logic here, then:
                    // next.set(UiState::HotReload);
                })
                .run_if(in_state(UiState::Ready)),
            )
            .add_systems(
                Update,
                (
                    hot_reload_theme_system,
                    |mut next: ResMut<NextState<UiState>>| next.set(UiState::Ready),
                )
                    .run_if(in_state(UiState::HotReload)),
            );

        info!("ForgeUiPlugin loaded. UiState={:?}", app.plugins_state());
    }
}
