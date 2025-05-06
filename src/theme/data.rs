// crates/forge_ui/src/theme/data.rs

use bevy::{asset::Asset, color::*, reflect::Reflect}; // FromReflect importieren
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Asset, Reflect)]
#[serde(default)]
pub struct UiThemeData {
    pub ui_scaling: f32,
    pub rem: f32,

    pub font: UiTypographyData,
    pub layout: UiLayoutData,
    pub color: UiColorDatas,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiTypographyData {
    #[serde(default)]
    pub font_size: UiFontSizeData,
    #[serde(default)]
    pub font_family: UiFontFamiliesData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiFontSizeData {
    #[serde(default)]
    pub xs: f32,
    #[serde(default)]
    pub sm: f32,
    #[serde(default)]
    pub base: f32,
    #[serde(default)]
    pub lg: f32,
    #[serde(default)]
    pub xl: f32,
    #[serde(default)]
    pub x2l: f32,
    #[serde(default)]
    pub x3l: f32,
    #[serde(default)]
    pub x4l: f32,
    #[serde(default)]
    pub x5l: f32,
    #[serde(default)]
    pub x6l: f32,
    #[serde(default)]
    pub x7l: f32,
    #[serde(default)]
    pub x8l: f32,
    #[serde(default)]
    pub x9l: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiFontFamiliesData {
    #[serde(default)]
    pub default: String,

    #[serde(default)]
    pub sans: FontVariantsData,

    #[serde(default)]
    pub serif: FontVariantsData,

    #[serde(default)]
    pub mono: FontVariantsData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct FontVariantsData {
    #[serde(default)]
    pub light: String,
    #[serde(default)]
    pub light_italic: String,
    #[serde(default)]
    pub regular: String,
    #[serde(default)]
    pub regular_italic: String,
    #[serde(default)]
    pub medium: String,
    #[serde(default)]
    pub medium_italic: String,
    #[serde(default)]
    pub bold: String,
    #[serde(default)]
    pub bold_italic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiLayoutData {
    #[serde(default)]
    pub spacing: f32,
    #[serde(default)]
    pub padding: UiSpacingData,
    #[serde(default)]
    pub margin: UiSpacingData,
    #[serde(default)]
    pub gap: UiSpacingData,
    #[serde(default)]
    pub radius: UiRadiusData,
    #[serde(default)]
    pub border: UiSpacingData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiSpacingData {
    #[serde(default)]
    pub xs: f32,
    #[serde(default)]
    pub sm: f32,
    #[serde(default)]
    pub base: f32,
    #[serde(default)]
    pub lg: f32,
    #[serde(default)]
    pub xl: f32,
    #[serde(default)]
    pub x2l: f32,
    #[serde(default)]
    pub x3l: f32,
    #[serde(default)]
    pub x4l: f32,
    #[serde(default)]
    pub x5l: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiRadiusData {
    #[serde(default)]
    pub xs: f32,
    #[serde(default)]
    pub sm: f32,
    #[serde(default)]
    pub base: f32,
    #[serde(default)]
    pub lg: f32,
    #[serde(default)]
    pub xl: f32,
    #[serde(default)]
    pub x2l: f32,
    #[serde(default)]
    pub x3l: f32,
    #[serde(default)]
    pub x4l: f32,
    #[serde(default)]
    pub full: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiColorDatas {
    #[serde(default)]
    pub white: UiColorData,
    #[serde(default)]
    pub black: UiColorData,
    #[serde(default)]
    pub gray: UiColorData,
    #[serde(default)]
    pub mauve: UiColorData,
    #[serde(default)]
    pub slate: UiColorData,
    #[serde(default)]
    pub sage: UiColorData,
    #[serde(default)]
    pub olive: UiColorData,
    #[serde(default)]
    pub sand: UiColorData,
    #[serde(default)]
    pub tomato: UiColorData,
    #[serde(default)]
    pub red: UiColorData,
    #[serde(default)]
    pub ruby: UiColorData,
    #[serde(default)]
    pub crimson: UiColorData,
    #[serde(default)]
    pub pink: UiColorData,
    #[serde(default)]
    pub plum: UiColorData,
    #[serde(default)]
    pub purple: UiColorData,
    #[serde(default)]
    pub violet: UiColorData,
    #[serde(default)]
    pub iris: UiColorData,
    #[serde(default)]
    pub indigo: UiColorData,
    #[serde(default)]
    pub blue: UiColorData,
    #[serde(default)]
    pub cyan: UiColorData,
    #[serde(default)]
    pub teal: UiColorData,
    #[serde(default)]
    pub jade: UiColorData,
    #[serde(default)]
    pub green: UiColorData,
    #[serde(default)]
    pub grass: UiColorData,
    #[serde(default)]
    pub bronze: UiColorData,
    #[serde(default)]
    pub gold: UiColorData,
    #[serde(default)]
    pub brown: UiColorData,
    #[serde(default)]
    pub orange: UiColorData,
    #[serde(default)]
    pub amber: UiColorData,
    #[serde(default)]
    pub yellow: UiColorData,
    #[serde(default)]
    pub lime: UiColorData,
    #[serde(default)]
    pub mint: UiColorData,
    #[serde(default)]
    pub sky: UiColorData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect, Default)]
pub struct UiColorData {
    #[serde(default)]
    pub background_primary: [f32; 4],
    #[serde(default)]
    pub background_secondary: [f32; 4],
    #[serde(default)]
    pub interaction_primary: [f32; 4],
    #[serde(default)]
    pub interaction_secondary: [f32; 4],
    #[serde(default)]
    pub interaction_tertiary: [f32; 4],
    #[serde(default)]
    pub border_primary: [f32; 4],
    #[serde(default)]
    pub border_secondary: [f32; 4],
    #[serde(default)]
    pub border_tertiary: [f32; 4],
    #[serde(default)]
    pub solid_primary: [f32; 4],
    #[serde(default)]
    pub solid_secondary: [f32; 4],
    #[serde(default)]
    pub text_primary: [f32; 4],
    #[serde(default)]
    pub text_secondary: [f32; 4],
}

impl Default for UiThemeData {
    fn default() -> Self {
        // genau wie vorher im Runtime-Default
        const DEFAULT_REM: f32 = 16.0;
        const DEFAULT_UI_SCALING: f32 = 1.0;
        const DEFAULT_SPACING: f32 = 0.25;

        UiThemeData {
            // --- Basis-Werte ---
            ui_scaling: DEFAULT_UI_SCALING,
            rem: DEFAULT_REM,

            // --- Fonts als Pfad-Strings ---
            font: UiTypographyData {
                font_size: UiFontSizeData {
                    xs: 0.75,
                    sm: 0.875,
                    base: 1.0,
                    lg: 1.125,
                    xl: 1.25,
                    x2l: 1.5,
                    x3l: 1.875,
                    x4l: 2.25,
                    x5l: 3.0,
                    x6l: 3.75,
                    x7l: 4.5,
                    x8l: 6.0,
                    x9l: 8.0,
                },
                font_family: UiFontFamiliesData {
                    default: "fonts/Roboto-Regular.ttf".to_string(),
                    sans: FontVariantsData {
                        light: "fonts/Roboto-Light.ttf".to_string(),
                        light_italic: "fonts/Roboto-LightItalic.ttf".to_string(),
                        regular: "fonts/Roboto-Regular.ttf".to_string(),
                        regular_italic: "fonts/Roboto-RegularItalic.ttf".to_string(),
                        medium: "fonts/Roboto-Medium.ttf".to_string(),
                        medium_italic: "fonts/Roboto-MediumItalic.ttf".to_string(),
                        bold: "fonts/Roboto-Bold.ttf".to_string(),
                        bold_italic: "fonts/Roboto-BoldItalic.ttf".to_string(),
                    },
                    serif: FontVariantsData {
                        light: "fonts/NotoSerif-Light.ttf".to_string(),
                        light_italic: "fonts/NotoSerif-LightItalic.ttf".to_string(),
                        regular: "fonts/NotoSerif-Regular.ttf".to_string(),
                        regular_italic: "fonts/NotoSerif-RegularItalic.ttf".to_string(),
                        medium: "fonts/NotoSerif-Medium.ttf".to_string(),
                        medium_italic: "fonts/NotoSerif-MediumItalic.ttf".to_string(),
                        bold: "fonts/NotoSerif-Bold.ttf".to_string(),
                        bold_italic: "fonts/NotoSerif-BoldItalic.ttf".to_string(),
                    },
                    mono: FontVariantsData {
                        light: "fonts/RobotoMono-Light.ttf".to_string(),
                        light_italic: "fonts/RobotoMono-LightItalic.ttf".to_string(),
                        regular: "fonts/RobotoMono-Regular.ttf".to_string(),
                        regular_italic: "fonts/RobotoMono-RegularItalic.ttf".to_string(),
                        medium: "fonts/RobotoMono-Medium.ttf".to_string(),
                        medium_italic: "fonts/RobotoMono-MediumItalic.ttf".to_string(),
                        bold: "fonts/RobotoMono-Bold.ttf".to_string(),
                        bold_italic: "fonts/RobotoMono-BoldItalic.ttf".to_string(),
                    },
                },
            },
            layout: UiLayoutData {
                spacing: DEFAULT_SPACING,
                padding: UiSpacingData {
                    xs: 1.0,
                    sm: 2.0,
                    base: 3.0,
                    lg: 4.0,
                    xl: 5.0,
                    x2l: 6.0,
                    x3l: 7.0,
                    x4l: 8.0,
                    x5l: 9.0,
                },
                margin: UiSpacingData {
                    /* genau wie padding */ ..Default::default()
                },
                gap: UiSpacingData {
                    /* genau wie padding */ ..Default::default()
                },
                radius: UiRadiusData {
                    xs: 0.125,
                    sm: 0.25,
                    base: 0.375,
                    lg: 0.5,
                    xl: 0.75,
                    x2l: 1.0,
                    x3l: 1.5,
                    x4l: 2.0,
                    full: f32::MAX,
                },
                border: UiSpacingData {
                    xs: 1.0,
                    sm: 2.0,
                    base: 3.0,
                    lg: 5.0,
                    xl: 7.0,
                    x2l: 9.0,
                    x3l: 12.0,
                    x4l: 15.0,
                    x5l: 19.0,
                },
            },

            color: UiColorDatas {
                white: UiColorData {
                    background_primary: [1.0, 1.0, 1.0, 0.05],
                    background_secondary: [1.0, 1.0, 1.0, 0.10],
                    interaction_primary: [1.0, 1.0, 1.0, 0.15],
                    interaction_secondary: [1.0, 1.0, 1.0, 0.20],
                    interaction_tertiary: [1.0, 1.0, 1.0, 0.30],
                    border_primary: [1.0, 1.0, 1.0, 0.40],
                    border_secondary: [1.0, 1.0, 1.0, 0.50],
                    border_tertiary: [1.0, 1.0, 1.0, 0.60],
                    solid_primary: [1.0, 1.0, 1.0, 0.70],
                    solid_secondary: [1.0, 1.0, 1.0, 0.80],
                    text_primary: [1.0, 1.0, 1.0, 0.90],
                    text_secondary: [1.0, 1.0, 1.0, 0.95],
                },
                black: UiColorData {
                    background_primary: [0.0, 0.0, 0.0, 0.05],
                    background_secondary: [0.0, 0.0, 0.0, 0.10],
                    interaction_primary: [0.0, 0.0, 0.0, 0.15],
                    interaction_secondary: [0.0, 0.0, 0.0, 0.20],
                    interaction_tertiary: [0.0, 0.0, 0.0, 0.30],
                    border_primary: [0.0, 0.0, 0.0, 0.40],
                    border_secondary: [0.0, 0.0, 0.0, 0.50],
                    border_tertiary: [0.0, 0.0, 0.0, 0.60],
                    solid_primary: [0.0, 0.0, 0.0, 0.70],
                    solid_secondary: [0.0, 0.0, 0.0, 0.80],
                    text_primary: [0.0, 0.0, 0.0, 0.90],
                    text_secondary: [0.0, 0.0, 0.0, 0.95],
                },
                gray: UiColorData {
                    // hex-Farben wandelt Srgba direkt ins [f32;4]-Array um
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                mauve: UiColorData {
                    background_primary: Srgba::hex("121113").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("1a191b").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("232225").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2b292d").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("323035").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3c393f").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("49474e").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("625f69").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6f6d78").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7c7a85").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("b5b2bc").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("eeeef0").unwrap().to_f32_array(),
                },
                slate: UiColorData {
                    background_primary: Srgba::hex("111113").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("18191b").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("212225").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("272a2d").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("2e3135").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("363a3f").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("43484e").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("5a6169").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("696e77").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("777b84").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("b0b4ba").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("edeef0").unwrap().to_f32_array(),
                },
                sage: UiColorData {
                    background_primary: Srgba::hex("101211").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("171918").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("202221").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("272a29").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("2e3130").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("373b39").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("444947").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("5b625f").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("63706b").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("717d79").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("adb5b2").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("eceeed").unwrap().to_f32_array(),
                },
                olive: UiColorData {
                    background_primary: Srgba::hex("111210").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("181917").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("212220").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("282a27").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("2f312e").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("383a36").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("454843").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("5c625b").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("687066").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("767d74").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("afb5ad").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("eceeec").unwrap().to_f32_array(),
                },
                sand: UiColorData {
                    background_primary: Srgba::hex("111110").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191918").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222221").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2a2a28").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("31312e").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3b3a37").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("494844").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("62605b").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6f6d66").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7c7b74").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("b5b3ad").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("eeeeec").unwrap().to_f32_array(),
                },
                tomato: UiColorData {
                    background_primary: Srgba::hex("181111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("1F1513").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("391714").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("4E1511").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("5E1C16").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("6E2920").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("853A2D").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("AC4D39").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("E54D2E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("EC6142").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("FF977D").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("FBD3CB").unwrap().to_f32_array(),
                },
                red: UiColorData {
                    background_primary: Srgba::hex("191111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("201314").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("3b1219").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("500f1c").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("611623").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("72232d").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("8c333a").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("b54548").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("e5484d").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("ec5d5e").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("ff9592").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("ffd1d9").unwrap().to_f32_array(),
                },
                ruby: UiColorData {
                    background_primary: Srgba::hex("191113").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("1e1517").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("3a141e").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("4e1325").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("5e1a2e").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("6f2539").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("883447").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("b3445a").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("e54666").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("ec5a72").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("ff949d").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("fed2e1").unwrap().to_f32_array(),
                },
                crimson: UiColorData {
                    background_primary: Srgba::hex("191114").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("201318").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("381525").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("4d122f").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("5c1839").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("6d2545").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("873356").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("b0436e").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("e93d82").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("ee518a").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("ff92ad").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("fdd3e8").unwrap().to_f32_array(),
                },
                pink: UiColorData {
                    background_primary: Srgba::hex("191117").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("21121d").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("37172f").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("4b143d").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("591c47").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("692955").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("833869").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("a84885").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("d6409f").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("de51a8").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("ff8dcc").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("fdd1ea").unwrap().to_f32_array(),
                },
                plum: UiColorData {
                    background_primary: Srgba::hex("181118").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("201320").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("351a35").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("451d47").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("512454").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("5e3061").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("734079").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("92549c").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("ab4aba").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("b658c4").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("e796f3").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("f4d4f4").unwrap().to_f32_array(),
                },
                purple: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                violet: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                iris: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                indigo: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                blue: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                cyan: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                teal: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                jade: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                green: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                grass: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                bronze: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                gold: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                brown: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                orange: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                amber: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                yellow: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                lime: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                mint: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
                sky: UiColorData {
                    background_primary: Srgba::hex("111111").unwrap().to_f32_array(),
                    background_secondary: Srgba::hex("191919").unwrap().to_f32_array(),
                    interaction_primary: Srgba::hex("222222").unwrap().to_f32_array(),
                    interaction_secondary: Srgba::hex("2A2A2A").unwrap().to_f32_array(),
                    interaction_tertiary: Srgba::hex("313131").unwrap().to_f32_array(),
                    border_primary: Srgba::hex("3A3A3A").unwrap().to_f32_array(),
                    border_secondary: Srgba::hex("484848").unwrap().to_f32_array(),
                    border_tertiary: Srgba::hex("606060").unwrap().to_f32_array(),
                    solid_primary: Srgba::hex("6E6E6E").unwrap().to_f32_array(),
                    solid_secondary: Srgba::hex("7B7B7B").unwrap().to_f32_array(),
                    text_primary: Srgba::hex("B4B4B4").unwrap().to_f32_array(),
                    text_secondary: Srgba::hex("EEEEEE").unwrap().to_f32_array(),
                },
            },
        }
    }
}
