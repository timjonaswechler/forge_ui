// components/button/builder.rs
use super::{
    enums::{ButtonChild, ButtonSize, ButtonVariant},
    style::ButtonStyle,
    {ButtonDisabledOverlayMarker, ButtonMarker, ButtonState},
};

use crate::components::helper::NoAction;
use crate::theme::{UiColorPalette, UiTheme};

use bevy::ecs::spawn::SpawnWith;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

pub struct ButtonBuilder<A: Component + Clone + Send + Sync = NoAction> {
    name: String, // Optional name for the button, useful for debugging
    variant: ButtonVariant,
    color_palette: UiColorPalette,
    size: ButtonSize,
    disabled: bool,
    children_defs: Vec<ButtonChild>,
    action: A,
    width: Option<Val>,
    height: Option<Val>,
    border_radius: Option<Val>,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl<A: Component + Clone + Send + Sync + Default> Default for ButtonBuilder<A> {
    fn default() -> Self {
        Self {
            name: "Button".to_string(),
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(),
            size: ButtonSize::Default,
            disabled: false,
            children_defs: Vec::new(),
            action: A::default(),
            width: None,
            height: None,
            border_radius: None,
            markers: Vec::new(),
        }
    }
}

impl<A: Component + Clone + Send + Sync> ButtonBuilder<A> {
    pub fn new(name: impl Into<String>) -> Self
    where
        ButtonBuilder<A>: Default,
    {
        let mut builder = Self::default();
        builder.name = name.into();
        builder
    }

    /// Sets the visual variant of the button.
    /// See [`ButtonVariant`] for available options.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the size of the button.
    /// Affects padding, minimum height, and font size.
    /// See [`ButtonSize`] for available options.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the color palette for the button.
    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.color_palette = color;
        self
    }

    /// Disables the button.
    ///
    /// A disabled button changes its appearance, is not clickable
    /// (has `FocusPolicy::Pass`), and doesn't send [`ButtonClickedEvent`]s.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Overrides the width of the button.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the height of the button.
    /// This also affects the `min_height`.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    /// Adds text as content to the button.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.children_defs.push(ButtonChild::Text(text.into()));
        self
    }

    /// Associates a specific action instance `A` with this button.
    ///
    /// This `action_instance` is added as a component to the button entity.
    /// On click, a clone of this instance is sent in the [`ButtonClickedEvent<A>`]
    /// in the `action_id` field.
    pub fn action(mut self, action_instance: A) -> Self
    where
        A: Component + Clone + Send + Sync,
    {
        self.action = action_instance;
        self
    }

    /// Sets the corner radius of the button in pixels.
    /// Overrides the default radius from the [`UiTheme`].
    pub fn border_radius(mut self, radius_px: f32) -> Self {
        self.border_radius = Some(Val::Px(radius_px));
        self
    }

    /// Sets the corner radius of the button with a [`Val`].
    /// Overrides the default radius from the [`UiTheme`].
    pub fn border_radius_val(mut self, radius: Val) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Adds a custom function that is applied directly to the [`EntityCommands`]
    /// of the button after spawning.
    ///
    /// Useful for adding any additional components or markers to the button.
    /// The function is executed once.
    ///
    /// # Example
    /// ```rust
    /// # use bevy::prelude::*;
    /// # use forge_ui::components::button::ButtonBuilder;
    /// # use forge_ui::theme::UiTheme;
    /// # #[derive(Resource)]
    /// # struct FontHandle(Handle<Font>);
    /// # #[derive(Component)]
    /// # struct MyCustomMarker;
    /// # let mut commands = Commands::spawn(NodeBundle::default()); // Dummy commands
    /// # let mut child_builder = commands.entity(Entity::PLACEHOLDER).commands();
    /// # let theme = UiTheme::default(); // Dummy theme
    /// # let font_handle = FontHandle(Handle::default()); // Dummy font
    ///
    /// ButtonBuilder::new()
    ///     .text("Marked Button")
    ///     .add_marker(|ecmds| {
    ///         ecmds.insert(MyCustomMarker);
    ///     })
    ///     .spawn(&mut child_builder.reborrow(), &theme, &font_handle.0);
    /// ```
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }

    /// Creates the button entity(ies) as a child of the given `parent` and returns
    /// [`EntityCommands`] for the main button node.
    ///
    /// The `#[must_use]` hint reminds that `EntityCommands` is typically
    /// used further (e.g., to get the ID or attach more children).
    #[must_use]
    pub fn build(self, theme: &UiTheme, font_family: &Handle<Font>) -> impl Bundle {
        let mut color_palette = self.color_palette.clone();
        if color_palette == UiColorPalette::default() {
            color_palette = theme.accent.clone();
        }

        let button_focus_policy = if self.disabled {
            FocusPolicy::Pass
        } else {
            FocusPolicy::Block
        };

        let interaction = if self.disabled {
            Interaction::None
        } else {
            Interaction::default()
        };

        let button_style =
            ButtonStyle::new(self.variant, self.size, &color_palette, interaction, theme);

        // Node-Style aus dem Theme übernehmen und optionale Overrides anwenden
        let mut final_style = button_style.node.clone();
        if let Some(w) = self.width {
            final_style.width = w;
        }
        if let Some(h) = self.height {
            final_style.height = h;
            final_style.min_height = h;
        }

        // Border-Radius ggf. überschreiben
        let border_radius = if let Some(radius) = self.border_radius {
            BorderRadius::all(radius)
        } else {
            button_style.border_radius
        };

        // --- Werte für die 'static Closure klonen ---
        let cloned_name = self.name.clone();
        let cloned_children = self.children_defs;
        let cloned_disabled = self.disabled;
        let overlay_bg_color: Color = theme.color.black.step08.into();
        let text_color = ButtonStyle::text_color(&color_palette, self.variant);
        let font_handle = font_family.clone();
        let font_size = button_style.text_style.font_size;
        let cloned_border_radius = border_radius;
        // Wichtig: Die Aktion wird hier nicht behandelt, da sie nicht Teil des
        // visuellen Bundles ist. Sie muss nach dem Spawnen separat eingefügt werden.

        (
            // --- Komponenten für den Haupt-Button-Knoten ---
            Name::new(self.name.clone()),
            Button,
            final_style,
            button_style.background_color,
            button_style.border_color,
            interaction,
            button_focus_policy,
            border_radius,
            ButtonMarker,
            ButtonState {
                variant: self.variant,
                size: self.size,
                color_palette: color_palette.clone(),
                disabled: self.disabled,
            },
            // --- Definition der Kinder ---
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                // 1. Spawne den Inhalt (Text)
                for child_def in &cloned_children {
                    match child_def {
                        ButtonChild::Text(text) => {
                            parent.spawn((
                                Text::new(text),
                                TextFont {
                                    font: font_handle.clone(),
                                    font_size,
                                    ..default()
                                },
                                text_color.clone(),
                                FocusPolicy::Pass,
                            ));
                        }
                        ButtonChild::Custom(custom) => {
                            parent.spawn(());
                        }
                    }
                }

                // 2. Spawne das Overlay darüber, wenn der Button deaktiviert ist
                if cloned_disabled {
                    let _ = parent.spawn((
                        Name::new(format!("{}_Overlay", cloned_name)),
                        ButtonDisabledOverlayMarker,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            ..default()
                        },
                        BackgroundColor(overlay_bg_color),
                        cloned_border_radius,
                        FocusPolicy::Block, // Blockiert keine Klicks
                    ));
                } else {
                    let _ = parent.spawn(());
                }
            })),
            self.action,
        )
    }
}
