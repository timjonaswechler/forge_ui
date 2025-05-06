// src/assets/fonts.rs

use bevy::asset::Asset;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Asset, TypePath)]
pub struct FontAssets {
    // Default
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub default: Handle<bevy::prelude::Font>,

    // Sans
    #[asset(path = "fonts/Roboto-Light.ttf")]
    pub sans_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-LightItalic.ttf")]
    pub sans_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Regular.ttf")]
    pub sans_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-RegularItalic.ttf")]
    pub sans_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Medium.ttf")]
    pub sans_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-MediumItalic.ttf")]
    pub sans_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-Bold.ttf")]
    pub sans_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/Roboto-BoldItalic.ttf")]
    pub sans_bold_italic: Handle<bevy::prelude::Font>,

    // Serif
    #[asset(path = "fonts/NotoSerif-Light.ttf")]
    pub serif_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-LightItalic.ttf")]
    pub serif_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Regular.ttf")]
    pub serif_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-RegularItalic.ttf")]
    pub serif_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Medium.ttf")]
    pub serif_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-MediumItalic.ttf")]
    pub serif_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-Bold.ttf")]
    pub serif_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/NotoSerif-BoldItalic.ttf")]
    pub serif_bold_italic: Handle<bevy::prelude::Font>,

    // Mono
    #[asset(path = "fonts/RobotoMono-Light.ttf")]
    pub mono_light: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-LightItalic.ttf")]
    pub mono_light_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Regular.ttf")]
    pub mono_regular: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-RegularItalic.ttf")]
    pub mono_regular_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Medium.ttf")]
    pub mono_medium: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-MediumItalic.ttf")]
    pub mono_medium_italic: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-Bold.ttf")]
    pub mono_bold: Handle<bevy::prelude::Font>,
    #[asset(path = "fonts/RobotoMono-BoldItalic.ttf")]
    pub mono_bold_italic: Handle<bevy::prelude::Font>,
}
