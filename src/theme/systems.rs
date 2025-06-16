// crates/forge_ui/src/theme/systems.rs
use crate::assets::FontAssets;
use crate::plugin::UiConfig;
use crate::theme::settings::Appearance;
use crate::theme::{data::*, runtime::*, UiTheme};
use bevy::prelude::*;

// Define a handle resource to track the theme asset
#[derive(Resource)]
/// A wrapper struct for a handle to `UiThemeData` assets.
///
/// This struct is used to manage and reference UI theme data assets within the application.
/// By encapsulating the `Handle<UiThemeData>`, it provides type safety and clarity when working
/// with theme-related resources in the UI system.
pub struct ThemeAssetHandle(Handle<UiThemeData>);

// System to load the asset handle during PreStartup
pub fn load_theme_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<UiConfig>,
) {
    let handle = asset_server.load(match config.appearance {
        Appearance::Dark => "theme/dark.theme.ron",
        Appearance::Light => "theme/light.theme.ron",
    });
    commands.insert_resource(ThemeAssetHandle(handle));

    info!(
        "Initiated loading {:?} theme (waiting for load state)",
        config.appearance
    );
}

pub fn check_theme_asset_readiness(
    mut commands: Commands,
    handles: Res<ThemeAssetHandle>,
    theme_assets: Res<Assets<UiThemeData>>,
    font_assets: Option<Res<FontAssets>>,
    asset_server: Res<AssetServer>,
    config: Res<UiConfig>,
) {
    use bevy::asset::LoadState;
    if let Some(font_assets) = font_assets {
        match asset_server.get_load_state(&handles.0) {
            Some(LoadState::Loaded) => {
                let data = theme_assets.get(&handles.0).unwrap();
                // Nutze jetzt die bereits geladenen Handles aus font_assets
                let theme = UiTheme::build_from_data(font_assets, data, &config);
                commands.insert_resource(theme);
            }
            Some(LoadState::Failed(_)) => {
                // läd Default aus Code
                warn!("Theme asset failed to load, using hard-coded default.");
                let data = UiThemeData::default();
                let theme = UiTheme::build_from_data(font_assets, &data, &config);
                commands.insert_resource(theme);
            }
            _ => {
                // Noch nicht fertig laden oder kein Status verfügbar → nichts tun
            }
        }
    } else {
        // FontAssets sind nicht verfügbar, also nichts tun
        error!("FontAssets not available yet, skipping theme loading.");
    }
}

// Keep hot_reload_theme_system
pub fn hot_reload_theme_system(
    mut ev_asset: EventReader<AssetEvent<UiThemeData>>,
    asset_server: Res<AssetServer>,
    theme_data_assets: Res<Assets<UiThemeData>>,
    theme_opt: Option<ResMut<UiTheme>>,
    config: Res<UiConfig>,
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
                    // Wichtig: Benutze den *gespeicherten* font_size-Wert aus dem Theme!
                    let effective_font_size = config.font_size_base;
                    // Nimm den *neuen* ui_scaling-Wert aus den geladenen Daten
                    let effective_ui_scaling = config.scaling;
                    // Berechne die neue Basis-Spacing-Einheit
                    let base_spacing_unit =
                        config.spacing_factor * effective_font_size * effective_ui_scaling;

                    info!(
                        "Hot Reload Effective Values: font_size={}, ui_scaling={}, base_spacing_unit={}",
                        effective_font_size, effective_ui_scaling, base_spacing_unit
                    );
                    // --- Felder in `theme` aktualisieren ---

                    let load_font = |path: &str| -> Handle<Font> {
                        if path.is_empty() {
                            Handle::default()
                        } else {
                            asset_server.load(path)
                        }
                    };
                    let conv_color = |c: [f32; 4]| Color::srgba(c[0], c[1], c[2], c[3]);
                    let color_palette = |data: &UiColorPaletteData| UiColorPalette {
                        step01: conv_color(data.step01),
                        step02: conv_color(data.step02),
                        step03: conv_color(data.step03),
                        step04: conv_color(data.step04),
                        step05: conv_color(data.step05),
                        step06: conv_color(data.step06),
                        step07: conv_color(data.step07),
                        step08: conv_color(data.step08),
                        step09: conv_color(data.step09),
                        step10: conv_color(data.step10),
                        step11: conv_color(data.step11),
                        step12: conv_color(data.step12),
                    };

                    // Update typography
                    theme.font.size = UiFontSize {
                        xs: data.font.size.xs * effective_font_size * effective_ui_scaling,
                        sm: data.font.size.sm * effective_font_size * effective_ui_scaling,
                        base: data.font.size.base * effective_font_size * effective_ui_scaling,
                        lg: data.font.size.lg * effective_font_size * effective_ui_scaling,
                        xl: data.font.size.xl * effective_font_size * effective_ui_scaling,
                        x2l: data.font.size.x2l * effective_font_size * effective_ui_scaling,
                        x3l: data.font.size.x3l * effective_font_size * effective_ui_scaling,
                        x4l: data.font.size.x4l * effective_font_size * effective_ui_scaling,
                        x5l: data.font.size.x5l * effective_font_size * effective_ui_scaling,
                        x6l: data.font.size.x6l * effective_font_size * effective_ui_scaling,
                        x7l: data.font.size.x7l * effective_font_size * effective_ui_scaling,
                        x8l: data.font.size.x8l * effective_font_size * effective_ui_scaling,
                        x9l: data.font.size.x9l * effective_font_size * effective_ui_scaling,
                        h1: data.font.size.x4l * effective_font_size * effective_ui_scaling,
                        h2: data.font.size.x2l * effective_font_size * effective_ui_scaling,
                        h3: data.font.size.xl * effective_font_size * effective_ui_scaling,
                        h4: data.font.size.lg * effective_font_size * effective_ui_scaling,
                    };
                    // Font-Familien nur setzen, wenn der Pfad nicht leer ist
                    if !data.font.family.default.is_empty() {
                        theme.font.family.default = load_font(&data.font.family.default);
                    }
                    macro_rules! set_font_variant {
                        ($theme:expr, $data:expr, $family:ident, $variant:ident) => {
                            if !$data.font.family.$family.$variant.is_empty() {
                                $theme.font.family.$family.$variant =
                                    load_font(&$data.font.family.$family.$variant);
                            }
                        };
                    }
                    // Sans
                    set_font_variant!(theme, data, sans, light);
                    set_font_variant!(theme, data, sans, light_italic);
                    set_font_variant!(theme, data, sans, regular);
                    set_font_variant!(theme, data, sans, regular_italic);
                    set_font_variant!(theme, data, sans, medium);
                    set_font_variant!(theme, data, sans, medium_italic);
                    set_font_variant!(theme, data, sans, bold);
                    set_font_variant!(theme, data, sans, bold_italic);
                    // Serif
                    set_font_variant!(theme, data, serif, light);
                    set_font_variant!(theme, data, serif, light_italic);
                    set_font_variant!(theme, data, serif, regular);
                    set_font_variant!(theme, data, serif, regular_italic);
                    set_font_variant!(theme, data, serif, medium);
                    set_font_variant!(theme, data, serif, medium_italic);
                    set_font_variant!(theme, data, serif, bold);
                    set_font_variant!(theme, data, serif, bold_italic);
                    // Mono
                    set_font_variant!(theme, data, mono, light);
                    set_font_variant!(theme, data, mono, light_italic);
                    set_font_variant!(theme, data, mono, regular);
                    set_font_variant!(theme, data, mono, regular_italic);
                    set_font_variant!(theme, data, mono, medium);
                    set_font_variant!(theme, data, mono, medium_italic);
                    set_font_variant!(theme, data, mono, bold);
                    set_font_variant!(theme, data, mono, bold_italic);

                    // Update layout
                    theme.layout = UiLayout {
                        z_index: UiZIndex {
                            modal_base: theme.layout.z_index.modal_base,
                        },
                        /* ... copy from plugin's check_theme_asset_readiness ... */
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
                                data.layout.radius.full * effective_font_size * effective_ui_scaling
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
                    theme.color = UiColorPalettes {
                        /* ... copy from plugin's check_theme_asset_readiness ... */
                        white: color_palette(&data.color.white),
                        black: color_palette(&data.color.black),
                        gray: color_palette(&data.color.gray),
                        gray_a: color_palette(&data.color.gray_a),
                        mauve: color_palette(&data.color.mauve),
                        mauve_a: color_palette(&data.color.mauve_a),
                        slate: color_palette(&data.color.slate),
                        slate_a: color_palette(&data.color.slate_a),
                        sage: color_palette(&data.color.sage),
                        sage_a: color_palette(&data.color.sage_a),
                        olive: color_palette(&data.color.olive),
                        olive_a: color_palette(&data.color.olive_a),
                        sand: color_palette(&data.color.sand),
                        sand_a: color_palette(&data.color.sand_a),
                        tomato: color_palette(&data.color.tomato),
                        tomato_a: color_palette(&data.color.tomato_a),
                        red: color_palette(&data.color.red),
                        red_a: color_palette(&data.color.red_a),
                        ruby: color_palette(&data.color.ruby),
                        ruby_a: color_palette(&data.color.ruby_a),
                        crimson: color_palette(&data.color.crimson),
                        crimson_a: color_palette(&data.color.crimson_a),
                        pink: color_palette(&data.color.pink),
                        pink_a: color_palette(&data.color.pink_a),
                        plum: color_palette(&data.color.plum),
                        plum_a: color_palette(&data.color.plum_a),
                        purple: color_palette(&data.color.purple),
                        purple_a: color_palette(&data.color.purple_a),
                        violet: color_palette(&data.color.violet),
                        violet_a: color_palette(&data.color.violet_a),
                        iris: color_palette(&data.color.iris),
                        iris_a: color_palette(&data.color.iris_a),
                        indigo: color_palette(&data.color.indigo),
                        indigo_a: color_palette(&data.color.indigo_a),
                        blue: color_palette(&data.color.blue),
                        blue_a: color_palette(&data.color.blue_a),
                        cyan: color_palette(&data.color.cyan),
                        cyan_a: color_palette(&data.color.cyan_a),
                        teal: color_palette(&data.color.teal),
                        teal_a: color_palette(&data.color.teal_a),
                        jade: color_palette(&data.color.jade),
                        jade_a: color_palette(&data.color.jade_a),
                        green: color_palette(&data.color.green),
                        green_a: color_palette(&data.color.green_a),
                        grass: color_palette(&data.color.grass),
                        grass_a: color_palette(&data.color.grass_a),
                        bronze: color_palette(&data.color.bronze),
                        bronze_a: color_palette(&data.color.bronze_a),
                        gold: color_palette(&data.color.gold),
                        gold_a: color_palette(&data.color.gold_a),
                        brown: color_palette(&data.color.brown),
                        brown_a: color_palette(&data.color.brown_a),
                        orange: color_palette(&data.color.orange),
                        orange_a: color_palette(&data.color.orange_a),
                        amber: color_palette(&data.color.amber),
                        amber_a: color_palette(&data.color.amber_a),
                        yellow: color_palette(&data.color.yellow),
                        yellow_a: color_palette(&data.color.yellow_a),
                        lime: color_palette(&data.color.lime),
                        lime_a: color_palette(&data.color.lime_a),
                        mint: color_palette(&data.color.mint),
                        mint_a: color_palette(&data.color.mint_a),
                        sky: color_palette(&data.color.sky),
                        sky_a: color_palette(&data.color.sky_a),
                    };

                    info!("UiTheme resource hot reloaded.");
                }
            }
            _ => {} // Ignore other asset events like Created, font_sizeoved for now
        }
    }
}

pub fn save_theme_system(theme: Res<UiTheme>, config: Res<UiConfig>) {
    info!("Attempting to save theme...");
    // Hole die effektiven Werte aus dem Theme
    let effective_font_size = config.font_size_base * config.scaling;
    let effective_spacing = config.spacing_factor * config.font_size_base * config.scaling;

    let family = {
        let default = if theme.font.family.default.path().is_some() {
            font_handle_to_path_string(&theme.font.family.default)
        } else {
            String::new()
        };
        let sans = FontVariantsData {
            light: if theme.font.family.sans.light.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.light)
            } else {
                String::new()
            },
            light_italic: if theme.font.family.sans.light_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.light_italic)
            } else {
                String::new()
            },
            regular: if theme.font.family.sans.regular.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.regular)
            } else {
                String::new()
            },
            regular_italic: if theme.font.family.sans.regular_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.regular_italic)
            } else {
                String::new()
            },
            medium: if theme.font.family.sans.medium.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.medium)
            } else {
                String::new()
            },
            medium_italic: if theme.font.family.sans.medium_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.medium_italic)
            } else {
                String::new()
            },
            bold: if theme.font.family.sans.bold.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.bold)
            } else {
                String::new()
            },
            bold_italic: if theme.font.family.sans.bold_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.sans.bold_italic)
            } else {
                String::new()
            },
        };
        let serif = FontVariantsData {
            light: if theme.font.family.serif.light.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.light)
            } else {
                String::new()
            },
            light_italic: if theme.font.family.serif.light_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.light_italic)
            } else {
                String::new()
            },
            regular: if theme.font.family.serif.regular.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.regular)
            } else {
                String::new()
            },
            regular_italic: if theme.font.family.serif.regular_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.regular_italic)
            } else {
                String::new()
            },
            medium: if theme.font.family.serif.medium.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.medium)
            } else {
                String::new()
            },
            medium_italic: if theme.font.family.serif.medium_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.medium_italic)
            } else {
                String::new()
            },
            bold: if theme.font.family.serif.bold.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.bold)
            } else {
                String::new()
            },
            bold_italic: if theme.font.family.serif.bold_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.serif.bold_italic)
            } else {
                String::new()
            },
        };
        let mono = FontVariantsData {
            light: if theme.font.family.mono.light.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.light)
            } else {
                String::new()
            },
            light_italic: if theme.font.family.mono.light_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.light_italic)
            } else {
                String::new()
            },
            regular: if theme.font.family.mono.regular.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.regular)
            } else {
                String::new()
            },
            regular_italic: if theme.font.family.mono.regular_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.regular_italic)
            } else {
                String::new()
            },
            medium: if theme.font.family.mono.medium.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.medium)
            } else {
                String::new()
            },
            medium_italic: if theme.font.family.mono.medium_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.medium_italic)
            } else {
                String::new()
            },
            bold: if theme.font.family.mono.bold.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.bold)
            } else {
                String::new()
            },
            bold_italic: if theme.font.family.mono.bold_italic.path().is_some() {
                font_handle_to_path_string(&theme.font.family.mono.bold_italic)
            } else {
                String::new()
            },
        };
        UiFontFamiliesData {
            default,
            sans,
            serif,
            mono,
        }
    };

    let data = UiThemeData {
        font: UiTypographyData {
            size: UiFontSizeData {
                // Direct copy is fine for f32 values
                xs: theme.font.size.xs / effective_font_size,
                sm: theme.font.size.sm / effective_font_size,
                base: theme.font.size.base / effective_font_size,
                lg: theme.font.size.lg / effective_font_size,
                xl: theme.font.size.xl / effective_font_size,
                x2l: theme.font.size.x2l / effective_font_size,
                x3l: theme.font.size.x3l / effective_font_size,
                x4l: theme.font.size.x4l / effective_font_size,
                x5l: theme.font.size.x5l / effective_font_size,
                x6l: theme.font.size.x6l / effective_font_size,
                x7l: theme.font.size.x7l / effective_font_size,
                x8l: theme.font.size.x8l / effective_font_size,
                x9l: theme.font.size.x9l / effective_font_size,
            },
            family,
        },
        layout: UiLayoutData {
            padding: UiSpacingData {
                xs: theme.layout.padding.xs / effective_spacing,
                sm: theme.layout.padding.sm / effective_spacing,
                base: theme.layout.padding.base / effective_spacing,
                lg: theme.layout.padding.lg / effective_spacing,
                xl: theme.layout.padding.xl / effective_spacing,
                x2l: theme.layout.padding.x2l / effective_spacing,
                x3l: theme.layout.padding.x3l / effective_spacing,
                x4l: theme.layout.padding.x4l / effective_spacing,
                x5l: theme.layout.padding.x5l / effective_spacing,
            },
            margin: UiSpacingData {
                xs: theme.layout.margin.xs / effective_spacing,
                sm: theme.layout.margin.sm / effective_spacing,
                base: theme.layout.margin.base / effective_spacing,
                lg: theme.layout.margin.lg / effective_spacing,
                xl: theme.layout.margin.xl / effective_spacing,
                x2l: theme.layout.margin.x2l / effective_spacing,
                x3l: theme.layout.margin.x3l / effective_spacing,
                x4l: theme.layout.margin.x4l / effective_spacing,
                x5l: theme.layout.margin.x5l / effective_spacing,
            },
            gap: UiSpacingData {
                xs: theme.layout.gap.xs / effective_spacing,
                sm: theme.layout.gap.sm / effective_spacing,
                base: theme.layout.gap.base / effective_spacing,
                lg: theme.layout.gap.lg / effective_spacing,
                xl: theme.layout.gap.xl / effective_spacing,
                x2l: theme.layout.gap.x2l / effective_spacing,
                x3l: theme.layout.gap.x3l / effective_spacing,
                x4l: theme.layout.gap.x4l / effective_spacing,
                x5l: theme.layout.gap.x5l / effective_spacing,
            },
            radius: UiRadiusData {
                xs: theme.layout.radius.xs / effective_font_size,
                sm: theme.layout.radius.sm / effective_font_size,
                base: theme.layout.radius.base / effective_font_size,
                lg: theme.layout.radius.lg / effective_font_size,
                xl: theme.layout.radius.xl / effective_font_size,
                x2l: theme.layout.radius.x2l / effective_font_size,
                x3l: theme.layout.radius.x3l / effective_font_size,
                x4l: theme.layout.radius.x4l / effective_font_size,
                full: if theme.layout.radius.full >= 9999.0 {
                    f32::MAX
                } else {
                    theme.layout.radius.full / effective_font_size
                }, // Annahme: 9
            },
            border: UiSpacingData {
                xs: theme.layout.border.xs,
                sm: theme.layout.border.sm,
                base: theme.layout.border.base,
                lg: theme.layout.border.lg,
                xl: theme.layout.border.xl,
                x2l: theme.layout.border.x2l,
                x3l: theme.layout.border.x3l,
                x4l: theme.layout.border.x4l,
                x5l: theme.layout.border.x5l,
            },
        },
        // Use the correct struct name: UiColorPalettesData
        color: UiColorPalettesData {
            // Convert runtime Colors back to data arrays
            white: to_palette(&theme.color.white),
            black: to_palette(&theme.color.black),
            gray: to_palette(&theme.color.gray),
            gray_a: to_palette(&theme.color.gray_a),
            mauve: to_palette(&theme.color.mauve),
            mauve_a: to_palette(&theme.color.mauve_a),
            slate: to_palette(&theme.color.slate),
            slate_a: to_palette(&theme.color.slate_a),
            sage: to_palette(&theme.color.sage),
            sage_a: to_palette(&theme.color.sage_a),
            olive: to_palette(&theme.color.olive),
            olive_a: to_palette(&theme.color.olive_a),
            sand: to_palette(&theme.color.sand),
            sand_a: to_palette(&theme.color.sand_a),
            tomato: to_palette(&theme.color.tomato),
            tomato_a: to_palette(&theme.color.tomato_a),
            red: to_palette(&theme.color.red),
            red_a: to_palette(&theme.color.red_a),
            ruby: to_palette(&theme.color.ruby),
            ruby_a: to_palette(&theme.color.ruby_a),
            crimson: to_palette(&theme.color.crimson),
            crimson_a: to_palette(&theme.color.crimson_a),
            pink: to_palette(&theme.color.pink),
            pink_a: to_palette(&theme.color.pink_a),
            plum: to_palette(&theme.color.plum),
            plum_a: to_palette(&theme.color.plum_a),
            purple: to_palette(&theme.color.purple),
            purple_a: to_palette(&theme.color.purple_a),
            violet: to_palette(&theme.color.violet),
            violet_a: to_palette(&theme.color.violet_a),
            iris: to_palette(&theme.color.iris),
            iris_a: to_palette(&theme.color.iris_a),
            indigo: to_palette(&theme.color.indigo),
            indigo_a: to_palette(&theme.color.indigo_a),
            blue: to_palette(&theme.color.blue),
            blue_a: to_palette(&theme.color.blue_a),
            cyan: to_palette(&theme.color.cyan),
            cyan_a: to_palette(&theme.color.cyan_a),
            teal: to_palette(&theme.color.teal),
            teal_a: to_palette(&theme.color.teal_a),
            jade: to_palette(&theme.color.jade),
            jade_a: to_palette(&theme.color.jade_a),
            green: to_palette(&theme.color.green),
            green_a: to_palette(&theme.color.green_a),
            grass: to_palette(&theme.color.grass),
            grass_a: to_palette(&theme.color.grass_a),
            bronze: to_palette(&theme.color.bronze),
            bronze_a: to_palette(&theme.color.bronze_a),
            gold: to_palette(&theme.color.gold),
            gold_a: to_palette(&theme.color.gold_a),
            brown: to_palette(&theme.color.brown),
            brown_a: to_palette(&theme.color.brown_a),
            orange: to_palette(&theme.color.orange),
            orange_a: to_palette(&theme.color.orange_a),
            amber: to_palette(&theme.color.amber),
            amber_a: to_palette(&theme.color.amber_a),
            yellow: to_palette(&theme.color.yellow),
            yellow_a: to_palette(&theme.color.yellow_a),
            lime: to_palette(&theme.color.lime),
            lime_a: to_palette(&theme.color.lime_a),
            mint: to_palette(&theme.color.mint),
            mint_a: to_palette(&theme.color.mint_a),
            sky: to_palette(&theme.color.sky),
            sky_a: to_palette(&theme.color.sky_a),
        },
        accent: to_palette(&theme.accent),
        accent_a: to_palette(&theme.accent_a),
        gray_accent: to_palette(&theme.gray_accent),
        gray_accent_a: to_palette(&theme.gray_accent_a),
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
fn to_palette(data: &UiColorPalette) -> UiColorPaletteData {
    UiColorPaletteData {
        step01: to_data_color(&data.step01),
        step02: to_data_color(&data.step02),
        step03: to_data_color(&data.step03),
        step04: to_data_color(&data.step04),
        step05: to_data_color(&data.step05),
        step06: to_data_color(&data.step06),
        step07: to_data_color(&data.step07),
        step08: to_data_color(&data.step08),
        step09: to_data_color(&data.step09),
        step10: to_data_color(&data.step10),
        step11: to_data_color(&data.step11),
        step12: to_data_color(&data.step12),
    }
}
