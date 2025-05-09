// crates/forge_ui/src/plugin.rs
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::assets::{FontAssets, Icon, IconAssets, IconLoader};
use crate::components::{
    button::{
        handle_button_clicks_event, handle_button_clicks_fn, update_button_visuals,
        ButtonClickedEvent,
    },
    checkbox::{
        handle_checkbox_clicks, update_checkbox_visuals,
        update_checkmark_visibility_on_state_change, CheckboxChangedEvent,
    },
    dialog::{
        close_dialog_system, handle_overlay_click_system, open_dialog_system,
        register_initially_open_dialogs, setup_dialog_portal_container, ActiveDialogs,
        CloseDialogEvent, DialogPortalContainer, OpenDialogEvent,
    },
    portal::{setup_global_portal_root, ForgeUiPortalRoot},
    radio::{
        handle_radio_click, update_radio_indicator, update_radio_visuals, OnSelectId,
        OnSelectRegistry,
    },
    switch::{handle_toggle_switch_clicks, update_toggle_switch_visuals, SwitchChangedEvent},
    tabs::{
        handle_tab_triggers, populate_initial_tab_content, update_tabs_visuals, TabChangedEvent,
        TabId,
    },
};
use crate::theme::{
    data::UiThemeData,
    settings::Appearance,
    systems::{
        check_theme_asset_readiness, hot_reload_theme_system, load_theme_asset, save_theme_system,
    },
    UiTheme,
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

#[derive(Resource, Debug, Clone)]
pub struct UiConfig {
    pub font_size_base: f32,
    pub appearance: Appearance,
    pub high_contrast: bool,
    pub scaling: f32,
    pub spacing_factor: f32,
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            font_size_base: 16.0,
            spacing_factor: 0.25,
            appearance: Appearance::Light,
            high_contrast: false,
            scaling: 1.0,
        }
    }
}

pub struct ForgeUiPlugin {
    pub config: UiConfig,
}

impl ForgeUiPlugin {
    pub fn new() -> Self {
        ForgeUiPlugin {
            config: UiConfig::default(),
        }
    }

    pub fn with_font_size(mut self, font_size_base: f32) -> Self {
        self.config.font_size_base = font_size_base;
        self
    }
}

impl Plugin for ForgeUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // region: 1) Initialize UiState and set starting variant
            .init_state::<UiState>()
            .insert_resource(self.config.clone())
            .insert_state(UiState::LoadingAssets)
            // endregion
            // region: 2) Asset-Loading: load FontAssets and IconAssets, then go to LoadingTheme
            .add_loading_state(
                LoadingState::new(UiState::LoadingAssets)
                    .continue_to_state(UiState::LoadingTheme)
                    .load_collection::<FontAssets>()
                    .load_collection::<IconAssets>(),
            )
            // endregion
            // region: 3) Register RON asset type
            .register_asset_reflect::<UiThemeData>()
            .init_asset_loader::<IconLoader>()
            .init_asset::<Icon>()
            .init_asset::<UiThemeData>()
            .add_plugins(RonAssetPlugin::<UiThemeData>::new(&["theme.ron", "theme"]))
            // endregion
            // region: 4) Theme loading systems in LoadingTheme
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
            // endregion
            // region: 5) Register UI systems in Ready
            // region: --- Badge ---
            // endregion --- Badge ---
            // region: --- Button ---
            .add_event::<ButtonClickedEvent>()
            .add_systems(
                Update,
                (
                    update_button_visuals.run_if(in_state(UiState::Ready)),
                    handle_button_clicks_event.run_if(in_state(UiState::Ready)),
                    handle_button_clicks_fn.run_if(in_state(UiState::Ready)),
                ),
            )
            // endregion --- Button ---
            // region: --- Checkboxen ---
            .add_event::<CheckboxChangedEvent>()
            .add_systems(
                Update,
                (
                    update_checkbox_visuals.run_if(in_state(UiState::Ready)),
                    handle_checkbox_clicks.run_if(in_state(UiState::Ready)),
                    update_checkmark_visibility_on_state_change.run_if(in_state(UiState::Ready)),
                ),
            )
            // endregion --- Checkboxen ---
            // region: --- Toggle Switches ---
            .add_event::<SwitchChangedEvent>()
            .add_systems(
                Update,
                (
                    handle_toggle_switch_clicks.run_if(in_state(UiState::Ready)),
                    update_toggle_switch_visuals.after(handle_toggle_switch_clicks),
                ),
            )
            // endregion --- Toggle Switches ---
            // region: --- Dialoge ---
            .insert_resource(DialogPortalContainer::default())
            .insert_resource(ActiveDialogs::default())
            .add_event::<OpenDialogEvent>()
            .add_event::<CloseDialogEvent>()
            .add_systems(
                Update,
                (
                    setup_dialog_portal_container.run_if(in_state(UiState::Ready)),
                    open_dialog_system.run_if(in_state(UiState::Ready)),
                    handle_overlay_click_system.run_if(in_state(UiState::Ready)),
                    close_dialog_system
                        .run_if(in_state(UiState::Ready))
                        .run_if(|active: Res<ActiveDialogs>| !active.modals.is_empty()),
                    register_initially_open_dialogs.run_if(in_state(UiState::Ready)),
                ),
            )
            // endregion --- Dialoge ---
            // region: --- Radio Buttons ---
            .insert_resource(OnSelectRegistry::default())
            .add_systems(
                Update,
                (
                    update_radio_visuals,
                    handle_radio_click,
                    update_radio_indicator,
                )
                    .run_if(in_state(UiState::Ready)), // Stelle sicher, dass dies korrekt ist für deine State-Logik
            )
            // endregion --- Radio Buttons ---
            // region: --- Tabs ---
            .add_event::<TabChangedEvent<TabId>>() // Use TabValue newtype as the event type
            .add_systems(
                Update,
                (
                    update_tabs_visuals::<TabId>.run_if(in_state(UiState::Ready)),
                    handle_tab_triggers::<TabId>.run_if(in_state(UiState::Ready)),
                    populate_initial_tab_content::<TabId>.run_if(in_state(UiState::Ready)),
                ),
            )
            // endregion --- Tabs ---
            // region: --- Portale ---
            .insert_resource(ForgeUiPortalRoot(Entity::PLACEHOLDER))
            .add_systems(
                Update,
                setup_global_portal_root.run_if(in_state(UiState::Ready)),
            )
            // endregion --- UI-Systeme in Ready ---
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
