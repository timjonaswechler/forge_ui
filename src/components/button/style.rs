//! Vereinfachtes `style.rs` nur für Buttons

use super::enums::{ButtonSize, ButtonVariant};
use crate::theme::{UiColorPalette, UiTheme};
use bevy::{prelude::*, utils::default};

/// Bündelt alle Style-Komponenten für Buttons.
#[derive(Bundle, Clone, Debug)]
pub struct ButtonStyle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub text_style: TextFont,
    pub text_color: TextColor,
}

impl ButtonStyle {
    /// Erzeugt das vollständige Style-Bundle für einen Button.
    pub fn new(
        variant: ButtonVariant,
        size: ButtonSize,
        palette: &UiColorPalette,
        interaction: Interaction,
        theme: &UiTheme,
    ) -> Self {
        // Basis-Layout
        let mut node = Node {
            display: Display::Flex,
            padding: UiRect::all(Val::Px(match size {
                ButtonSize::Default => theme.layout.padding.base,
                ButtonSize::Small => theme.layout.padding.sm,
                ButtonSize::Large => theme.layout.padding.lg,
            })),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(theme.layout.gap.base),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };

        // Padding & Schriftgröße
        let font_size = match size {
            ButtonSize::Default => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.base), Val::Px(8.0));
                theme.font.size.base
            }
            ButtonSize::Small => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.sm), Val::Px(4.0));
                theme.font.size.sm
            }
            ButtonSize::Large => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.lg), Val::Px(8.0));
                theme.font.size.lg
            }
        };

        // Colors (alte Logik übernehmen)
        let background_color = Self::background(palette, variant, interaction);

        let border_color = Self::border(palette, variant, interaction);

        let text_style = TextFont {
            font_size,
            ..default()
        };
        let text_color = Self::text_color(palette, variant);

        ButtonStyle {
            node,
            background_color,
            border_color,
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
            text_style,
            text_color,
        }
    }

    pub fn background(
        palette: &UiColorPalette,
        variant: ButtonVariant,
        interaction: Interaction,
    ) -> BackgroundColor {
        match (variant, interaction) {
            (ButtonVariant::Solid, Interaction::None) => BackgroundColor(palette.step09),
            (ButtonVariant::Soft, Interaction::None)
            | (ButtonVariant::Outline, Interaction::None) => BackgroundColor(palette.step02),

            (ButtonVariant::Solid, Interaction::Hovered) => BackgroundColor(palette.step10),
            (ButtonVariant::Soft, Interaction::Hovered)
            | (ButtonVariant::Outline, Interaction::Hovered) => BackgroundColor(palette.step03),

            (ButtonVariant::Solid, Interaction::Pressed) => BackgroundColor(palette.step11),
            (ButtonVariant::Soft, Interaction::Pressed)
            | (ButtonVariant::Outline, Interaction::Pressed) => BackgroundColor(palette.step04),
        }
    }

    pub fn border(
        palette: &UiColorPalette,
        variant: ButtonVariant,
        interaction: Interaction,
    ) -> BorderColor {
        match (variant, interaction) {
            (ButtonVariant::Solid, Interaction::None)
            | (ButtonVariant::Soft, Interaction::None)
            | (ButtonVariant::Solid, Interaction::Hovered)
            | (ButtonVariant::Soft, Interaction::Hovered)
            | (ButtonVariant::Solid, Interaction::Pressed)
            | (ButtonVariant::Soft, Interaction::Pressed) => BorderColor(Color::NONE),
            (ButtonVariant::Outline, Interaction::None)
            | (ButtonVariant::Outline, Interaction::Hovered)
            | (ButtonVariant::Outline, Interaction::Pressed) => BorderColor(palette.step11),
        }
    }

    pub fn text_color(palette: &UiColorPalette, variant: ButtonVariant) -> TextColor {
        match variant {
            ButtonVariant::Solid => TextColor(palette.step02),
            ButtonVariant::Soft | ButtonVariant::Outline => TextColor(palette.step11),
        }
    }
}
