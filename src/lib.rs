// forge_ui/src/lib.rs
pub mod components;
pub use components::{
    badge::{
        BadgeBuilder, // Der Builder
        BadgeMarker,  // Der Marker
        BadgeVariant, // Das Varianten-Enum
    },
    button::{
        handle_button_clicks_event, handle_button_clicks_fn, update_button_visuals,
        ButtonClickedEvent,
    },
    // << Enum für die Stile
    card::{
        // -- Builder Pattern --
        CardBuilder,  // << Der Haupt-Builder
        ElementStyle, // << Enum für Text-Stile
        NodeElement,  // << Enum für Kind-Elemente
    },
    checkbox::{
        handle_checkbox_clicks,
        update_checkbox_visuals,
        update_checkmark_visibility_on_state_change,
        CheckboxBuilder,      // Der Builder
        CheckboxChangedEvent, // Das Event bei Änderung
        CheckboxMarker,       // Marker-Komponente
        CheckboxState,        // Die Zustandskomponente
    },
    dialog::{
        close_dialog_system,
        handle_overlay_click_system,
        open_dialog_system,
        register_initially_open_dialogs,
        ActiveDialogs,
        CloseDialogEvent,   // Event zum Schließen
        DialogBuilder,      // Der Builder
        DialogCloseTrigger, // Marker für Schließen-Buttons
        DialogId,           // ID-Komponente
        OpenDialogEvent,    // Event zum Öffnen
    },
    label::{
        LabelBuilder, // << Builder exportieren
        LabelMarker,  // << Marker exportieren
    },
    tabs::{
        handle_tab_triggers,
        populate_initial_tab_content,
        update_tabs_visuals,
        TabChangedEvent, // Event bei Tab-Wechsel
        TabId,           // Re-export TabId for use in this file
        TabsBuilder,     // Der Haupt-Builder
    },
};

pub mod layout;
pub use layout::{
    HorizontalStackBuilder, // Builder für horizontale Stacks
    UiRoot,                 // Marker für die Wurzel (falls benötigt)
    VerticalStackBuilder,   // Builder für vertikale Stacks
};

pub mod theme;
// Theme-Management für UI-Elemente
use bevy::prelude::*;

use crate::theme::{
    data::UiThemeData, // Theme-Daten
    runtime::*,
    // We will move the setup logic into a loading state system
    // systems::{hot_reload_theme_system, setup_theme_resource},
    systems::{hot_reload_theme_system, save_theme_system}, // Keep hot reload system
};

use bevy_common_assets::ron::RonAssetPlugin;

#[derive(Resource, Debug, Clone)]
pub struct UiConfig {
    /// Der REM-Basiswert, in Pixeln. Wenn `None`, wird `ui_scaling` aus theme.ron genutzt.
    pub rem: Option<f32>,
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig { rem: None }
    }
}

/// Plugin für die Kernfunktionalität der Forge UI Widgets.
/// Fügt Interaktionssysteme und Events hinzu.
pub struct ForgeUiPlugin {
    /// Konfiguration für das Plugin.
    pub config: UiConfig,
}

impl ForgeUiPlugin {
    /// Erstelle das Plugin mit Standard-Config (kein Override).
    pub fn new() -> Self {
        ForgeUiPlugin {
            config: UiConfig::default(),
        }
    }

    /// Überschreibe den REM-Wert (in Pixeln).
    pub fn with_rem(mut self, rem: f32) -> Self {
        self.config.rem = Some(rem);
        self
    }
}

impl Plugin for ForgeUiPlugin {
    fn build(&self, app: &mut App) {
        // 1) Config als Resource einfügen
        app.insert_resource(self.config.clone());
        app.register_asset_reflect::<UiThemeData>();
        app.init_asset::<UiThemeData>();
        app.insert_resource(UiConfig {
            rem: self.config.rem,
        });

        app.add_plugins(RonAssetPlugin::<UiThemeData>::new(&["theme.ron"]))
            // --- Asset Loading Logic ---
            // 1. Load the asset during PreStartup and store the handle
            .add_systems(PreStartup, theme::systems::load_theme_asset)
            // 2. Add a system to check loading state *during* your app's loading phase.
            //    Replace 'AppState::Loading' with your actual loading state enum variant.
            //    This system will insert the UiTheme resource once loading is done.
            //    If your app doesn't have a loading state, you might need to add one
            //    or run this check repeatedly in Update until the resource is inserted.
            .add_systems(
                Update,
                theme::systems::check_theme_asset_readiness
                    .run_if(resource_exists::<theme::systems::ThemeAssetHandle>) // Only run if handle exists
                    .run_if(not(resource_exists::<UiTheme>)), // Only run if UiTheme not yet inserted
                                                              // Optional: Run only during a specific loading state
                                                              // .run_if(in_state(AppState::Loading)) // Replace AppState::Loading with your state
            )
            // --- End Asset Loading Logic ---
            .add_systems(Update, hot_reload_theme_system) // Keep hot reload
            .add_event::<CheckboxChangedEvent>()
            // --- NEU: Tabs spezifisch (Generisch über Typ T) ---
            // Man muss das Event und die Systeme für JEDEN verwendeten Wert-Typ T registrieren.
            // Hier ein Beispiel für TabId als Wert-Typ:
            .add_event::<TabChangedEvent<TabId>>() // <-- Beispiel mit TabId
            .init_resource::<ActiveDialogs>() // WICHTIG: Ressource initialisieren
            .add_event::<OpenDialogEvent>()
            .add_event::<CloseDialogEvent>()
            .add_event::<ButtonClickedEvent>()
            .add_systems(
                PostUpdate,
                (
                    // PostUpdate, um Inhalt nach dem Bauen zu füllen
                    populate_initial_tab_content::<TabId>, // <-- Beispiel mit TabId
                ),
            )
            .add_systems(
                Update,
                (
                    handle_tab_triggers::<TabId>, // <-- Beispiel mit TabId
                    handle_button_clicks_event,
                    handle_checkbox_clicks,
                    handle_overlay_click_system,
                    close_dialog_system,
                ),
            );
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            save_theme_system.run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::KeyS,
            )),
        );
        // ----------------------------------------------------
        // TODO: Wenn Sie andere Typen als Tab-Werte verwenden (z.B. Enums),
        //       müssen Sie das Event und die Systeme dafür separat hinzufügen!
        //       Alternative: Komplexeres Plugin mit Typ-Registrierung.
        info!("ForgeUiPlugin loaded. (Systems must be added by app)");
    }
}

// Zukünftige Erweiterungen (Card, Tabs etc.) würden hier ebenfalls ihre Events
// registrieren und ihre Update-Systeme hinzufügen.
// z.B.:
// pub mod card;
// pub use card::{CardBuilder, CardMarker, CardClickedEvent};
// use card::{update_card_visuals, handle_card_clicks};
// ... und im build():
// .add_event::<CardClickedEvent>()
// .add_systems(Update, (update_card_visuals, handle_card_clicks))
