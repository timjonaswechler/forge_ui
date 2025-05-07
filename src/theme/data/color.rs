// crates/forge_ui/src/theme/color.rs

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiColorPalettesData {
    #[serde(default)]
    pub white: UiColorPaletteData,
    #[serde(default)]
    pub black: UiColorPaletteData,
    #[serde(default)]
    pub gray: UiColorPaletteData,
    #[serde(default)]
    pub mauve: UiColorPaletteData,
    #[serde(default)]
    pub slate: UiColorPaletteData,
    #[serde(default)]
    pub sage: UiColorPaletteData,
    #[serde(default)]
    pub olive: UiColorPaletteData,
    #[serde(default)]
    pub sand: UiColorPaletteData,
    #[serde(default)]
    pub tomato: UiColorPaletteData,
    #[serde(default)]
    pub red: UiColorPaletteData,
    #[serde(default)]
    pub ruby: UiColorPaletteData,
    #[serde(default)]
    pub crimson: UiColorPaletteData,
    #[serde(default)]
    pub pink: UiColorPaletteData,
    #[serde(default)]
    pub plum: UiColorPaletteData,
    #[serde(default)]
    pub purple: UiColorPaletteData,
    #[serde(default)]
    pub violet: UiColorPaletteData,
    #[serde(default)]
    pub iris: UiColorPaletteData,
    #[serde(default)]
    pub indigo: UiColorPaletteData,
    #[serde(default)]
    pub blue: UiColorPaletteData,
    #[serde(default)]
    pub cyan: UiColorPaletteData,
    #[serde(default)]
    pub teal: UiColorPaletteData,
    #[serde(default)]
    pub jade: UiColorPaletteData,
    #[serde(default)]
    pub green: UiColorPaletteData,
    #[serde(default)]
    pub grass: UiColorPaletteData,
    #[serde(default)]
    pub bronze: UiColorPaletteData,
    #[serde(default)]
    pub gold: UiColorPaletteData,
    #[serde(default)]
    pub brown: UiColorPaletteData,
    #[serde(default)]
    pub orange: UiColorPaletteData,
    #[serde(default)]
    pub amber: UiColorPaletteData,
    #[serde(default)]
    pub yellow: UiColorPaletteData,
    #[serde(default)]
    pub lime: UiColorPaletteData,
    #[serde(default)]
    pub mint: UiColorPaletteData,
    #[serde(default)]
    pub sky: UiColorPaletteData,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiColorPaletteData {
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
impl Default for UiColorPaletteData {
    fn default() -> Self {
        UiColorPaletteData {
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiAccentColorPaletteData {
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
impl Default for UiAccentColorPaletteData {
    fn default() -> Self {
        UiAccentColorPaletteData {
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct UiGrayAccentColorPaletteData {
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
impl Default for UiGrayAccentColorPaletteData {
    fn default() -> Self {
        UiGrayAccentColorPaletteData {
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
        }
    }
}

impl Default for UiColorPalettesData {
    fn default() -> Self {
        // Hier nur exemplarisch für white und gray – den Rest analog ergänzen!
        UiColorPalettesData {
            white: UiColorPaletteData {
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
            black: UiColorPaletteData {
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
            gray: UiColorPaletteData {
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
            mauve: UiColorPaletteData {
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
            slate: UiColorPaletteData {
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
            sage: UiColorPaletteData {
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
            olive: UiColorPaletteData {
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
            sand: UiColorPaletteData {
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
            tomato: UiColorPaletteData {
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
            red: UiColorPaletteData {
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
            ruby: UiColorPaletteData {
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
            crimson: UiColorPaletteData {
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
            pink: UiColorPaletteData {
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
            plum: UiColorPaletteData {
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
            purple: UiColorPaletteData {
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
            violet: UiColorPaletteData {
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
            iris: UiColorPaletteData {
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
            indigo: UiColorPaletteData {
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
            blue: UiColorPaletteData {
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
            cyan: UiColorPaletteData {
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
            teal: UiColorPaletteData {
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
            jade: UiColorPaletteData {
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
            green: UiColorPaletteData {
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
            grass: UiColorPaletteData {
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
            bronze: UiColorPaletteData {
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
            gold: UiColorPaletteData {
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
            brown: UiColorPaletteData {
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
            orange: UiColorPaletteData {
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
            amber: UiColorPaletteData {
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
            yellow: UiColorPaletteData {
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
            lime: UiColorPaletteData {
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
            mint: UiColorPaletteData {
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
            sky: UiColorPaletteData {
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
        }
    }
}
