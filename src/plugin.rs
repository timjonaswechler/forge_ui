//src/plugin.rs
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::theme::{
    data::UiThemeData,
    systems::{
        check_theme_asset_readiness, hot_reload_theme_system, load_theme_asset, save_theme_system,
        ThemeAssetHandle,
    },
    UiTheme,
};
use crate::{
    close_dialog_system,
    // Button-Systeme
    handle_button_clicks_event,
    handle_button_clicks_fn,
    // Checkbox-Systeme
    handle_checkbox_clicks,
    handle_overlay_click_system,
    // Dialog-Systeme
    open_dialog_system,
    register_initially_open_dialogs,
    // (Tab- & Card-Systeme bleiben nutzungsspezifisch und extern registriert)
    update_button_visuals,

    update_checkbox_visuals,
    update_checkmark_visibility_on_state_change,

    // Ressourcen
    ActiveDialogs,

    // Events
    ButtonClickedEvent,
    CheckboxChangedEvent,
    CloseDialogEvent,

    OpenDialogEvent,
};
/// Konfiguration für das UI-Plugin (REM-Basiswert)
#[derive(Resource, Debug, Clone)]
pub struct UiConfig {
    pub rem: Option<f32>,
}
impl Default for UiConfig {
    fn default() -> Self {
        UiConfig { rem: None }
    }
}

/// Haupt-Plugin für Forge UI
pub struct ForgeUiPlugin {
    pub config: UiConfig,
}

impl ForgeUiPlugin {
    pub fn new() -> Self {
        ForgeUiPlugin {
            config: UiConfig::default(),
        }
    }

    pub fn with_rem(mut self, rem: f32) -> Self {
        self.config.rem = Some(rem);
        self
    }
}

impl Plugin for ForgeUiPlugin {
    fn build(&self, app: &mut App) {
        // --- Ressourcen & Theme Assets ---
        app.insert_resource(self.config.clone())
            .register_asset_reflect::<UiThemeData>()
            .init_asset::<UiThemeData>()
            .insert_resource(UiConfig {
                rem: self.config.rem,
            })
            .add_plugins(RonAssetPlugin::<UiThemeData>::new(&["theme.ron"]))
            // --- Theme-Lade-Logik ---
            .add_systems(PreStartup, load_theme_asset)
            .add_systems(
                Update,
                check_theme_asset_readiness
                    .run_if(|res: Option<Res<ThemeAssetHandle>>| resource_exists(res))
                    .run_if(not(resource_exists::<UiTheme>)),
            )
            .add_systems(Update, hot_reload_theme_system)
            // --- Button-Systeme ---
            .add_systems(Update, update_button_visuals) // Aktualisiert Farben & Stil :contentReference[oaicite:0]{index=0}:contentReference[oaicite:1]{index=1}
            .add_systems(Update, handle_button_clicks_event) // Event bei Klick :contentReference[oaicite:2]{index=2}
            .add_systems(Update, handle_button_clicks_fn) // Fn-Callback Variante :contentReference[oaicite:4]{index=4}
            // --- Checkbox-Systeme ---
            .add_systems(Update, update_checkbox_visuals) // Styling basierend auf State & Interaction :contentReference[oaicite:6]{index=6}:contentReference[oaicite:7]{index=7}
            .add_systems(Update, handle_checkbox_clicks) // State-Toggle + Event :contentReference[oaicite:8]{index=8}:contentReference[oaicite:9]{index=9}
            .add_systems(Update, update_checkmark_visibility_on_state_change) // Icon Visibility :contentReference[oaicite:10]{index=10}:contentReference[oaicite:11]{index=11}
            // --- Dialog-Systeme ---
            .add_event::<OpenDialogEvent>()
            .add_event::<CloseDialogEvent>()
            .add_event::<ButtonClickedEvent>()
            .add_event::<CheckboxChangedEvent>()
            .init_resource::<ActiveDialogs>() // Modal-Tracking
            .add_systems(Update, open_dialog_system) // Öffnet bei Event :contentReference[oaicite:12]{index=12}:contentReference[oaicite:13]{index=13}
            .add_systems(Update, handle_overlay_click_system) // Overlay-Click schließt :contentReference[oaicite:14]{index=14}:contentReference[oaicite:15]{index=15}
            .add_systems(
                Update,
                close_dialog_system.run_if(|active_dialogs: Res<ActiveDialogs>| {
                    active_dialogs.current_modal.is_some()
                }),
            )
            .add_systems(Update, register_initially_open_dialogs); // Start-Registrierung :contentReference[oaicite:18]{index=18}:contentReference[oaicite:19]{index=19}

        // --- Save Theme (Debug) ---
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            save_theme_system.run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::KeyS,
            )),
        );

        info!("ForgeUiPlugin loaded. Alle generischen UI-Systeme sind nun registriert.");
    }
}
