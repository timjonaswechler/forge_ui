// crates/forge_ui/src/plugin.rs
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::assets::{FontAssets, IconAssets};
use crate::camera::CameraPlugin;
use crate::components::helper::*;
use crate::components::{
    accordion::*, alert_dialog::*, avatar::*, button::*, checkbox::*, checkbox_cards::*,
    checkbox_group::*, dialog::*, hover_card::*, menubar::*, navigation_menu::*,
    one_time_password_field::*, password_toggle_field::*, popover::*, portal::*, progress::*,
    radio::*, radio_cards::*, radio_group::*, scroll_area::*, select::*, switch::*, toast::*,
    toggle::*, toggle_group::*, toolbar::*, tooltip::*,
};
use crate::theme::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UiState {
    #[default]
    LoadingAssets,
    LoadingTheme,
    Ready,
    HotReload,
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
            .add_plugins(CameraPlugin)
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
            .add_plugins(ButtonNoActionPlugin::default())
            .add_plugins(DialogPlugin)
            .add_plugins(AlertDialogPlugin)
            .add_plugins(AccordionPlugin)
            .add_plugins(AvatarPlugin)
            .add_plugins(HoverCardPlugin)
            .add_plugins(MenubarPlugin)
            .add_plugins(NavigationMenuPlugin)
            .add_plugins(PopoverPlugin)
            .add_plugins(SelectPlugin)
            .add_plugins(OneTimePasswordFieldPlugin)
            .add_plugins(PasswordToggleFieldPlugin)
            .add_plugins(ToastPlugin)
            .add_plugins(ToolbarPlugin)
            .add_plugins(TooltipPlugin)
            .add_plugins(TogglePlugin::<NoAction>::default())
            .add_plugins(ToggleGroupPlugin::<NoAction>::default())
            .add_plugins(CheckboxPlugin)
            .add_plugins(CheckboxCardsPlugin)
            .add_plugins(RadioCardsPlugin)
            .add_plugins(CheckboxGroupPlugin)
            .add_plugins(RadioGroupPlugin)
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
            // region: --- Portale ---
            .add_plugins(PortalPlugin)
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
