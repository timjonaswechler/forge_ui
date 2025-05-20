use super::enums::{ButtonSize, ButtonVariant};
use crate::theme::{UiColorPalette, UiTheme};
use bevy::{prelude::*, utils::default};

/// Bundles all style components for buttons.
///
/// This structure combines all visual properties of a button and
/// provides methods to generate consistent styles based on variant, size,
/// and interaction state.
#[derive(Bundle, Clone, Debug)]
pub struct ButtonStyle {
    /// Defines layout properties like padding, alignment and spacing
    pub node: Node,
    /// Background color of the button
    pub background_color: BackgroundColor,
    /// Border color of the button
    pub border_color: BorderColor,
    /// Border radius for the button corners
    pub border_radius: BorderRadius,
    /// Font and size for the button text
    pub text_style: TextFont,
    /// Text color for the button text
    pub text_color: TextColor,
}

impl ButtonStyle {
    /// Creates the complete style bundle for a button.
    ///
    /// Generates all style components based on the provided parameters
    /// and ensures the button adheres to the design system.
    ///
    /// # Parameters
    /// * `variant` - The button variant (Solid, Soft, Outline)
    /// * `size` - The button size (Default, Small, Large)
    /// * `palette` - The color palette for the UI
    /// * `interaction` - The current interaction state of the button
    /// * `theme` - The UI theme with layout and typography definitions
    ///
    /// # Returns
    /// A complete `ButtonStyle` bundle with all necessary components
    pub fn new(
        variant: ButtonVariant,
        size: ButtonSize,
        palette: &UiColorPalette,
        interaction: Interaction,
        theme: &UiTheme,
    ) -> Self {
        // Basic layout
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

        // Padding & font size
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

        // Colors (use existing logic)
        let background_color = Self::background(palette, variant, interaction);

        let border_color = Self::border(palette, variant, interaction);

        let text_style = TextFont {
            font_size,
            ..default()
        };
        let text_color = Self::text_color(palette, variant);

        ButtonStyle {
            node: node,
            background_color: background_color,
            border_color: border_color,
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
            text_style: text_style,
            text_color: text_color,
        }
    }

    /// Determines the background color of a button.
    ///
    /// Selects the appropriate background color based on variant and interaction state.
    ///
    /// # Parameters
    /// * `palette` - The color palette for the UI
    /// * `variant` - The button variant
    /// * `interaction` - The current interaction state
    ///
    /// # Returns
    /// The `BackgroundColor` component with the corresponding color
    pub fn background(
        palette: &UiColorPalette,
        variant: ButtonVariant,
        interaction: Interaction,
    ) -> BackgroundColor {
        match (variant, interaction) {
            (ButtonVariant::Solid, Interaction::None) => BackgroundColor(palette.step09),
            (ButtonVariant::Ghost, Interaction::None) => BackgroundColor(palette.step01),
            (ButtonVariant::Soft, Interaction::None)
            | (ButtonVariant::Outline, Interaction::None) => BackgroundColor(palette.step03),

            (ButtonVariant::Solid, Interaction::Hovered) => BackgroundColor(palette.step10),
            (ButtonVariant::Ghost, Interaction::Hovered) => BackgroundColor(palette.step03),
            (ButtonVariant::Soft, Interaction::Hovered)
            | (ButtonVariant::Outline, Interaction::Hovered) => BackgroundColor(palette.step04),

            (ButtonVariant::Solid, Interaction::Pressed) => BackgroundColor(palette.step11),
            (ButtonVariant::Ghost, Interaction::Pressed) => BackgroundColor(palette.step04),
            (ButtonVariant::Soft, Interaction::Pressed)
            | (ButtonVariant::Outline, Interaction::Pressed) => BackgroundColor(palette.step05),
        }
    }

    /// Determines the border color of a button.
    ///
    /// Selects the appropriate border color based on variant and interaction state.
    /// For Solid and Soft variants, no border is displayed (Color::NONE).
    ///
    /// # Parameters
    /// * `palette` - The color palette for the UI
    /// * `variant` - The button variant
    /// * `interaction` - The current interaction state
    ///
    /// # Returns
    /// The `BorderColor` component with the corresponding color
    pub fn border(
        palette: &UiColorPalette,
        variant: ButtonVariant,
        interaction: Interaction,
    ) -> BorderColor {
        match (variant, interaction) {
            (ButtonVariant::Solid, Interaction::None)
            | (ButtonVariant::Solid, Interaction::Hovered)
            | (ButtonVariant::Solid, Interaction::Pressed)
            | (ButtonVariant::Soft, Interaction::None)
            | (ButtonVariant::Soft, Interaction::Hovered)
            | (ButtonVariant::Soft, Interaction::Pressed)
            | (ButtonVariant::Ghost, Interaction::None)
            | (ButtonVariant::Ghost, Interaction::Pressed)
            | (ButtonVariant::Ghost, Interaction::Hovered) => BorderColor(Color::NONE),
            (ButtonVariant::Outline, Interaction::None)
            | (ButtonVariant::Outline, Interaction::Hovered)
            | (ButtonVariant::Outline, Interaction::Pressed) => BorderColor(palette.step11),
        }
    }

    /// Determines the text color of a button.
    ///
    /// Selects the appropriate text color based on the button variant.
    /// For Solid buttons, a lighter color is used, while for
    /// Soft and Outline variants, a darker color is used.
    ///
    /// # Parameters
    /// * `palette` - The color palette for the UI
    /// * `variant` - The button variant
    ///
    /// # Returns
    /// The `TextColor` component with the corresponding color
    pub fn text_color(palette: &UiColorPalette, variant: ButtonVariant) -> TextColor {
        match variant {
            ButtonVariant::Solid => TextColor(palette.step02),
            ButtonVariant::Soft | ButtonVariant::Outline | ButtonVariant::Ghost => {
                TextColor(palette.step11)
            }
        }
    }
}
