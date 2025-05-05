// crates/forge_ui/src/theme/systems.rs

use crate::theme::{data::*, runtime::*, UiTheme};
use crate::UiConfig;
use bevy::{asset::LoadState, prelude::*};

// Define a handle resource to track the theme asset
#[derive(Resource)]
pub struct ThemeAssetHandle(Handle<UiThemeData>);

// System to load the asset handle during PreStartup
pub fn load_theme_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("theme/default.theme.ron");
    commands.insert_resource(ThemeAssetHandle(handle));
    info!("Initiated loading theme/default.theme.ron");
}

// Keep hot_reload_theme_system
pub fn hot_reload_theme_system(
    mut ev_asset: EventReader<AssetEvent<UiThemeData>>,
    asset_server: Res<AssetServer>,
    theme_data_assets: Res<Assets<UiThemeData>>,
    theme_opt: Option<ResMut<UiTheme>>,
) {
    // --- ADD GUARD ---
    let Some(mut theme) = theme_opt else {
        // Theme resource doesn't exist yet (e.g., initial load failed or hasn't happened)
        // We can just ignore asset modification events in this case.
        // Clear the events to avoid processing them again later if the theme loads.
        ev_asset.clear();
        return;
    };
    // --- END GUARD --
    for event in ev_asset.read() {
        match event {
            AssetEvent::Modified { id } => {
                info!("UiThemeData asset modified: {:?}. Reloading...", id);
                if let Some(data) = theme_data_assets.get(*id) {
                    // --- Werte für Neuberechnung holen ---
                    // Wichtig: Benutze den *gespeicherten* rem-Wert aus dem Theme!
                    let effective_rem = theme.rem;
                    // Nimm den *neuen* ui_scaling-Wert aus den geladenen Daten
                    let effective_ui_scaling = data.ui_scaling;
                    // Berechne die neue Basis-Spacing-Einheit
                    let base_spacing_unit = data.spacing * effective_rem * effective_ui_scaling;

                    info!(
                        "Hot Reload Effective Values: rem={}, ui_scaling={}, base_spacing_unit={}",
                        effective_rem, effective_ui_scaling, base_spacing_unit
                    );
                    // --- Felder in `theme` aktualisieren ---
                    theme.ui_scaling = effective_ui_scaling; // Neuen Skalierungsfaktor speichern
                                                             // theme.rem bleibt unverändert!
                                                             // Re-run the conversion logic, but update the existing resource

                    let load_font = |path: &str| -> Handle<Font> {
                        if path.is_empty() {
                            Handle::default()
                        } else {
                            asset_server.load(path)
                        }
                    };
                    let conv_color = |c: [f32; 4]| Color::srgba(c[0], c[1], c[2], c[3]);

                    // Update typography
                    theme.font.font_size = UiFontSize {
                        /* ... copy from plugin's check_theme_asset_readiness ... */
                        xs: data.font.font_size.xs * effective_rem * effective_ui_scaling,
                        sm: data.font.font_size.sm * effective_rem * effective_ui_scaling,
                        base: data.font.font_size.base * effective_rem * effective_ui_scaling,
                        lg: data.font.font_size.lg * effective_rem * effective_ui_scaling,
                        xl: data.font.font_size.xl * effective_rem * effective_ui_scaling,
                        x2l: data.font.font_size.x2l * effective_rem * effective_ui_scaling,
                        x3l: data.font.font_size.x3l * effective_rem * effective_ui_scaling,
                        x4l: data.font.font_size.x4l * effective_rem * effective_ui_scaling,
                        x5l: data.font.font_size.x5l * effective_rem * effective_ui_scaling,
                        x6l: data.font.font_size.x6l * effective_rem * effective_ui_scaling,
                        x7l: data.font.font_size.x7l * effective_rem * effective_ui_scaling,
                        x8l: data.font.font_size.x8l * effective_rem * effective_ui_scaling,
                        x9l: data.font.font_size.x9l * effective_rem * effective_ui_scaling,
                    };
                    theme.font.font_family = UiFontFamilies {
                        /* ... copy from plugin's check_theme_asset_readiness ... */
                        default: load_font(&data.font.font_family.default),
                        sans: FontVariants {
                            light: load_font(&data.font.font_family.sans.light),
                            light_italic: load_font(&data.font.font_family.sans.light_italic),
                            regular: load_font(&data.font.font_family.sans.regular),
                            regular_italic: load_font(&data.font.font_family.sans.regular_italic),
                            medium: load_font(&data.font.font_family.sans.medium),
                            medium_italic: load_font(&data.font.font_family.sans.medium_italic),
                            bold: load_font(&data.font.font_family.sans.bold),
                            bold_italic: load_font(&data.font.font_family.sans.bold_italic),
                        },
                        serif: FontVariants {
                            light: load_font(&data.font.font_family.serif.light),
                            light_italic: load_font(&data.font.font_family.serif.light_italic),
                            regular: load_font(&data.font.font_family.serif.regular),
                            regular_italic: load_font(&data.font.font_family.serif.regular_italic),
                            medium: load_font(&data.font.font_family.serif.medium),
                            medium_italic: load_font(&data.font.font_family.serif.medium_italic),
                            bold: load_font(&data.font.font_family.serif.bold),
                            bold_italic: load_font(&data.font.font_family.serif.bold_italic),
                        },
                        mono: FontVariants {
                            light: load_font(&data.font.font_family.mono.light),
                            light_italic: load_font(&data.font.font_family.mono.light_italic),
                            regular: load_font(&data.font.font_family.mono.regular),
                            regular_italic: load_font(&data.font.font_family.mono.regular_italic),
                            medium: load_font(&data.font.font_family.mono.medium),
                            medium_italic: load_font(&data.font.font_family.mono.medium_italic),
                            bold: load_font(&data.font.font_family.mono.bold),
                            bold_italic: load_font(&data.font.font_family.mono.bold_italic),
                        },
                    };
                    // Update layout
                    theme.layout = UiLayout {
                        /* ... copy from plugin's check_theme_asset_readiness ... */
                        spacing: base_spacing_unit,
                        padding: UiSpacing {
                            xs: data.layout.padding.xs * base_spacing_unit,
                            sm: data.layout.padding.sm * base_spacing_unit,
                            base: data.layout.padding.base * base_spacing_unit,
                            lg: data.layout.padding.lg * base_spacing_unit,
                            xl: data.layout.padding.xl * base_spacing_unit,
                            x2l: data.layout.padding.x2l * base_spacing_unit,
                            x3l: data.layout.padding.x3l * base_spacing_unit,
                            x4l: data.layout.padding.x4l * base_spacing_unit,
                            x5l: data.layout.padding.x5l * base_spacing_unit,
                        },
                        margin: UiSpacing {
                            xs: data.layout.margin.xs * base_spacing_unit,
                            sm: data.layout.margin.sm * base_spacing_unit,
                            base: data.layout.margin.base * base_spacing_unit,
                            lg: data.layout.margin.lg * base_spacing_unit,
                            xl: data.layout.margin.xl * base_spacing_unit,
                            x2l: data.layout.margin.x2l * base_spacing_unit,
                            x3l: data.layout.margin.x3l * base_spacing_unit,
                            x4l: data.layout.margin.x4l * base_spacing_unit,
                            x5l: data.layout.margin.x5l * base_spacing_unit,
                        },
                        gap: UiSpacing {
                            xs: data.layout.gap.xs * base_spacing_unit,
                            sm: data.layout.gap.sm * base_spacing_unit,
                            base: data.layout.gap.base * base_spacing_unit,
                            lg: data.layout.gap.lg * base_spacing_unit,
                            xl: data.layout.gap.xl * base_spacing_unit,
                            x2l: data.layout.gap.x2l * base_spacing_unit,
                            x3l: data.layout.gap.x3l * base_spacing_unit,
                            x4l: data.layout.gap.x4l * base_spacing_unit,
                            x5l: data.layout.gap.x5l * base_spacing_unit,
                        },
                        radius: UiRadius {
                            xs: data.layout.radius.xs * base_spacing_unit,
                            sm: data.layout.radius.sm * base_spacing_unit,
                            base: data.layout.radius.base * base_spacing_unit,
                            lg: data.layout.radius.lg * base_spacing_unit,
                            xl: data.layout.radius.xl * base_spacing_unit,
                            x2l: data.layout.radius.x2l * base_spacing_unit,
                            x3l: data.layout.radius.x3l * base_spacing_unit,
                            x4l: data.layout.radius.x4l * base_spacing_unit,
                            full: if data.layout.radius.full > 0.0
                                && data.layout.radius.full < f32::MAX / 2.0
                            {
                                data.layout.radius.full * effective_rem * effective_ui_scaling
                            } else {
                                9999.0
                            },
                        },
                        border: UiSpacing {
                            xs: data.layout.border.xs,
                            sm: data.layout.border.sm,
                            base: data.layout.border.base,
                            lg: data.layout.border.lg,
                            xl: data.layout.border.xl,
                            x2l: data.layout.border.x2l,
                            x3l: data.layout.border.x3l,
                            x4l: data.layout.border.x4l,
                            x5l: data.layout.border.x5l,
                        },
                    };
                    // Update colors
                    theme.color = UiColors {
                        /* ... copy from plugin's check_theme_asset_readiness ... */
                        white: UiColor {
                            background_primary: conv_color(data.color.white.background_primary),
                            background_secondary: conv_color(data.color.white.background_secondary),
                            interaction_primary: conv_color(data.color.white.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.white.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.white.interaction_tertiary),
                            border_primary: conv_color(data.color.white.border_primary),
                            border_secondary: conv_color(data.color.white.border_secondary),
                            border_tertiary: conv_color(data.color.white.border_tertiary),
                            solid_primary: conv_color(data.color.white.solid_primary),
                            solid_secondary: conv_color(data.color.white.solid_secondary),
                            text_primary: conv_color(data.color.white.text_primary),
                            text_secondary: conv_color(data.color.white.text_secondary),
                        },
                        black: UiColor {
                            background_primary: conv_color(data.color.black.background_primary),
                            background_secondary: conv_color(data.color.black.background_secondary),
                            interaction_primary: conv_color(data.color.black.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.black.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.black.interaction_tertiary),
                            border_primary: conv_color(data.color.black.border_primary),
                            border_secondary: conv_color(data.color.black.border_secondary),
                            border_tertiary: conv_color(data.color.black.border_tertiary),
                            solid_primary: conv_color(data.color.black.solid_primary),
                            solid_secondary: conv_color(data.color.black.solid_secondary),
                            text_primary: conv_color(data.color.black.text_primary),
                            text_secondary: conv_color(data.color.black.text_secondary),
                        },
                        gray: UiColor {
                            background_primary: conv_color(data.color.gray.background_primary),
                            background_secondary: conv_color(data.color.gray.background_secondary),
                            interaction_primary: conv_color(data.color.gray.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.gray.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.gray.interaction_tertiary),
                            border_primary: conv_color(data.color.gray.border_primary),
                            border_secondary: conv_color(data.color.gray.border_secondary),
                            border_tertiary: conv_color(data.color.gray.border_tertiary),
                            solid_primary: conv_color(data.color.gray.solid_primary),
                            solid_secondary: conv_color(data.color.gray.solid_secondary),
                            text_primary: conv_color(data.color.gray.text_primary),
                            text_secondary: conv_color(data.color.gray.text_secondary),
                        },
                        mauve: UiColor {
                            background_primary: conv_color(data.color.mauve.background_primary),
                            background_secondary: conv_color(data.color.mauve.background_secondary),
                            interaction_primary: conv_color(data.color.mauve.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.mauve.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.mauve.interaction_tertiary),
                            border_primary: conv_color(data.color.mauve.border_primary),
                            border_secondary: conv_color(data.color.mauve.border_secondary),
                            border_tertiary: conv_color(data.color.mauve.border_tertiary),
                            solid_primary: conv_color(data.color.mauve.solid_primary),
                            solid_secondary: conv_color(data.color.mauve.solid_secondary),
                            text_primary: conv_color(data.color.mauve.text_primary),
                            text_secondary: conv_color(data.color.mauve.text_secondary),
                        },
                        slate: UiColor {
                            background_primary: conv_color(data.color.slate.background_primary),
                            background_secondary: conv_color(data.color.slate.background_secondary),
                            interaction_primary: conv_color(data.color.slate.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.slate.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.slate.interaction_tertiary),
                            border_primary: conv_color(data.color.slate.border_primary),
                            border_secondary: conv_color(data.color.slate.border_secondary),
                            border_tertiary: conv_color(data.color.slate.border_tertiary),
                            solid_primary: conv_color(data.color.slate.solid_primary),
                            solid_secondary: conv_color(data.color.slate.solid_secondary),
                            text_primary: conv_color(data.color.slate.text_primary),
                            text_secondary: conv_color(data.color.slate.text_secondary),
                        },
                        sage: UiColor {
                            background_primary: conv_color(data.color.sage.background_primary),
                            background_secondary: conv_color(data.color.sage.background_secondary),
                            interaction_primary: conv_color(data.color.sage.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.sage.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.sage.interaction_tertiary),
                            border_primary: conv_color(data.color.sage.border_primary),
                            border_secondary: conv_color(data.color.sage.border_secondary),
                            border_tertiary: conv_color(data.color.sage.border_tertiary),
                            solid_primary: conv_color(data.color.sage.solid_primary),
                            solid_secondary: conv_color(data.color.sage.solid_secondary),
                            text_primary: conv_color(data.color.sage.text_primary),
                            text_secondary: conv_color(data.color.sage.text_secondary),
                        },
                        olive: UiColor {
                            background_primary: conv_color(data.color.olive.background_primary),
                            background_secondary: conv_color(data.color.olive.background_secondary),
                            interaction_primary: conv_color(data.color.olive.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.olive.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.olive.interaction_tertiary),
                            border_primary: conv_color(data.color.olive.border_primary),
                            border_secondary: conv_color(data.color.olive.border_secondary),
                            border_tertiary: conv_color(data.color.olive.border_tertiary),
                            solid_primary: conv_color(data.color.olive.solid_primary),
                            solid_secondary: conv_color(data.color.olive.solid_secondary),
                            text_primary: conv_color(data.color.olive.text_primary),
                            text_secondary: conv_color(data.color.olive.text_secondary),
                        },
                        sand: UiColor {
                            background_primary: conv_color(data.color.sand.background_primary),
                            background_secondary: conv_color(data.color.sand.background_secondary),
                            interaction_primary: conv_color(data.color.sand.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.sand.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.sand.interaction_tertiary),
                            border_primary: conv_color(data.color.sand.border_primary),
                            border_secondary: conv_color(data.color.sand.border_secondary),
                            border_tertiary: conv_color(data.color.sand.border_tertiary),
                            solid_primary: conv_color(data.color.sand.solid_primary),
                            solid_secondary: conv_color(data.color.sand.solid_secondary),
                            text_primary: conv_color(data.color.sand.text_primary),
                            text_secondary: conv_color(data.color.sand.text_secondary),
                        },
                        tomato: UiColor {
                            background_primary: conv_color(data.color.tomato.background_primary),
                            background_secondary: conv_color(
                                data.color.tomato.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.tomato.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.tomato.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.tomato.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.tomato.border_primary),
                            border_secondary: conv_color(data.color.tomato.border_secondary),
                            border_tertiary: conv_color(data.color.tomato.border_tertiary),
                            solid_primary: conv_color(data.color.tomato.solid_primary),
                            solid_secondary: conv_color(data.color.tomato.solid_secondary),
                            text_primary: conv_color(data.color.tomato.text_primary),
                            text_secondary: conv_color(data.color.tomato.text_secondary),
                        },
                        red: UiColor {
                            background_primary: conv_color(data.color.red.background_primary),
                            background_secondary: conv_color(data.color.red.background_secondary),
                            interaction_primary: conv_color(data.color.red.interaction_primary),
                            interaction_secondary: conv_color(data.color.red.interaction_secondary),
                            interaction_tertiary: conv_color(data.color.red.interaction_tertiary),
                            border_primary: conv_color(data.color.red.border_primary),
                            border_secondary: conv_color(data.color.red.border_secondary),
                            border_tertiary: conv_color(data.color.red.border_tertiary),
                            solid_primary: conv_color(data.color.red.solid_primary),
                            solid_secondary: conv_color(data.color.red.solid_secondary),
                            text_primary: conv_color(data.color.red.text_primary),
                            text_secondary: conv_color(data.color.red.text_secondary),
                        },
                        ruby: UiColor {
                            background_primary: conv_color(data.color.ruby.background_primary),
                            background_secondary: conv_color(data.color.ruby.background_secondary),
                            interaction_primary: conv_color(data.color.ruby.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.ruby.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.ruby.interaction_tertiary),
                            border_primary: conv_color(data.color.ruby.border_primary),
                            border_secondary: conv_color(data.color.ruby.border_secondary),
                            border_tertiary: conv_color(data.color.ruby.border_tertiary),
                            solid_primary: conv_color(data.color.ruby.solid_primary),
                            solid_secondary: conv_color(data.color.ruby.solid_secondary),
                            text_primary: conv_color(data.color.ruby.text_primary),
                            text_secondary: conv_color(data.color.ruby.text_secondary),
                        },
                        crimson: UiColor {
                            background_primary: conv_color(data.color.crimson.background_primary),
                            background_secondary: conv_color(
                                data.color.crimson.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.crimson.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.crimson.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.crimson.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.crimson.border_primary),
                            border_secondary: conv_color(data.color.crimson.border_secondary),
                            border_tertiary: conv_color(data.color.crimson.border_tertiary),
                            solid_primary: conv_color(data.color.crimson.solid_primary),
                            solid_secondary: conv_color(data.color.crimson.solid_secondary),
                            text_primary: conv_color(data.color.crimson.text_primary),
                            text_secondary: conv_color(data.color.crimson.text_secondary),
                        },
                        pink: UiColor {
                            background_primary: conv_color(data.color.pink.background_primary),
                            background_secondary: conv_color(data.color.pink.background_secondary),
                            interaction_primary: conv_color(data.color.pink.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.pink.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.pink.interaction_tertiary),
                            border_primary: conv_color(data.color.pink.border_primary),
                            border_secondary: conv_color(data.color.pink.border_secondary),
                            border_tertiary: conv_color(data.color.pink.border_tertiary),
                            solid_primary: conv_color(data.color.pink.solid_primary),
                            solid_secondary: conv_color(data.color.pink.solid_secondary),
                            text_primary: conv_color(data.color.pink.text_primary),
                            text_secondary: conv_color(data.color.pink.text_secondary),
                        },
                        plum: UiColor {
                            background_primary: conv_color(data.color.plum.background_primary),
                            background_secondary: conv_color(data.color.plum.background_secondary),
                            interaction_primary: conv_color(data.color.plum.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.plum.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.plum.interaction_tertiary),
                            border_primary: conv_color(data.color.plum.border_primary),
                            border_secondary: conv_color(data.color.plum.border_secondary),
                            border_tertiary: conv_color(data.color.plum.border_tertiary),
                            solid_primary: conv_color(data.color.plum.solid_primary),
                            solid_secondary: conv_color(data.color.plum.solid_secondary),
                            text_primary: conv_color(data.color.plum.text_primary),
                            text_secondary: conv_color(data.color.plum.text_secondary),
                        },
                        purple: UiColor {
                            background_primary: conv_color(data.color.purple.background_primary),
                            background_secondary: conv_color(
                                data.color.purple.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.purple.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.purple.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.purple.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.purple.border_primary),
                            border_secondary: conv_color(data.color.purple.border_secondary),
                            border_tertiary: conv_color(data.color.purple.border_tertiary),
                            solid_primary: conv_color(data.color.purple.solid_primary),
                            solid_secondary: conv_color(data.color.purple.solid_secondary),
                            text_primary: conv_color(data.color.purple.text_primary),
                            text_secondary: conv_color(data.color.purple.text_secondary),
                        },
                        violet: UiColor {
                            background_primary: conv_color(data.color.violet.background_primary),
                            background_secondary: conv_color(
                                data.color.violet.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.violet.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.violet.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.violet.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.violet.border_primary),
                            border_secondary: conv_color(data.color.violet.border_secondary),
                            border_tertiary: conv_color(data.color.violet.border_tertiary),
                            solid_primary: conv_color(data.color.violet.solid_primary),
                            solid_secondary: conv_color(data.color.violet.solid_secondary),
                            text_primary: conv_color(data.color.violet.text_primary),
                            text_secondary: conv_color(data.color.violet.text_secondary),
                        },
                        iris: UiColor {
                            background_primary: conv_color(data.color.iris.background_primary),
                            background_secondary: conv_color(data.color.iris.background_secondary),
                            interaction_primary: conv_color(data.color.iris.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.iris.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.iris.interaction_tertiary),
                            border_primary: conv_color(data.color.iris.border_primary),
                            border_secondary: conv_color(data.color.iris.border_secondary),
                            border_tertiary: conv_color(data.color.iris.border_tertiary),
                            solid_primary: conv_color(data.color.iris.solid_primary),
                            solid_secondary: conv_color(data.color.iris.solid_secondary),
                            text_primary: conv_color(data.color.iris.text_primary),
                            text_secondary: conv_color(data.color.iris.text_secondary),
                        },
                        indigo: UiColor {
                            background_primary: conv_color(data.color.indigo.background_primary),
                            background_secondary: conv_color(
                                data.color.indigo.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.indigo.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.indigo.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.indigo.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.indigo.border_primary),
                            border_secondary: conv_color(data.color.indigo.border_secondary),
                            border_tertiary: conv_color(data.color.indigo.border_tertiary),
                            solid_primary: conv_color(data.color.indigo.solid_primary),
                            solid_secondary: conv_color(data.color.indigo.solid_secondary),
                            text_primary: conv_color(data.color.indigo.text_primary),
                            text_secondary: conv_color(data.color.indigo.text_secondary),
                        },
                        blue: UiColor {
                            background_primary: conv_color(data.color.blue.background_primary),
                            background_secondary: conv_color(data.color.blue.background_secondary),
                            interaction_primary: conv_color(data.color.blue.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.blue.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.blue.interaction_tertiary),
                            border_primary: conv_color(data.color.blue.border_primary),
                            border_secondary: conv_color(data.color.blue.border_secondary),
                            border_tertiary: conv_color(data.color.blue.border_tertiary),
                            solid_primary: conv_color(data.color.blue.solid_primary),
                            solid_secondary: conv_color(data.color.blue.solid_secondary),
                            text_primary: conv_color(data.color.blue.text_primary),
                            text_secondary: conv_color(data.color.blue.text_secondary),
                        },
                        cyan: UiColor {
                            background_primary: conv_color(data.color.cyan.background_primary),
                            background_secondary: conv_color(data.color.cyan.background_secondary),
                            interaction_primary: conv_color(data.color.cyan.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.cyan.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.cyan.interaction_tertiary),
                            border_primary: conv_color(data.color.cyan.border_primary),
                            border_secondary: conv_color(data.color.cyan.border_secondary),
                            border_tertiary: conv_color(data.color.cyan.border_tertiary),
                            solid_primary: conv_color(data.color.cyan.solid_primary),
                            solid_secondary: conv_color(data.color.cyan.solid_secondary),
                            text_primary: conv_color(data.color.cyan.text_primary),
                            text_secondary: conv_color(data.color.cyan.text_secondary),
                        },
                        teal: UiColor {
                            background_primary: conv_color(data.color.teal.background_primary),
                            background_secondary: conv_color(data.color.teal.background_secondary),
                            interaction_primary: conv_color(data.color.teal.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.teal.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.teal.interaction_tertiary),
                            border_primary: conv_color(data.color.teal.border_primary),
                            border_secondary: conv_color(data.color.teal.border_secondary),
                            border_tertiary: conv_color(data.color.teal.border_tertiary),
                            solid_primary: conv_color(data.color.teal.solid_primary),
                            solid_secondary: conv_color(data.color.teal.solid_secondary),
                            text_primary: conv_color(data.color.teal.text_primary),
                            text_secondary: conv_color(data.color.teal.text_secondary),
                        },
                        jade: UiColor {
                            background_primary: conv_color(data.color.jade.background_primary),
                            background_secondary: conv_color(data.color.jade.background_secondary),
                            interaction_primary: conv_color(data.color.jade.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.jade.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.jade.interaction_tertiary),
                            border_primary: conv_color(data.color.jade.border_primary),
                            border_secondary: conv_color(data.color.jade.border_secondary),
                            border_tertiary: conv_color(data.color.jade.border_tertiary),
                            solid_primary: conv_color(data.color.jade.solid_primary),
                            solid_secondary: conv_color(data.color.jade.solid_secondary),
                            text_primary: conv_color(data.color.jade.text_primary),
                            text_secondary: conv_color(data.color.jade.text_secondary),
                        },
                        green: UiColor {
                            background_primary: conv_color(data.color.green.background_primary),
                            background_secondary: conv_color(data.color.green.background_secondary),
                            interaction_primary: conv_color(data.color.green.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.green.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.green.interaction_tertiary),
                            border_primary: conv_color(data.color.green.border_primary),
                            border_secondary: conv_color(data.color.green.border_secondary),
                            border_tertiary: conv_color(data.color.green.border_tertiary),
                            solid_primary: conv_color(data.color.green.solid_primary),
                            solid_secondary: conv_color(data.color.green.solid_secondary),
                            text_primary: conv_color(data.color.green.text_primary),
                            text_secondary: conv_color(data.color.green.text_secondary),
                        },
                        grass: UiColor {
                            background_primary: conv_color(data.color.grass.background_primary),
                            background_secondary: conv_color(data.color.grass.background_secondary),
                            interaction_primary: conv_color(data.color.grass.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.grass.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.grass.interaction_tertiary),
                            border_primary: conv_color(data.color.grass.border_primary),
                            border_secondary: conv_color(data.color.grass.border_secondary),
                            border_tertiary: conv_color(data.color.grass.border_tertiary),
                            solid_primary: conv_color(data.color.grass.solid_primary),
                            solid_secondary: conv_color(data.color.grass.solid_secondary),
                            text_primary: conv_color(data.color.grass.text_primary),
                            text_secondary: conv_color(data.color.grass.text_secondary),
                        },
                        bronze: UiColor {
                            background_primary: conv_color(data.color.bronze.background_primary),
                            background_secondary: conv_color(
                                data.color.bronze.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.bronze.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.bronze.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.bronze.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.bronze.border_primary),
                            border_secondary: conv_color(data.color.bronze.border_secondary),
                            border_tertiary: conv_color(data.color.bronze.border_tertiary),
                            solid_primary: conv_color(data.color.bronze.solid_primary),
                            solid_secondary: conv_color(data.color.bronze.solid_secondary),
                            text_primary: conv_color(data.color.bronze.text_primary),
                            text_secondary: conv_color(data.color.bronze.text_secondary),
                        },
                        gold: UiColor {
                            background_primary: conv_color(data.color.gold.background_primary),
                            background_secondary: conv_color(data.color.gold.background_secondary),
                            interaction_primary: conv_color(data.color.gold.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.gold.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.gold.interaction_tertiary),
                            border_primary: conv_color(data.color.gold.border_primary),
                            border_secondary: conv_color(data.color.gold.border_secondary),
                            border_tertiary: conv_color(data.color.gold.border_tertiary),
                            solid_primary: conv_color(data.color.gold.solid_primary),
                            solid_secondary: conv_color(data.color.gold.solid_secondary),
                            text_primary: conv_color(data.color.gold.text_primary),
                            text_secondary: conv_color(data.color.gold.text_secondary),
                        },
                        brown: UiColor {
                            background_primary: conv_color(data.color.brown.background_primary),
                            background_secondary: conv_color(data.color.brown.background_secondary),
                            interaction_primary: conv_color(data.color.brown.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.brown.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.brown.interaction_tertiary),
                            border_primary: conv_color(data.color.brown.border_primary),
                            border_secondary: conv_color(data.color.brown.border_secondary),
                            border_tertiary: conv_color(data.color.brown.border_tertiary),
                            solid_primary: conv_color(data.color.brown.solid_primary),
                            solid_secondary: conv_color(data.color.brown.solid_secondary),
                            text_primary: conv_color(data.color.brown.text_primary),
                            text_secondary: conv_color(data.color.brown.text_secondary),
                        },
                        orange: UiColor {
                            background_primary: conv_color(data.color.orange.background_primary),
                            background_secondary: conv_color(
                                data.color.orange.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.orange.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.orange.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.orange.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.orange.border_primary),
                            border_secondary: conv_color(data.color.orange.border_secondary),
                            border_tertiary: conv_color(data.color.orange.border_tertiary),
                            solid_primary: conv_color(data.color.orange.solid_primary),
                            solid_secondary: conv_color(data.color.orange.solid_secondary),
                            text_primary: conv_color(data.color.orange.text_primary),
                            text_secondary: conv_color(data.color.orange.text_secondary),
                        },
                        amber: UiColor {
                            background_primary: conv_color(data.color.amber.background_primary),
                            background_secondary: conv_color(data.color.amber.background_secondary),
                            interaction_primary: conv_color(data.color.amber.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.amber.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.amber.interaction_tertiary),
                            border_primary: conv_color(data.color.amber.border_primary),
                            border_secondary: conv_color(data.color.amber.border_secondary),
                            border_tertiary: conv_color(data.color.amber.border_tertiary),
                            solid_primary: conv_color(data.color.amber.solid_primary),
                            solid_secondary: conv_color(data.color.amber.solid_secondary),
                            text_primary: conv_color(data.color.amber.text_primary),
                            text_secondary: conv_color(data.color.amber.text_secondary),
                        },
                        yellow: UiColor {
                            background_primary: conv_color(data.color.yellow.background_primary),
                            background_secondary: conv_color(
                                data.color.yellow.background_secondary,
                            ),
                            interaction_primary: conv_color(data.color.yellow.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.yellow.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(
                                data.color.yellow.interaction_tertiary,
                            ),
                            border_primary: conv_color(data.color.yellow.border_primary),
                            border_secondary: conv_color(data.color.yellow.border_secondary),
                            border_tertiary: conv_color(data.color.yellow.border_tertiary),
                            solid_primary: conv_color(data.color.yellow.solid_primary),
                            solid_secondary: conv_color(data.color.yellow.solid_secondary),
                            text_primary: conv_color(data.color.yellow.text_primary),
                            text_secondary: conv_color(data.color.yellow.text_secondary),
                        },
                        lime: UiColor {
                            background_primary: conv_color(data.color.lime.background_primary),
                            background_secondary: conv_color(data.color.lime.background_secondary),
                            interaction_primary: conv_color(data.color.lime.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.lime.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.lime.interaction_tertiary),
                            border_primary: conv_color(data.color.lime.border_primary),
                            border_secondary: conv_color(data.color.lime.border_secondary),
                            border_tertiary: conv_color(data.color.lime.border_tertiary),
                            solid_primary: conv_color(data.color.lime.solid_primary),
                            solid_secondary: conv_color(data.color.lime.solid_secondary),
                            text_primary: conv_color(data.color.lime.text_primary),
                            text_secondary: conv_color(data.color.lime.text_secondary),
                        },
                        mint: UiColor {
                            background_primary: conv_color(data.color.mint.background_primary),
                            background_secondary: conv_color(data.color.mint.background_secondary),
                            interaction_primary: conv_color(data.color.mint.interaction_primary),
                            interaction_secondary: conv_color(
                                data.color.mint.interaction_secondary,
                            ),
                            interaction_tertiary: conv_color(data.color.mint.interaction_tertiary),
                            border_primary: conv_color(data.color.mint.border_primary),
                            border_secondary: conv_color(data.color.mint.border_secondary),
                            border_tertiary: conv_color(data.color.mint.border_tertiary),
                            solid_primary: conv_color(data.color.mint.solid_primary),
                            solid_secondary: conv_color(data.color.mint.solid_secondary),
                            text_primary: conv_color(data.color.mint.text_primary),
                            text_secondary: conv_color(data.color.mint.text_secondary),
                        },
                        sky: UiColor {
                            background_primary: conv_color(data.color.sky.background_primary),
                            background_secondary: conv_color(data.color.sky.background_secondary),
                            interaction_primary: conv_color(data.color.sky.interaction_primary),
                            interaction_secondary: conv_color(data.color.sky.interaction_secondary),
                            interaction_tertiary: conv_color(data.color.sky.interaction_tertiary),
                            border_primary: conv_color(data.color.sky.border_primary),
                            border_secondary: conv_color(data.color.sky.border_secondary),
                            border_tertiary: conv_color(data.color.sky.border_tertiary),
                            solid_primary: conv_color(data.color.sky.solid_primary),
                            solid_secondary: conv_color(data.color.sky.solid_secondary),
                            text_primary: conv_color(data.color.sky.text_primary),
                            text_secondary: conv_color(data.color.sky.text_secondary),
                        },
                    };
                    // Update scaling
                    theme.ui_scaling = data.ui_scaling;

                    info!("UiTheme resource hot reloaded.");
                }
            }
            _ => {} // Ignore other asset events like Created, Removed for now
        }
    }
}

fn font_handle_to_path_string(handle: &Handle<Font>) -> String {
    handle
        .path()
        .map(|p| p.path().to_string_lossy().into_owned()) // Convert Path to String
        .unwrap_or_default() // Return empty string if no path
}

// Helper to convert runtime Color to data [f32; 4]
fn to_data_color(c: &Color) -> [f32; 4] {
    // Use to_srgba() as the original data used hex, implying sRGB
    let srgba = c.to_srgba();
    [srgba.red, srgba.green, srgba.blue, srgba.alpha]
}

#[cfg(debug_assertions)]
pub fn save_theme_system(theme: Res<UiTheme>) {
    info!("Attempting to save theme...");
    // Hole die effektiven Werte aus dem Theme
    let effective_rem = theme.rem;
    let effective_ui_scaling = theme.ui_scaling;

    // Rückwärtsrechnung vermeiden, wenn die Werte 0 sind (um Division durch Null zu verhindern)
    let scale_factor = if effective_rem * effective_ui_scaling != 0.0 {
        effective_rem * effective_ui_scaling
    } else {
        1.0 // Fallback, sollte nicht passieren
    };
    let base_spacing_unit = theme.layout.spacing; // Nimm die gespeicherte absolute Basiseinheit
    let spacing_scale_factor = if base_spacing_unit != 0.0 {
        base_spacing_unit
    } else {
        1.0 // Fallback
    };
    let border_scale_factor = if effective_ui_scaling != 0.0 {
        effective_ui_scaling
    } else {
        1.0 // Fallback
    };

    // Berechne den Basis-Spacing-Multiplikator zurück
    let data_spacing = if scale_factor != 0.0 {
        base_spacing_unit / scale_factor
    } else {
        0.25 // Fallback auf Standard (rem/4)
    };

    let data = UiThemeData {
        ui_scaling: theme.ui_scaling,
        rem: theme.rem,
        spacing: data_spacing,
        font: UiTypographyData {
            font_size: UiFontSizeData {
                // Direct copy is fine for f32 values
                xs: theme.font.font_size.xs / scale_factor,
                sm: theme.font.font_size.sm / scale_factor,
                base: theme.font.font_size.base / scale_factor,
                lg: theme.font.font_size.lg / scale_factor,
                xl: theme.font.font_size.xl / scale_factor,
                x2l: theme.font.font_size.x2l / scale_factor,
                x3l: theme.font.font_size.x3l / scale_factor,
                x4l: theme.font.font_size.x4l / scale_factor,
                x5l: theme.font.font_size.x5l / scale_factor,
                x6l: theme.font.font_size.x6l / scale_factor,
                x7l: theme.font.font_size.x7l / scale_factor,
                x8l: theme.font.font_size.x8l / scale_factor,
                x9l: theme.font.font_size.x9l / scale_factor,
            },
            font_family: UiFontFamiliesData {
                // Convert Handles back to paths
                default: font_handle_to_path_string(&theme.font.font_family.default),
                sans: FontVariantsData {
                    light: font_handle_to_path_string(&theme.font.font_family.sans.light),
                    light_italic: font_handle_to_path_string(
                        &theme.font.font_family.sans.light_italic,
                    ),
                    regular: font_handle_to_path_string(&theme.font.font_family.sans.regular),
                    regular_italic: font_handle_to_path_string(
                        &theme.font.font_family.sans.regular_italic,
                    ),
                    medium: font_handle_to_path_string(&theme.font.font_family.sans.medium),
                    medium_italic: font_handle_to_path_string(
                        &theme.font.font_family.sans.medium_italic,
                    ),
                    bold: font_handle_to_path_string(&theme.font.font_family.sans.bold),
                    bold_italic: font_handle_to_path_string(
                        &theme.font.font_family.sans.bold_italic,
                    ),
                },
                serif: FontVariantsData {
                    light: font_handle_to_path_string(&theme.font.font_family.serif.light),
                    light_italic: font_handle_to_path_string(
                        &theme.font.font_family.serif.light_italic,
                    ),
                    regular: font_handle_to_path_string(&theme.font.font_family.serif.regular),
                    regular_italic: font_handle_to_path_string(
                        &theme.font.font_family.serif.regular_italic,
                    ),
                    medium: font_handle_to_path_string(&theme.font.font_family.serif.medium),
                    medium_italic: font_handle_to_path_string(
                        &theme.font.font_family.serif.medium_italic,
                    ),
                    bold: font_handle_to_path_string(&theme.font.font_family.serif.bold),
                    bold_italic: font_handle_to_path_string(
                        &theme.font.font_family.serif.bold_italic,
                    ),
                },
                mono: FontVariantsData {
                    light: font_handle_to_path_string(&theme.font.font_family.mono.light),
                    light_italic: font_handle_to_path_string(
                        &theme.font.font_family.mono.light_italic,
                    ),
                    regular: font_handle_to_path_string(&theme.font.font_family.mono.regular),
                    regular_italic: font_handle_to_path_string(
                        &theme.font.font_family.mono.regular_italic,
                    ),
                    medium: font_handle_to_path_string(&theme.font.font_family.mono.medium),
                    medium_italic: font_handle_to_path_string(
                        &theme.font.font_family.mono.medium_italic,
                    ),
                    bold: font_handle_to_path_string(&theme.font.font_family.mono.bold),
                    bold_italic: font_handle_to_path_string(
                        &theme.font.font_family.mono.bold_italic,
                    ),
                },
            },
        },
        layout: UiLayoutData {
            // Direct copy
            padding: UiSpacingData {
                xs: theme.layout.padding.xs / spacing_scale_factor,
                sm: theme.layout.padding.sm / spacing_scale_factor,
                base: theme.layout.padding.base / spacing_scale_factor,
                lg: theme.layout.padding.lg / spacing_scale_factor,
                xl: theme.layout.padding.xl / spacing_scale_factor,
                x2l: theme.layout.padding.x2l / spacing_scale_factor,
                x3l: theme.layout.padding.x3l / spacing_scale_factor,
                x4l: theme.layout.padding.x4l / spacing_scale_factor,
                x5l: theme.layout.padding.x5l / spacing_scale_factor,
            },
            margin: UiSpacingData {
                xs: theme.layout.margin.xs / spacing_scale_factor,
                sm: theme.layout.margin.sm / spacing_scale_factor,
                base: theme.layout.margin.base / spacing_scale_factor,
                lg: theme.layout.margin.lg / spacing_scale_factor,
                xl: theme.layout.margin.xl / spacing_scale_factor,
                x2l: theme.layout.margin.x2l / spacing_scale_factor,
                x3l: theme.layout.margin.x3l / spacing_scale_factor,
                x4l: theme.layout.margin.x4l / spacing_scale_factor,
                x5l: theme.layout.margin.x5l / spacing_scale_factor,
            },
            gap: UiSpacingData {
                xs: theme.layout.gap.xs / spacing_scale_factor,
                sm: theme.layout.gap.sm / spacing_scale_factor,
                base: theme.layout.gap.base / spacing_scale_factor,
                lg: theme.layout.gap.lg / spacing_scale_factor,
                xl: theme.layout.gap.xl / spacing_scale_factor,
                x2l: theme.layout.gap.x2l / spacing_scale_factor,
                x3l: theme.layout.gap.x3l / spacing_scale_factor,
                x4l: theme.layout.gap.x4l / spacing_scale_factor,
                x5l: theme.layout.gap.x5l / spacing_scale_factor,
            },
            radius: UiRadiusData {
                xs: theme.layout.radius.xs / scale_factor,
                sm: theme.layout.radius.sm / scale_factor,
                base: theme.layout.radius.base / scale_factor,
                lg: theme.layout.radius.lg / scale_factor,
                xl: theme.layout.radius.xl / scale_factor,
                x2l: theme.layout.radius.x2l / scale_factor,
                x3l: theme.layout.radius.x3l / scale_factor,
                x4l: theme.layout.radius.x4l / scale_factor,
                full: if theme.layout.radius.full >= 9999.0 {
                    f32::MAX
                } else {
                    theme.layout.radius.full / scale_factor
                }, // Annahme: 9
            },
            border: UiSpacingData {
                xs: theme.layout.border.xs / border_scale_factor,
                sm: theme.layout.border.sm / border_scale_factor,
                base: theme.layout.border.base / border_scale_factor,
                lg: theme.layout.border.lg / border_scale_factor,
                xl: theme.layout.border.xl / border_scale_factor,
                x2l: theme.layout.border.x2l / border_scale_factor,
                x3l: theme.layout.border.x3l / border_scale_factor,
                x4l: theme.layout.border.x4l / border_scale_factor,
                x5l: theme.layout.border.x5l / border_scale_factor,
            },
        },
        // Use the correct struct name: UiColorDatas
        color: UiColorDatas {
            // Convert runtime Colors back to data arrays
            white: UiColorData {
                background_primary: to_data_color(&theme.color.white.background_primary),
                background_secondary: to_data_color(&theme.color.white.background_secondary),
                interaction_primary: to_data_color(&theme.color.white.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.white.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.white.interaction_tertiary),
                border_primary: to_data_color(&theme.color.white.border_primary),
                border_secondary: to_data_color(&theme.color.white.border_secondary),
                border_tertiary: to_data_color(&theme.color.white.border_tertiary),
                solid_primary: to_data_color(&theme.color.white.solid_primary),
                solid_secondary: to_data_color(&theme.color.white.solid_secondary),
                text_primary: to_data_color(&theme.color.white.text_primary),
                text_secondary: to_data_color(&theme.color.white.text_secondary),
            },
            black: UiColorData {
                background_primary: to_data_color(&theme.color.black.background_primary),
                background_secondary: to_data_color(&theme.color.black.background_secondary),
                interaction_primary: to_data_color(&theme.color.black.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.black.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.black.interaction_tertiary),
                border_primary: to_data_color(&theme.color.black.border_primary),
                border_secondary: to_data_color(&theme.color.black.border_secondary),
                border_tertiary: to_data_color(&theme.color.black.border_tertiary),
                solid_primary: to_data_color(&theme.color.black.solid_primary),
                solid_secondary: to_data_color(&theme.color.black.solid_secondary),
                text_primary: to_data_color(&theme.color.black.text_primary),
                text_secondary: to_data_color(&theme.color.black.text_secondary),
            },
            gray: UiColorData {
                background_primary: to_data_color(&theme.color.gray.background_primary),
                background_secondary: to_data_color(&theme.color.gray.background_secondary),
                interaction_primary: to_data_color(&theme.color.gray.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.gray.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.gray.interaction_tertiary),
                border_primary: to_data_color(&theme.color.gray.border_primary),
                border_secondary: to_data_color(&theme.color.gray.border_secondary),
                border_tertiary: to_data_color(&theme.color.gray.border_tertiary),
                solid_primary: to_data_color(&theme.color.gray.solid_primary),
                solid_secondary: to_data_color(&theme.color.gray.solid_secondary),
                text_primary: to_data_color(&theme.color.gray.text_primary),
                text_secondary: to_data_color(&theme.color.gray.text_secondary),
            },
            mauve: UiColorData {
                background_primary: to_data_color(&theme.color.mauve.background_primary),
                background_secondary: to_data_color(&theme.color.mauve.background_secondary),
                interaction_primary: to_data_color(&theme.color.mauve.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.mauve.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.mauve.interaction_tertiary),
                border_primary: to_data_color(&theme.color.mauve.border_primary),
                border_secondary: to_data_color(&theme.color.mauve.border_secondary),
                border_tertiary: to_data_color(&theme.color.mauve.border_tertiary),
                solid_primary: to_data_color(&theme.color.mauve.solid_primary),
                solid_secondary: to_data_color(&theme.color.mauve.solid_secondary),
                text_primary: to_data_color(&theme.color.mauve.text_primary),
                text_secondary: to_data_color(&theme.color.mauve.text_secondary),
            },
            slate: UiColorData {
                background_primary: to_data_color(&theme.color.slate.background_primary),
                background_secondary: to_data_color(&theme.color.slate.background_secondary),
                interaction_primary: to_data_color(&theme.color.slate.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.slate.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.slate.interaction_tertiary),
                border_primary: to_data_color(&theme.color.slate.border_primary),
                border_secondary: to_data_color(&theme.color.slate.border_secondary),
                border_tertiary: to_data_color(&theme.color.slate.border_tertiary),
                solid_primary: to_data_color(&theme.color.slate.solid_primary),
                solid_secondary: to_data_color(&theme.color.slate.solid_secondary),
                text_primary: to_data_color(&theme.color.slate.text_primary),
                text_secondary: to_data_color(&theme.color.slate.text_secondary),
            },
            sage: UiColorData {
                background_primary: to_data_color(&theme.color.sage.background_primary),
                background_secondary: to_data_color(&theme.color.sage.background_secondary),
                interaction_primary: to_data_color(&theme.color.sage.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.sage.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.sage.interaction_tertiary),
                border_primary: to_data_color(&theme.color.sage.border_primary),
                border_secondary: to_data_color(&theme.color.sage.border_secondary),
                border_tertiary: to_data_color(&theme.color.sage.border_tertiary),
                solid_primary: to_data_color(&theme.color.sage.solid_primary),
                solid_secondary: to_data_color(&theme.color.sage.solid_secondary),
                text_primary: to_data_color(&theme.color.sage.text_primary),
                text_secondary: to_data_color(&theme.color.sage.text_secondary),
            },
            olive: UiColorData {
                background_primary: to_data_color(&theme.color.olive.background_primary),
                background_secondary: to_data_color(&theme.color.olive.background_secondary),
                interaction_primary: to_data_color(&theme.color.olive.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.olive.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.olive.interaction_tertiary),
                border_primary: to_data_color(&theme.color.olive.border_primary),
                border_secondary: to_data_color(&theme.color.olive.border_secondary),
                border_tertiary: to_data_color(&theme.color.olive.border_tertiary),
                solid_primary: to_data_color(&theme.color.olive.solid_primary),
                solid_secondary: to_data_color(&theme.color.olive.solid_secondary),
                text_primary: to_data_color(&theme.color.olive.text_primary),
                text_secondary: to_data_color(&theme.color.olive.text_secondary),
            },
            sand: UiColorData {
                background_primary: to_data_color(&theme.color.sand.background_primary),
                background_secondary: to_data_color(&theme.color.sand.background_secondary),
                interaction_primary: to_data_color(&theme.color.sand.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.sand.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.sand.interaction_tertiary),
                border_primary: to_data_color(&theme.color.sand.border_primary),
                border_secondary: to_data_color(&theme.color.sand.border_secondary),
                border_tertiary: to_data_color(&theme.color.sand.border_tertiary),
                solid_primary: to_data_color(&theme.color.sand.solid_primary),
                solid_secondary: to_data_color(&theme.color.sand.solid_secondary),
                text_primary: to_data_color(&theme.color.sand.text_primary),
                text_secondary: to_data_color(&theme.color.sand.text_secondary),
            },
            tomato: UiColorData {
                background_primary: to_data_color(&theme.color.tomato.background_primary),
                background_secondary: to_data_color(&theme.color.tomato.background_secondary),
                interaction_primary: to_data_color(&theme.color.tomato.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.tomato.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.tomato.interaction_tertiary),
                border_primary: to_data_color(&theme.color.tomato.border_primary),
                border_secondary: to_data_color(&theme.color.tomato.border_secondary),
                border_tertiary: to_data_color(&theme.color.tomato.border_tertiary),
                solid_primary: to_data_color(&theme.color.tomato.solid_primary),
                solid_secondary: to_data_color(&theme.color.tomato.solid_secondary),
                text_primary: to_data_color(&theme.color.tomato.text_primary),
                text_secondary: to_data_color(&theme.color.tomato.text_secondary),
            },
            red: UiColorData {
                background_primary: to_data_color(&theme.color.red.background_primary),
                background_secondary: to_data_color(&theme.color.red.background_secondary),
                interaction_primary: to_data_color(&theme.color.red.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.red.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.red.interaction_tertiary),
                border_primary: to_data_color(&theme.color.red.border_primary),
                border_secondary: to_data_color(&theme.color.red.border_secondary),
                border_tertiary: to_data_color(&theme.color.red.border_tertiary),
                solid_primary: to_data_color(&theme.color.red.solid_primary),
                solid_secondary: to_data_color(&theme.color.red.solid_secondary),
                text_primary: to_data_color(&theme.color.red.text_primary),
                text_secondary: to_data_color(&theme.color.red.text_secondary),
            },
            ruby: UiColorData {
                background_primary: to_data_color(&theme.color.ruby.background_primary),
                background_secondary: to_data_color(&theme.color.ruby.background_secondary),
                interaction_primary: to_data_color(&theme.color.ruby.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.ruby.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.ruby.interaction_tertiary),
                border_primary: to_data_color(&theme.color.ruby.border_primary),
                border_secondary: to_data_color(&theme.color.ruby.border_secondary),
                border_tertiary: to_data_color(&theme.color.ruby.border_tertiary),
                solid_primary: to_data_color(&theme.color.ruby.solid_primary),
                solid_secondary: to_data_color(&theme.color.ruby.solid_secondary),
                text_primary: to_data_color(&theme.color.ruby.text_primary),
                text_secondary: to_data_color(&theme.color.ruby.text_secondary),
            },
            crimson: UiColorData {
                background_primary: to_data_color(&theme.color.crimson.background_primary),
                background_secondary: to_data_color(&theme.color.crimson.background_secondary),
                interaction_primary: to_data_color(&theme.color.crimson.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.crimson.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.crimson.interaction_tertiary),
                border_primary: to_data_color(&theme.color.crimson.border_primary),
                border_secondary: to_data_color(&theme.color.crimson.border_secondary),
                border_tertiary: to_data_color(&theme.color.crimson.border_tertiary),
                solid_primary: to_data_color(&theme.color.crimson.solid_primary),
                solid_secondary: to_data_color(&theme.color.crimson.solid_secondary),
                text_primary: to_data_color(&theme.color.crimson.text_primary),
                text_secondary: to_data_color(&theme.color.crimson.text_secondary),
            },
            pink: UiColorData {
                background_primary: to_data_color(&theme.color.pink.background_primary),
                background_secondary: to_data_color(&theme.color.pink.background_secondary),
                interaction_primary: to_data_color(&theme.color.pink.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.pink.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.pink.interaction_tertiary),
                border_primary: to_data_color(&theme.color.pink.border_primary),
                border_secondary: to_data_color(&theme.color.pink.border_secondary),
                border_tertiary: to_data_color(&theme.color.pink.border_tertiary),
                solid_primary: to_data_color(&theme.color.pink.solid_primary),
                solid_secondary: to_data_color(&theme.color.pink.solid_secondary),
                text_primary: to_data_color(&theme.color.pink.text_primary),
                text_secondary: to_data_color(&theme.color.pink.text_secondary),
            },
            plum: UiColorData {
                background_primary: to_data_color(&theme.color.plum.background_primary),
                background_secondary: to_data_color(&theme.color.plum.background_secondary),
                interaction_primary: to_data_color(&theme.color.plum.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.plum.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.plum.interaction_tertiary),
                border_primary: to_data_color(&theme.color.plum.border_primary),
                border_secondary: to_data_color(&theme.color.plum.border_secondary),
                border_tertiary: to_data_color(&theme.color.plum.border_tertiary),
                solid_primary: to_data_color(&theme.color.plum.solid_primary),
                solid_secondary: to_data_color(&theme.color.plum.solid_secondary),
                text_primary: to_data_color(&theme.color.plum.text_primary),
                text_secondary: to_data_color(&theme.color.plum.text_secondary),
            },
            purple: UiColorData {
                background_primary: to_data_color(&theme.color.purple.background_primary),
                background_secondary: to_data_color(&theme.color.purple.background_secondary),
                interaction_primary: to_data_color(&theme.color.purple.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.purple.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.purple.interaction_tertiary),
                border_primary: to_data_color(&theme.color.purple.border_primary),
                border_secondary: to_data_color(&theme.color.purple.border_secondary),
                border_tertiary: to_data_color(&theme.color.purple.border_tertiary),
                solid_primary: to_data_color(&theme.color.purple.solid_primary),
                solid_secondary: to_data_color(&theme.color.purple.solid_secondary),
                text_primary: to_data_color(&theme.color.purple.text_primary),
                text_secondary: to_data_color(&theme.color.purple.text_secondary),
            },
            violet: UiColorData {
                background_primary: to_data_color(&theme.color.violet.background_primary),
                background_secondary: to_data_color(&theme.color.violet.background_secondary),
                interaction_primary: to_data_color(&theme.color.violet.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.violet.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.violet.interaction_tertiary),
                border_primary: to_data_color(&theme.color.violet.border_primary),
                border_secondary: to_data_color(&theme.color.violet.border_secondary),
                border_tertiary: to_data_color(&theme.color.violet.border_tertiary),
                solid_primary: to_data_color(&theme.color.violet.solid_primary),
                solid_secondary: to_data_color(&theme.color.violet.solid_secondary),
                text_primary: to_data_color(&theme.color.violet.text_primary),
                text_secondary: to_data_color(&theme.color.violet.text_secondary),
            },
            iris: UiColorData {
                background_primary: to_data_color(&theme.color.iris.background_primary),
                background_secondary: to_data_color(&theme.color.iris.background_secondary),
                interaction_primary: to_data_color(&theme.color.iris.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.iris.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.iris.interaction_tertiary),
                border_primary: to_data_color(&theme.color.iris.border_primary),
                border_secondary: to_data_color(&theme.color.iris.border_secondary),
                border_tertiary: to_data_color(&theme.color.iris.border_tertiary),
                solid_primary: to_data_color(&theme.color.iris.solid_primary),
                solid_secondary: to_data_color(&theme.color.iris.solid_secondary),
                text_primary: to_data_color(&theme.color.iris.text_primary),
                text_secondary: to_data_color(&theme.color.iris.text_secondary),
            },
            indigo: UiColorData {
                background_primary: to_data_color(&theme.color.indigo.background_primary),
                background_secondary: to_data_color(&theme.color.indigo.background_secondary),
                interaction_primary: to_data_color(&theme.color.indigo.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.indigo.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.indigo.interaction_tertiary),
                border_primary: to_data_color(&theme.color.indigo.border_primary),
                border_secondary: to_data_color(&theme.color.indigo.border_secondary),
                border_tertiary: to_data_color(&theme.color.indigo.border_tertiary),
                solid_primary: to_data_color(&theme.color.indigo.solid_primary),
                solid_secondary: to_data_color(&theme.color.indigo.solid_secondary),
                text_primary: to_data_color(&theme.color.indigo.text_primary),
                text_secondary: to_data_color(&theme.color.indigo.text_secondary),
            },
            blue: UiColorData {
                background_primary: to_data_color(&theme.color.blue.background_primary),
                background_secondary: to_data_color(&theme.color.blue.background_secondary),
                interaction_primary: to_data_color(&theme.color.blue.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.blue.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.blue.interaction_tertiary),
                border_primary: to_data_color(&theme.color.blue.border_primary),
                border_secondary: to_data_color(&theme.color.blue.border_secondary),
                border_tertiary: to_data_color(&theme.color.blue.border_tertiary),
                solid_primary: to_data_color(&theme.color.blue.solid_primary),
                solid_secondary: to_data_color(&theme.color.blue.solid_secondary),
                text_primary: to_data_color(&theme.color.blue.text_primary),
                text_secondary: to_data_color(&theme.color.blue.text_secondary),
            },
            cyan: UiColorData {
                background_primary: to_data_color(&theme.color.cyan.background_primary),
                background_secondary: to_data_color(&theme.color.cyan.background_secondary),
                interaction_primary: to_data_color(&theme.color.cyan.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.cyan.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.cyan.interaction_tertiary),
                border_primary: to_data_color(&theme.color.cyan.border_primary),
                border_secondary: to_data_color(&theme.color.cyan.border_secondary),
                border_tertiary: to_data_color(&theme.color.cyan.border_tertiary),
                solid_primary: to_data_color(&theme.color.cyan.solid_primary),
                solid_secondary: to_data_color(&theme.color.cyan.solid_secondary),
                text_primary: to_data_color(&theme.color.cyan.text_primary),
                text_secondary: to_data_color(&theme.color.cyan.text_secondary),
            },
            teal: UiColorData {
                background_primary: to_data_color(&theme.color.teal.background_primary),
                background_secondary: to_data_color(&theme.color.teal.background_secondary),
                interaction_primary: to_data_color(&theme.color.teal.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.teal.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.teal.interaction_tertiary),
                border_primary: to_data_color(&theme.color.teal.border_primary),
                border_secondary: to_data_color(&theme.color.teal.border_secondary),
                border_tertiary: to_data_color(&theme.color.teal.border_tertiary),
                solid_primary: to_data_color(&theme.color.teal.solid_primary),
                solid_secondary: to_data_color(&theme.color.teal.solid_secondary),
                text_primary: to_data_color(&theme.color.teal.text_primary),
                text_secondary: to_data_color(&theme.color.teal.text_secondary),
            },
            jade: UiColorData {
                background_primary: to_data_color(&theme.color.jade.background_primary),
                background_secondary: to_data_color(&theme.color.jade.background_secondary),
                interaction_primary: to_data_color(&theme.color.jade.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.jade.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.jade.interaction_tertiary),
                border_primary: to_data_color(&theme.color.jade.border_primary),
                border_secondary: to_data_color(&theme.color.jade.border_secondary),
                border_tertiary: to_data_color(&theme.color.jade.border_tertiary),
                solid_primary: to_data_color(&theme.color.jade.solid_primary),
                solid_secondary: to_data_color(&theme.color.jade.solid_secondary),
                text_primary: to_data_color(&theme.color.jade.text_primary),
                text_secondary: to_data_color(&theme.color.jade.text_secondary),
            },
            green: UiColorData {
                background_primary: to_data_color(&theme.color.green.background_primary),
                background_secondary: to_data_color(&theme.color.green.background_secondary),
                interaction_primary: to_data_color(&theme.color.green.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.green.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.green.interaction_tertiary),
                border_primary: to_data_color(&theme.color.green.border_primary),
                border_secondary: to_data_color(&theme.color.green.border_secondary),
                border_tertiary: to_data_color(&theme.color.green.border_tertiary),
                solid_primary: to_data_color(&theme.color.green.solid_primary),
                solid_secondary: to_data_color(&theme.color.green.solid_secondary),
                text_primary: to_data_color(&theme.color.green.text_primary),
                text_secondary: to_data_color(&theme.color.green.text_secondary),
            },
            grass: UiColorData {
                background_primary: to_data_color(&theme.color.grass.background_primary),
                background_secondary: to_data_color(&theme.color.grass.background_secondary),
                interaction_primary: to_data_color(&theme.color.grass.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.grass.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.grass.interaction_tertiary),
                border_primary: to_data_color(&theme.color.grass.border_primary),
                border_secondary: to_data_color(&theme.color.grass.border_secondary),
                border_tertiary: to_data_color(&theme.color.grass.border_tertiary),
                solid_primary: to_data_color(&theme.color.grass.solid_primary),
                solid_secondary: to_data_color(&theme.color.grass.solid_secondary),
                text_primary: to_data_color(&theme.color.grass.text_primary),
                text_secondary: to_data_color(&theme.color.grass.text_secondary),
            },
            bronze: UiColorData {
                background_primary: to_data_color(&theme.color.bronze.background_primary),
                background_secondary: to_data_color(&theme.color.bronze.background_secondary),
                interaction_primary: to_data_color(&theme.color.bronze.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.bronze.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.bronze.interaction_tertiary),
                border_primary: to_data_color(&theme.color.bronze.border_primary),
                border_secondary: to_data_color(&theme.color.bronze.border_secondary),
                border_tertiary: to_data_color(&theme.color.bronze.border_tertiary),
                solid_primary: to_data_color(&theme.color.bronze.solid_primary),
                solid_secondary: to_data_color(&theme.color.bronze.solid_secondary),
                text_primary: to_data_color(&theme.color.bronze.text_primary),
                text_secondary: to_data_color(&theme.color.bronze.text_secondary),
            },
            gold: UiColorData {
                background_primary: to_data_color(&theme.color.gold.background_primary),
                background_secondary: to_data_color(&theme.color.gold.background_secondary),
                interaction_primary: to_data_color(&theme.color.gold.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.gold.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.gold.interaction_tertiary),
                border_primary: to_data_color(&theme.color.gold.border_primary),
                border_secondary: to_data_color(&theme.color.gold.border_secondary),
                border_tertiary: to_data_color(&theme.color.gold.border_tertiary),
                solid_primary: to_data_color(&theme.color.gold.solid_primary),
                solid_secondary: to_data_color(&theme.color.gold.solid_secondary),
                text_primary: to_data_color(&theme.color.gold.text_primary),
                text_secondary: to_data_color(&theme.color.gold.text_secondary),
            },
            brown: UiColorData {
                background_primary: to_data_color(&theme.color.brown.background_primary),
                background_secondary: to_data_color(&theme.color.brown.background_secondary),
                interaction_primary: to_data_color(&theme.color.brown.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.brown.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.brown.interaction_tertiary),
                border_primary: to_data_color(&theme.color.brown.border_primary),
                border_secondary: to_data_color(&theme.color.brown.border_secondary),
                border_tertiary: to_data_color(&theme.color.brown.border_tertiary),
                solid_primary: to_data_color(&theme.color.brown.solid_primary),
                solid_secondary: to_data_color(&theme.color.brown.solid_secondary),
                text_primary: to_data_color(&theme.color.brown.text_primary),
                text_secondary: to_data_color(&theme.color.brown.text_secondary),
            },
            orange: UiColorData {
                background_primary: to_data_color(&theme.color.orange.background_primary),
                background_secondary: to_data_color(&theme.color.orange.background_secondary),
                interaction_primary: to_data_color(&theme.color.orange.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.orange.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.orange.interaction_tertiary),
                border_primary: to_data_color(&theme.color.orange.border_primary),
                border_secondary: to_data_color(&theme.color.orange.border_secondary),
                border_tertiary: to_data_color(&theme.color.orange.border_tertiary),
                solid_primary: to_data_color(&theme.color.orange.solid_primary),
                solid_secondary: to_data_color(&theme.color.orange.solid_secondary),
                text_primary: to_data_color(&theme.color.orange.text_primary),
                text_secondary: to_data_color(&theme.color.orange.text_secondary),
            },
            amber: UiColorData {
                background_primary: to_data_color(&theme.color.amber.background_primary),
                background_secondary: to_data_color(&theme.color.amber.background_secondary),
                interaction_primary: to_data_color(&theme.color.amber.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.amber.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.amber.interaction_tertiary),
                border_primary: to_data_color(&theme.color.amber.border_primary),
                border_secondary: to_data_color(&theme.color.amber.border_secondary),
                border_tertiary: to_data_color(&theme.color.amber.border_tertiary),
                solid_primary: to_data_color(&theme.color.amber.solid_primary),
                solid_secondary: to_data_color(&theme.color.amber.solid_secondary),
                text_primary: to_data_color(&theme.color.amber.text_primary),
                text_secondary: to_data_color(&theme.color.amber.text_secondary),
            },
            yellow: UiColorData {
                background_primary: to_data_color(&theme.color.yellow.background_primary),
                background_secondary: to_data_color(&theme.color.yellow.background_secondary),
                interaction_primary: to_data_color(&theme.color.yellow.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.yellow.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.yellow.interaction_tertiary),
                border_primary: to_data_color(&theme.color.yellow.border_primary),
                border_secondary: to_data_color(&theme.color.yellow.border_secondary),
                border_tertiary: to_data_color(&theme.color.yellow.border_tertiary),
                solid_primary: to_data_color(&theme.color.yellow.solid_primary),
                solid_secondary: to_data_color(&theme.color.yellow.solid_secondary),
                text_primary: to_data_color(&theme.color.yellow.text_primary),
                text_secondary: to_data_color(&theme.color.yellow.text_secondary),
            },
            lime: UiColorData {
                background_primary: to_data_color(&theme.color.lime.background_primary),
                background_secondary: to_data_color(&theme.color.lime.background_secondary),
                interaction_primary: to_data_color(&theme.color.lime.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.lime.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.lime.interaction_tertiary),
                border_primary: to_data_color(&theme.color.lime.border_primary),
                border_secondary: to_data_color(&theme.color.lime.border_secondary),
                border_tertiary: to_data_color(&theme.color.lime.border_tertiary),
                solid_primary: to_data_color(&theme.color.lime.solid_primary),
                solid_secondary: to_data_color(&theme.color.lime.solid_secondary),
                text_primary: to_data_color(&theme.color.lime.text_primary),
                text_secondary: to_data_color(&theme.color.lime.text_secondary),
            },
            mint: UiColorData {
                background_primary: to_data_color(&theme.color.mint.background_primary),
                background_secondary: to_data_color(&theme.color.mint.background_secondary),
                interaction_primary: to_data_color(&theme.color.mint.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.mint.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.mint.interaction_tertiary),
                border_primary: to_data_color(&theme.color.mint.border_primary),
                border_secondary: to_data_color(&theme.color.mint.border_secondary),
                border_tertiary: to_data_color(&theme.color.mint.border_tertiary),
                solid_primary: to_data_color(&theme.color.mint.solid_primary),
                solid_secondary: to_data_color(&theme.color.mint.solid_secondary),
                text_primary: to_data_color(&theme.color.mint.text_primary),
                text_secondary: to_data_color(&theme.color.mint.text_secondary),
            },
            sky: UiColorData {
                background_primary: to_data_color(&theme.color.sky.background_primary),
                background_secondary: to_data_color(&theme.color.sky.background_secondary),
                interaction_primary: to_data_color(&theme.color.sky.interaction_primary),
                interaction_secondary: to_data_color(&theme.color.sky.interaction_secondary),
                interaction_tertiary: to_data_color(&theme.color.sky.interaction_tertiary),
                border_primary: to_data_color(&theme.color.sky.border_primary),
                border_secondary: to_data_color(&theme.color.sky.border_secondary),
                border_tertiary: to_data_color(&theme.color.sky.border_tertiary),
                solid_primary: to_data_color(&theme.color.sky.solid_primary),
                solid_secondary: to_data_color(&theme.color.sky.solid_secondary),
                text_primary: to_data_color(&theme.color.sky.text_primary),
                text_secondary: to_data_color(&theme.color.sky.text_secondary),
            },
        },
    };

    // Use ron crate for serialization
    match ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::new()) {
        Ok(ron_str) => {
            // Determine the path relative to the executable or manifest
            // Adjust "../../assets/theme.ron" if needed
            match std::fs::write("assets/theme/new.theme.ron", ron_str) {
                Ok(_) => info!("Theme saved successfully to assets/theme.ron!"),
                Err(e) => error!("Failed to write theme.ron: {}", e),
            }
        }
        Err(e) => {
            error!("Failed to serialize theme to RON: {}", e);
        }
    }
}

// System to check if the theme asset is loaded and then insert the UiTheme resource
pub fn check_theme_asset_readiness(
    mut commands: Commands,
    theme_handle_res: Option<Res<ThemeAssetHandle>>, // Make handle optional to avoid panic if removed early
    config_res: Option<Res<UiConfig>>,
    asset_server: Res<AssetServer>,
    theme_data_assets: Res<Assets<UiThemeData>>,
    existing_theme: Option<Res<UiTheme>>,
) {
    if existing_theme.is_some() {
        // Theme existiert bereits, nichts tun (run_if sollte dies verhindern)
        // Handle und Config evtl. trotzdem entfernen, falls sie noch da sind
        if theme_handle_res.is_some() {
            commands.remove_resource::<ThemeAssetHandle>();
        }
        if config_res.is_some() {
            commands.remove_resource::<UiConfig>();
        }
        return;
    }

    // Check if the handle resource exists
    let Some(theme_handle) = theme_handle_res else {
        // Should not happen if run_if condition is correct, but good practice
        // info!("ThemeAssetHandle resource not found."); // Optional log
        return;
    };

    let config = config_res.map(|r| r.clone()).unwrap_or_default();

    let handle_id = &theme_handle.0;
    let load_state = asset_server.load_state(handle_id);

    // --- DIAGNOSTIC LOG ---
    // Add this log to see the state every time the system runs
    // Be aware: This will be very spammy until loading finishes!
    debug!(
        "Checking theme asset readiness. Handle: {:?}, State: {:?}",
        handle_id.id(),
        load_state
    );
    // ----------------------

    match load_state {
        LoadState::Loaded => {
            info!("Theme asset {:?} reported as Loaded.", handle_id.id()); // Log success
                                                                           // Asset is loaded, proceed with conversion and resource insertion
            if let Some(data) = theme_data_assets.get(handle_id) {
                info!("Successfully retrieved theme data from asset."); // Log data retrieval
                                                                        // --- Effektive Werte bestimmen ---
                let effective_rem = config.rem.unwrap_or(data.rem);
                let ui_scaling = data.ui_scaling;

                // data.spacing ist der Basis-Spacing-Multiplikator (z.B. 0.25 für rem/4)
                let spacing = data.spacing * effective_rem;

                info!(
                    "Effective Theme Values: rem={}, ui_scaling={}, base_spacing_unit={}",
                    effective_rem, ui_scaling, spacing
                );

                // --- Conversion Logic (Copied from old setup_theme_resource) ---
                let load_font = |path: &str| -> Handle<Font> {
                    if path.is_empty() {
                        Handle::default()
                    } else {
                        asset_server.load(path)
                    }
                };
                let conv_color = |c: [f32; 4]| Color::srgba(c[0], c[1], c[2], c[3]);

                let typography = UiTypography {
                    /* ... copy from previous theme/systems.rs ... */
                    font_size: UiFontSize {
                        xs: data.font.font_size.xs * effective_rem * ui_scaling,
                        sm: data.font.font_size.sm * effective_rem * ui_scaling,
                        base: data.font.font_size.base * effective_rem * ui_scaling,
                        lg: data.font.font_size.lg * effective_rem * ui_scaling,
                        xl: data.font.font_size.xl * effective_rem * ui_scaling,
                        x2l: data.font.font_size.x2l * effective_rem * ui_scaling,
                        x3l: data.font.font_size.x3l * effective_rem * ui_scaling,
                        x4l: data.font.font_size.x4l * effective_rem * ui_scaling,
                        x5l: data.font.font_size.x5l * effective_rem * ui_scaling,
                        x6l: data.font.font_size.x6l * effective_rem * ui_scaling,
                        x7l: data.font.font_size.x7l * effective_rem * ui_scaling,
                        x8l: data.font.font_size.x8l * effective_rem * ui_scaling,
                        x9l: data.font.font_size.x9l * effective_rem * ui_scaling,
                    },
                    font_family: UiFontFamilies {
                        default: load_font(&data.font.font_family.default),
                        sans: FontVariants {
                            light: load_font(&data.font.font_family.sans.light),
                            light_italic: load_font(&data.font.font_family.sans.light_italic),
                            regular: load_font(&data.font.font_family.sans.regular),
                            regular_italic: load_font(&data.font.font_family.sans.regular_italic),
                            medium: load_font(&data.font.font_family.sans.medium),
                            medium_italic: load_font(&data.font.font_family.sans.medium_italic),
                            bold: load_font(&data.font.font_family.sans.bold),
                            bold_italic: load_font(&data.font.font_family.sans.bold_italic),
                        },
                        serif: FontVariants {
                            light: load_font(&data.font.font_family.serif.light),
                            light_italic: load_font(&data.font.font_family.serif.light_italic),
                            regular: load_font(&data.font.font_family.serif.regular),
                            regular_italic: load_font(&data.font.font_family.serif.regular_italic),
                            medium: load_font(&data.font.font_family.serif.medium),
                            medium_italic: load_font(&data.font.font_family.serif.medium_italic),
                            bold: load_font(&data.font.font_family.serif.bold),
                            bold_italic: load_font(&data.font.font_family.serif.bold_italic),
                        },
                        mono: FontVariants {
                            light: load_font(&data.font.font_family.mono.light),
                            light_italic: load_font(&data.font.font_family.mono.light_italic),
                            regular: load_font(&data.font.font_family.mono.regular),
                            regular_italic: load_font(&data.font.font_family.mono.regular_italic),
                            medium: load_font(&data.font.font_family.mono.medium),
                            medium_italic: load_font(&data.font.font_family.mono.medium_italic),
                            bold: load_font(&data.font.font_family.mono.bold),
                            bold_italic: load_font(&data.font.font_family.mono.bold_italic),
                        },
                    },
                };
                let layout = UiLayout {
                    /* ... copy from previous theme/systems.rs ... */
                    spacing: spacing,
                    padding: UiSpacing {
                        xs: data.layout.padding.xs * spacing * ui_scaling,
                        sm: data.layout.padding.sm * spacing * ui_scaling,
                        base: data.layout.padding.base * spacing * ui_scaling,
                        lg: data.layout.padding.lg * spacing * ui_scaling,
                        xl: data.layout.padding.xl * spacing * ui_scaling,
                        x2l: data.layout.padding.x2l * spacing * ui_scaling,
                        x3l: data.layout.padding.x3l * spacing * ui_scaling,
                        x4l: data.layout.padding.x4l * spacing * ui_scaling,
                        x5l: data.layout.padding.x5l * spacing * ui_scaling,
                    },
                    margin: UiSpacing {
                        xs: data.layout.margin.xs * spacing * ui_scaling,
                        sm: data.layout.margin.sm * spacing * ui_scaling,
                        base: data.layout.margin.base * spacing * ui_scaling,
                        lg: data.layout.margin.lg * spacing * ui_scaling,
                        xl: data.layout.margin.xl * spacing * ui_scaling,
                        x2l: data.layout.margin.x2l * spacing * ui_scaling,
                        x3l: data.layout.margin.x3l * spacing * ui_scaling,
                        x4l: data.layout.margin.x4l * spacing * ui_scaling,
                        x5l: data.layout.margin.x5l * spacing * ui_scaling,
                    },
                    gap: UiSpacing {
                        xs: data.layout.gap.xs * spacing * ui_scaling,
                        sm: data.layout.gap.sm * spacing * ui_scaling,
                        base: data.layout.gap.base * spacing * ui_scaling,
                        lg: data.layout.gap.lg * spacing * ui_scaling,
                        xl: data.layout.gap.xl * spacing * ui_scaling,
                        x2l: data.layout.gap.x2l * spacing * ui_scaling,
                        x3l: data.layout.gap.x3l * spacing * ui_scaling,
                        x4l: data.layout.gap.x4l * spacing * ui_scaling,
                        x5l: data.layout.gap.x5l * spacing * ui_scaling,
                    },
                    radius: UiRadius {
                        xs: data.layout.radius.xs * spacing * ui_scaling,
                        sm: data.layout.radius.sm * spacing * ui_scaling,
                        base: data.layout.radius.base * spacing * ui_scaling,
                        lg: data.layout.radius.lg * spacing * ui_scaling,
                        xl: data.layout.radius.xl * spacing * ui_scaling,
                        x2l: data.layout.radius.x2l * spacing * ui_scaling,
                        x3l: data.layout.radius.x3l * spacing * ui_scaling,
                        x4l: data.layout.radius.x4l * spacing * ui_scaling,
                        full: data.layout.radius.full,
                    },
                    border: UiSpacing {
                        xs: data.layout.border.xs,
                        sm: data.layout.border.sm,
                        base: data.layout.border.base,
                        lg: data.layout.border.lg,
                        xl: data.layout.border.xl,
                        x2l: data.layout.border.x2l,
                        x3l: data.layout.border.x3l,
                        x4l: data.layout.border.x4l,
                        x5l: data.layout.border.x5l,
                    },
                };
                let colors = UiColors {
                    /* ... copy from previous theme/systems.rs ... */
                    white: UiColor {
                        background_primary: conv_color(data.color.white.background_primary),
                        background_secondary: conv_color(data.color.white.background_secondary),
                        interaction_primary: conv_color(data.color.white.interaction_primary),
                        interaction_secondary: conv_color(data.color.white.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.white.interaction_tertiary),
                        border_primary: conv_color(data.color.white.border_primary),
                        border_secondary: conv_color(data.color.white.border_secondary),
                        border_tertiary: conv_color(data.color.white.border_tertiary),
                        solid_primary: conv_color(data.color.white.solid_primary),
                        solid_secondary: conv_color(data.color.white.solid_secondary),
                        text_primary: conv_color(data.color.white.text_primary),
                        text_secondary: conv_color(data.color.white.text_secondary),
                    },
                    black: UiColor {
                        background_primary: conv_color(data.color.black.background_primary),
                        background_secondary: conv_color(data.color.black.background_secondary),
                        interaction_primary: conv_color(data.color.black.interaction_primary),
                        interaction_secondary: conv_color(data.color.black.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.black.interaction_tertiary),
                        border_primary: conv_color(data.color.black.border_primary),
                        border_secondary: conv_color(data.color.black.border_secondary),
                        border_tertiary: conv_color(data.color.black.border_tertiary),
                        solid_primary: conv_color(data.color.black.solid_primary),
                        solid_secondary: conv_color(data.color.black.solid_secondary),
                        text_primary: conv_color(data.color.black.text_primary),
                        text_secondary: conv_color(data.color.black.text_secondary),
                    },
                    gray: UiColor {
                        background_primary: conv_color(data.color.gray.background_primary),
                        background_secondary: conv_color(data.color.gray.background_secondary),
                        interaction_primary: conv_color(data.color.gray.interaction_primary),
                        interaction_secondary: conv_color(data.color.gray.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.gray.interaction_tertiary),
                        border_primary: conv_color(data.color.gray.border_primary),
                        border_secondary: conv_color(data.color.gray.border_secondary),
                        border_tertiary: conv_color(data.color.gray.border_tertiary),
                        solid_primary: conv_color(data.color.gray.solid_primary),
                        solid_secondary: conv_color(data.color.gray.solid_secondary),
                        text_primary: conv_color(data.color.gray.text_primary),
                        text_secondary: conv_color(data.color.gray.text_secondary),
                    },
                    mauve: UiColor {
                        background_primary: conv_color(data.color.mauve.background_primary),
                        background_secondary: conv_color(data.color.mauve.background_secondary),
                        interaction_primary: conv_color(data.color.mauve.interaction_primary),
                        interaction_secondary: conv_color(data.color.mauve.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.mauve.interaction_tertiary),
                        border_primary: conv_color(data.color.mauve.border_primary),
                        border_secondary: conv_color(data.color.mauve.border_secondary),
                        border_tertiary: conv_color(data.color.mauve.border_tertiary),
                        solid_primary: conv_color(data.color.mauve.solid_primary),
                        solid_secondary: conv_color(data.color.mauve.solid_secondary),
                        text_primary: conv_color(data.color.mauve.text_primary),
                        text_secondary: conv_color(data.color.mauve.text_secondary),
                    },
                    slate: UiColor {
                        background_primary: conv_color(data.color.slate.background_primary),
                        background_secondary: conv_color(data.color.slate.background_secondary),
                        interaction_primary: conv_color(data.color.slate.interaction_primary),
                        interaction_secondary: conv_color(data.color.slate.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.slate.interaction_tertiary),
                        border_primary: conv_color(data.color.slate.border_primary),
                        border_secondary: conv_color(data.color.slate.border_secondary),
                        border_tertiary: conv_color(data.color.slate.border_tertiary),
                        solid_primary: conv_color(data.color.slate.solid_primary),
                        solid_secondary: conv_color(data.color.slate.solid_secondary),
                        text_primary: conv_color(data.color.slate.text_primary),
                        text_secondary: conv_color(data.color.slate.text_secondary),
                    },
                    sage: UiColor {
                        background_primary: conv_color(data.color.sage.background_primary),
                        background_secondary: conv_color(data.color.sage.background_secondary),
                        interaction_primary: conv_color(data.color.sage.interaction_primary),
                        interaction_secondary: conv_color(data.color.sage.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.sage.interaction_tertiary),
                        border_primary: conv_color(data.color.sage.border_primary),
                        border_secondary: conv_color(data.color.sage.border_secondary),
                        border_tertiary: conv_color(data.color.sage.border_tertiary),
                        solid_primary: conv_color(data.color.sage.solid_primary),
                        solid_secondary: conv_color(data.color.sage.solid_secondary),
                        text_primary: conv_color(data.color.sage.text_primary),
                        text_secondary: conv_color(data.color.sage.text_secondary),
                    },
                    olive: UiColor {
                        background_primary: conv_color(data.color.olive.background_primary),
                        background_secondary: conv_color(data.color.olive.background_secondary),
                        interaction_primary: conv_color(data.color.olive.interaction_primary),
                        interaction_secondary: conv_color(data.color.olive.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.olive.interaction_tertiary),
                        border_primary: conv_color(data.color.olive.border_primary),
                        border_secondary: conv_color(data.color.olive.border_secondary),
                        border_tertiary: conv_color(data.color.olive.border_tertiary),
                        solid_primary: conv_color(data.color.olive.solid_primary),
                        solid_secondary: conv_color(data.color.olive.solid_secondary),
                        text_primary: conv_color(data.color.olive.text_primary),
                        text_secondary: conv_color(data.color.olive.text_secondary),
                    },
                    sand: UiColor {
                        background_primary: conv_color(data.color.sand.background_primary),
                        background_secondary: conv_color(data.color.sand.background_secondary),
                        interaction_primary: conv_color(data.color.sand.interaction_primary),
                        interaction_secondary: conv_color(data.color.sand.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.sand.interaction_tertiary),
                        border_primary: conv_color(data.color.sand.border_primary),
                        border_secondary: conv_color(data.color.sand.border_secondary),
                        border_tertiary: conv_color(data.color.sand.border_tertiary),
                        solid_primary: conv_color(data.color.sand.solid_primary),
                        solid_secondary: conv_color(data.color.sand.solid_secondary),
                        text_primary: conv_color(data.color.sand.text_primary),
                        text_secondary: conv_color(data.color.sand.text_secondary),
                    },
                    tomato: UiColor {
                        background_primary: conv_color(data.color.tomato.background_primary),
                        background_secondary: conv_color(data.color.tomato.background_secondary),
                        interaction_primary: conv_color(data.color.tomato.interaction_primary),
                        interaction_secondary: conv_color(data.color.tomato.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.tomato.interaction_tertiary),
                        border_primary: conv_color(data.color.tomato.border_primary),
                        border_secondary: conv_color(data.color.tomato.border_secondary),
                        border_tertiary: conv_color(data.color.tomato.border_tertiary),
                        solid_primary: conv_color(data.color.tomato.solid_primary),
                        solid_secondary: conv_color(data.color.tomato.solid_secondary),
                        text_primary: conv_color(data.color.tomato.text_primary),
                        text_secondary: conv_color(data.color.tomato.text_secondary),
                    },
                    red: UiColor {
                        background_primary: conv_color(data.color.red.background_primary),
                        background_secondary: conv_color(data.color.red.background_secondary),
                        interaction_primary: conv_color(data.color.red.interaction_primary),
                        interaction_secondary: conv_color(data.color.red.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.red.interaction_tertiary),
                        border_primary: conv_color(data.color.red.border_primary),
                        border_secondary: conv_color(data.color.red.border_secondary),
                        border_tertiary: conv_color(data.color.red.border_tertiary),
                        solid_primary: conv_color(data.color.red.solid_primary),
                        solid_secondary: conv_color(data.color.red.solid_secondary),
                        text_primary: conv_color(data.color.red.text_primary),
                        text_secondary: conv_color(data.color.red.text_secondary),
                    },
                    ruby: UiColor {
                        background_primary: conv_color(data.color.ruby.background_primary),
                        background_secondary: conv_color(data.color.ruby.background_secondary),
                        interaction_primary: conv_color(data.color.ruby.interaction_primary),
                        interaction_secondary: conv_color(data.color.ruby.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.ruby.interaction_tertiary),
                        border_primary: conv_color(data.color.ruby.border_primary),
                        border_secondary: conv_color(data.color.ruby.border_secondary),
                        border_tertiary: conv_color(data.color.ruby.border_tertiary),
                        solid_primary: conv_color(data.color.ruby.solid_primary),
                        solid_secondary: conv_color(data.color.ruby.solid_secondary),
                        text_primary: conv_color(data.color.ruby.text_primary),
                        text_secondary: conv_color(data.color.ruby.text_secondary),
                    },
                    crimson: UiColor {
                        background_primary: conv_color(data.color.crimson.background_primary),
                        background_secondary: conv_color(data.color.crimson.background_secondary),
                        interaction_primary: conv_color(data.color.crimson.interaction_primary),
                        interaction_secondary: conv_color(data.color.crimson.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.crimson.interaction_tertiary),
                        border_primary: conv_color(data.color.crimson.border_primary),
                        border_secondary: conv_color(data.color.crimson.border_secondary),
                        border_tertiary: conv_color(data.color.crimson.border_tertiary),
                        solid_primary: conv_color(data.color.crimson.solid_primary),
                        solid_secondary: conv_color(data.color.crimson.solid_secondary),
                        text_primary: conv_color(data.color.crimson.text_primary),
                        text_secondary: conv_color(data.color.crimson.text_secondary),
                    },
                    pink: UiColor {
                        background_primary: conv_color(data.color.pink.background_primary),
                        background_secondary: conv_color(data.color.pink.background_secondary),
                        interaction_primary: conv_color(data.color.pink.interaction_primary),
                        interaction_secondary: conv_color(data.color.pink.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.pink.interaction_tertiary),
                        border_primary: conv_color(data.color.pink.border_primary),
                        border_secondary: conv_color(data.color.pink.border_secondary),
                        border_tertiary: conv_color(data.color.pink.border_tertiary),
                        solid_primary: conv_color(data.color.pink.solid_primary),
                        solid_secondary: conv_color(data.color.pink.solid_secondary),
                        text_primary: conv_color(data.color.pink.text_primary),
                        text_secondary: conv_color(data.color.pink.text_secondary),
                    },
                    plum: UiColor {
                        background_primary: conv_color(data.color.plum.background_primary),
                        background_secondary: conv_color(data.color.plum.background_secondary),
                        interaction_primary: conv_color(data.color.plum.interaction_primary),
                        interaction_secondary: conv_color(data.color.plum.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.plum.interaction_tertiary),
                        border_primary: conv_color(data.color.plum.border_primary),
                        border_secondary: conv_color(data.color.plum.border_secondary),
                        border_tertiary: conv_color(data.color.plum.border_tertiary),
                        solid_primary: conv_color(data.color.plum.solid_primary),
                        solid_secondary: conv_color(data.color.plum.solid_secondary),
                        text_primary: conv_color(data.color.plum.text_primary),
                        text_secondary: conv_color(data.color.plum.text_secondary),
                    },
                    purple: UiColor {
                        background_primary: conv_color(data.color.purple.background_primary),
                        background_secondary: conv_color(data.color.purple.background_secondary),
                        interaction_primary: conv_color(data.color.purple.interaction_primary),
                        interaction_secondary: conv_color(data.color.purple.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.purple.interaction_tertiary),
                        border_primary: conv_color(data.color.purple.border_primary),
                        border_secondary: conv_color(data.color.purple.border_secondary),
                        border_tertiary: conv_color(data.color.purple.border_tertiary),
                        solid_primary: conv_color(data.color.purple.solid_primary),
                        solid_secondary: conv_color(data.color.purple.solid_secondary),
                        text_primary: conv_color(data.color.purple.text_primary),
                        text_secondary: conv_color(data.color.purple.text_secondary),
                    },
                    violet: UiColor {
                        background_primary: conv_color(data.color.violet.background_primary),
                        background_secondary: conv_color(data.color.violet.background_secondary),
                        interaction_primary: conv_color(data.color.violet.interaction_primary),
                        interaction_secondary: conv_color(data.color.violet.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.violet.interaction_tertiary),
                        border_primary: conv_color(data.color.violet.border_primary),
                        border_secondary: conv_color(data.color.violet.border_secondary),
                        border_tertiary: conv_color(data.color.violet.border_tertiary),
                        solid_primary: conv_color(data.color.violet.solid_primary),
                        solid_secondary: conv_color(data.color.violet.solid_secondary),
                        text_primary: conv_color(data.color.violet.text_primary),
                        text_secondary: conv_color(data.color.violet.text_secondary),
                    },
                    iris: UiColor {
                        background_primary: conv_color(data.color.iris.background_primary),
                        background_secondary: conv_color(data.color.iris.background_secondary),
                        interaction_primary: conv_color(data.color.iris.interaction_primary),
                        interaction_secondary: conv_color(data.color.iris.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.iris.interaction_tertiary),
                        border_primary: conv_color(data.color.iris.border_primary),
                        border_secondary: conv_color(data.color.iris.border_secondary),
                        border_tertiary: conv_color(data.color.iris.border_tertiary),
                        solid_primary: conv_color(data.color.iris.solid_primary),
                        solid_secondary: conv_color(data.color.iris.solid_secondary),
                        text_primary: conv_color(data.color.iris.text_primary),
                        text_secondary: conv_color(data.color.iris.text_secondary),
                    },
                    indigo: UiColor {
                        background_primary: conv_color(data.color.indigo.background_primary),
                        background_secondary: conv_color(data.color.indigo.background_secondary),
                        interaction_primary: conv_color(data.color.indigo.interaction_primary),
                        interaction_secondary: conv_color(data.color.indigo.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.indigo.interaction_tertiary),
                        border_primary: conv_color(data.color.indigo.border_primary),
                        border_secondary: conv_color(data.color.indigo.border_secondary),
                        border_tertiary: conv_color(data.color.indigo.border_tertiary),
                        solid_primary: conv_color(data.color.indigo.solid_primary),
                        solid_secondary: conv_color(data.color.indigo.solid_secondary),
                        text_primary: conv_color(data.color.indigo.text_primary),
                        text_secondary: conv_color(data.color.indigo.text_secondary),
                    },
                    blue: UiColor {
                        background_primary: conv_color(data.color.blue.background_primary),
                        background_secondary: conv_color(data.color.blue.background_secondary),
                        interaction_primary: conv_color(data.color.blue.interaction_primary),
                        interaction_secondary: conv_color(data.color.blue.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.blue.interaction_tertiary),
                        border_primary: conv_color(data.color.blue.border_primary),
                        border_secondary: conv_color(data.color.blue.border_secondary),
                        border_tertiary: conv_color(data.color.blue.border_tertiary),
                        solid_primary: conv_color(data.color.blue.solid_primary),
                        solid_secondary: conv_color(data.color.blue.solid_secondary),
                        text_primary: conv_color(data.color.blue.text_primary),
                        text_secondary: conv_color(data.color.blue.text_secondary),
                    },
                    cyan: UiColor {
                        background_primary: conv_color(data.color.cyan.background_primary),
                        background_secondary: conv_color(data.color.cyan.background_secondary),
                        interaction_primary: conv_color(data.color.cyan.interaction_primary),
                        interaction_secondary: conv_color(data.color.cyan.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.cyan.interaction_tertiary),
                        border_primary: conv_color(data.color.cyan.border_primary),
                        border_secondary: conv_color(data.color.cyan.border_secondary),
                        border_tertiary: conv_color(data.color.cyan.border_tertiary),
                        solid_primary: conv_color(data.color.cyan.solid_primary),
                        solid_secondary: conv_color(data.color.cyan.solid_secondary),
                        text_primary: conv_color(data.color.cyan.text_primary),
                        text_secondary: conv_color(data.color.cyan.text_secondary),
                    },
                    teal: UiColor {
                        background_primary: conv_color(data.color.teal.background_primary),
                        background_secondary: conv_color(data.color.teal.background_secondary),
                        interaction_primary: conv_color(data.color.teal.interaction_primary),
                        interaction_secondary: conv_color(data.color.teal.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.teal.interaction_tertiary),
                        border_primary: conv_color(data.color.teal.border_primary),
                        border_secondary: conv_color(data.color.teal.border_secondary),
                        border_tertiary: conv_color(data.color.teal.border_tertiary),
                        solid_primary: conv_color(data.color.teal.solid_primary),
                        solid_secondary: conv_color(data.color.teal.solid_secondary),
                        text_primary: conv_color(data.color.teal.text_primary),
                        text_secondary: conv_color(data.color.teal.text_secondary),
                    },
                    jade: UiColor {
                        background_primary: conv_color(data.color.jade.background_primary),
                        background_secondary: conv_color(data.color.jade.background_secondary),
                        interaction_primary: conv_color(data.color.jade.interaction_primary),
                        interaction_secondary: conv_color(data.color.jade.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.jade.interaction_tertiary),
                        border_primary: conv_color(data.color.jade.border_primary),
                        border_secondary: conv_color(data.color.jade.border_secondary),
                        border_tertiary: conv_color(data.color.jade.border_tertiary),
                        solid_primary: conv_color(data.color.jade.solid_primary),
                        solid_secondary: conv_color(data.color.jade.solid_secondary),
                        text_primary: conv_color(data.color.jade.text_primary),
                        text_secondary: conv_color(data.color.jade.text_secondary),
                    },
                    green: UiColor {
                        background_primary: conv_color(data.color.green.background_primary),
                        background_secondary: conv_color(data.color.green.background_secondary),
                        interaction_primary: conv_color(data.color.green.interaction_primary),
                        interaction_secondary: conv_color(data.color.green.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.green.interaction_tertiary),
                        border_primary: conv_color(data.color.green.border_primary),
                        border_secondary: conv_color(data.color.green.border_secondary),
                        border_tertiary: conv_color(data.color.green.border_tertiary),
                        solid_primary: conv_color(data.color.green.solid_primary),
                        solid_secondary: conv_color(data.color.green.solid_secondary),
                        text_primary: conv_color(data.color.green.text_primary),
                        text_secondary: conv_color(data.color.green.text_secondary),
                    },
                    grass: UiColor {
                        background_primary: conv_color(data.color.grass.background_primary),
                        background_secondary: conv_color(data.color.grass.background_secondary),
                        interaction_primary: conv_color(data.color.grass.interaction_primary),
                        interaction_secondary: conv_color(data.color.grass.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.grass.interaction_tertiary),
                        border_primary: conv_color(data.color.grass.border_primary),
                        border_secondary: conv_color(data.color.grass.border_secondary),
                        border_tertiary: conv_color(data.color.grass.border_tertiary),
                        solid_primary: conv_color(data.color.grass.solid_primary),
                        solid_secondary: conv_color(data.color.grass.solid_secondary),
                        text_primary: conv_color(data.color.grass.text_primary),
                        text_secondary: conv_color(data.color.grass.text_secondary),
                    },
                    bronze: UiColor {
                        background_primary: conv_color(data.color.bronze.background_primary),
                        background_secondary: conv_color(data.color.bronze.background_secondary),
                        interaction_primary: conv_color(data.color.bronze.interaction_primary),
                        interaction_secondary: conv_color(data.color.bronze.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.bronze.interaction_tertiary),
                        border_primary: conv_color(data.color.bronze.border_primary),
                        border_secondary: conv_color(data.color.bronze.border_secondary),
                        border_tertiary: conv_color(data.color.bronze.border_tertiary),
                        solid_primary: conv_color(data.color.bronze.solid_primary),
                        solid_secondary: conv_color(data.color.bronze.solid_secondary),
                        text_primary: conv_color(data.color.bronze.text_primary),
                        text_secondary: conv_color(data.color.bronze.text_secondary),
                    },
                    gold: UiColor {
                        background_primary: conv_color(data.color.gold.background_primary),
                        background_secondary: conv_color(data.color.gold.background_secondary),
                        interaction_primary: conv_color(data.color.gold.interaction_primary),
                        interaction_secondary: conv_color(data.color.gold.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.gold.interaction_tertiary),
                        border_primary: conv_color(data.color.gold.border_primary),
                        border_secondary: conv_color(data.color.gold.border_secondary),
                        border_tertiary: conv_color(data.color.gold.border_tertiary),
                        solid_primary: conv_color(data.color.gold.solid_primary),
                        solid_secondary: conv_color(data.color.gold.solid_secondary),
                        text_primary: conv_color(data.color.gold.text_primary),
                        text_secondary: conv_color(data.color.gold.text_secondary),
                    },
                    brown: UiColor {
                        background_primary: conv_color(data.color.brown.background_primary),
                        background_secondary: conv_color(data.color.brown.background_secondary),
                        interaction_primary: conv_color(data.color.brown.interaction_primary),
                        interaction_secondary: conv_color(data.color.brown.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.brown.interaction_tertiary),
                        border_primary: conv_color(data.color.brown.border_primary),
                        border_secondary: conv_color(data.color.brown.border_secondary),
                        border_tertiary: conv_color(data.color.brown.border_tertiary),
                        solid_primary: conv_color(data.color.brown.solid_primary),
                        solid_secondary: conv_color(data.color.brown.solid_secondary),
                        text_primary: conv_color(data.color.brown.text_primary),
                        text_secondary: conv_color(data.color.brown.text_secondary),
                    },
                    orange: UiColor {
                        background_primary: conv_color(data.color.orange.background_primary),
                        background_secondary: conv_color(data.color.orange.background_secondary),
                        interaction_primary: conv_color(data.color.orange.interaction_primary),
                        interaction_secondary: conv_color(data.color.orange.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.orange.interaction_tertiary),
                        border_primary: conv_color(data.color.orange.border_primary),
                        border_secondary: conv_color(data.color.orange.border_secondary),
                        border_tertiary: conv_color(data.color.orange.border_tertiary),
                        solid_primary: conv_color(data.color.orange.solid_primary),
                        solid_secondary: conv_color(data.color.orange.solid_secondary),
                        text_primary: conv_color(data.color.orange.text_primary),
                        text_secondary: conv_color(data.color.orange.text_secondary),
                    },
                    amber: UiColor {
                        background_primary: conv_color(data.color.amber.background_primary),
                        background_secondary: conv_color(data.color.amber.background_secondary),
                        interaction_primary: conv_color(data.color.amber.interaction_primary),
                        interaction_secondary: conv_color(data.color.amber.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.amber.interaction_tertiary),
                        border_primary: conv_color(data.color.amber.border_primary),
                        border_secondary: conv_color(data.color.amber.border_secondary),
                        border_tertiary: conv_color(data.color.amber.border_tertiary),
                        solid_primary: conv_color(data.color.amber.solid_primary),
                        solid_secondary: conv_color(data.color.amber.solid_secondary),
                        text_primary: conv_color(data.color.amber.text_primary),
                        text_secondary: conv_color(data.color.amber.text_secondary),
                    },
                    yellow: UiColor {
                        background_primary: conv_color(data.color.yellow.background_primary),
                        background_secondary: conv_color(data.color.yellow.background_secondary),
                        interaction_primary: conv_color(data.color.yellow.interaction_primary),
                        interaction_secondary: conv_color(data.color.yellow.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.yellow.interaction_tertiary),
                        border_primary: conv_color(data.color.yellow.border_primary),
                        border_secondary: conv_color(data.color.yellow.border_secondary),
                        border_tertiary: conv_color(data.color.yellow.border_tertiary),
                        solid_primary: conv_color(data.color.yellow.solid_primary),
                        solid_secondary: conv_color(data.color.yellow.solid_secondary),
                        text_primary: conv_color(data.color.yellow.text_primary),
                        text_secondary: conv_color(data.color.yellow.text_secondary),
                    },
                    lime: UiColor {
                        background_primary: conv_color(data.color.lime.background_primary),
                        background_secondary: conv_color(data.color.lime.background_secondary),
                        interaction_primary: conv_color(data.color.lime.interaction_primary),
                        interaction_secondary: conv_color(data.color.lime.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.lime.interaction_tertiary),
                        border_primary: conv_color(data.color.lime.border_primary),
                        border_secondary: conv_color(data.color.lime.border_secondary),
                        border_tertiary: conv_color(data.color.lime.border_tertiary),
                        solid_primary: conv_color(data.color.lime.solid_primary),
                        solid_secondary: conv_color(data.color.lime.solid_secondary),
                        text_primary: conv_color(data.color.lime.text_primary),
                        text_secondary: conv_color(data.color.lime.text_secondary),
                    },
                    mint: UiColor {
                        background_primary: conv_color(data.color.mint.background_primary),
                        background_secondary: conv_color(data.color.mint.background_secondary),
                        interaction_primary: conv_color(data.color.mint.interaction_primary),
                        interaction_secondary: conv_color(data.color.mint.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.mint.interaction_tertiary),
                        border_primary: conv_color(data.color.mint.border_primary),
                        border_secondary: conv_color(data.color.mint.border_secondary),
                        border_tertiary: conv_color(data.color.mint.border_tertiary),
                        solid_primary: conv_color(data.color.mint.solid_primary),
                        solid_secondary: conv_color(data.color.mint.solid_secondary),
                        text_primary: conv_color(data.color.mint.text_primary),
                        text_secondary: conv_color(data.color.mint.text_secondary),
                    },
                    sky: UiColor {
                        background_primary: conv_color(data.color.sky.background_primary),
                        background_secondary: conv_color(data.color.sky.background_secondary),
                        interaction_primary: conv_color(data.color.sky.interaction_primary),
                        interaction_secondary: conv_color(data.color.sky.interaction_secondary),
                        interaction_tertiary: conv_color(data.color.sky.interaction_tertiary),
                        border_primary: conv_color(data.color.sky.border_primary),
                        border_secondary: conv_color(data.color.sky.border_secondary),
                        border_tertiary: conv_color(data.color.sky.border_tertiary),
                        solid_primary: conv_color(data.color.sky.solid_primary),
                        solid_secondary: conv_color(data.color.sky.solid_secondary),
                        text_primary: conv_color(data.color.sky.text_primary),
                        text_secondary: conv_color(data.color.sky.text_secondary),
                    },
                };
                // --- End Conversion Logic ---

                let theme = UiTheme {
                    ui_scaling: ui_scaling,
                    rem: effective_rem,
                    font: typography,
                    layout,
                    color: colors,
                };

                commands.insert_resource(theme);
                // Remove the handle *after* inserting the resource
                // Use remove_resource::<ThemeAssetHandle>() directly on commands
                commands.remove_resource::<ThemeAssetHandle>();
                commands.remove_resource::<UiConfig>();
                info!("UiTheme resource inserted and ThemeAssetHandle removed.");
            } else {
                error!("UiThemeData asset data not found for handle {:?} even though LoadState is Loaded.", handle_id.id());
                // Handle und Config trotzdem entfernen, um Endlosschleife zu vermeiden
                commands.remove_resource::<ThemeAssetHandle>();
                if config_res.is_some() {
                    // Nur entfernen, wenn es existiert
                    commands.remove_resource::<UiConfig>();
                }
            }
        }
        LoadState::Failed(error) => {
            error!(
                "Failed to load theme asset {:?}: {:?}",
                handle_id.id(),
                error
            );
            // Fehlerhafte Ressourcen entfernen
            commands.remove_resource::<ThemeAssetHandle>();
            if config_res.is_some() {
                commands.remove_resource::<UiConfig>();
            }
            // Optional: Hier Default-Theme einfügen oder App anders behandeln
            // commands.insert_resource(UiTheme::default());
            // warn!("Inserted default UiTheme due to loading failure.");
        }
        LoadState::Loading => {
            // Still loading, do nothing - the debug log above shows this state
            info!("Theme asset {:?} is still loading.", handle_id.id());
        }
        LoadState::NotLoaded => {
            // Asset might not have been processed by asset server yet
            debug!("Theme asset {:?} is NotLoaded.", handle_id.id());
        }
    }
    // If still loading or not started, do nothing and wait for the next run
}
