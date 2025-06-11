// components/button/builder.rs
use super::{
    enums::{ButtonChild, ButtonSize, ButtonVariant},
    style::ButtonStyle,
    {ButtonDisabledOverlayMarker, ButtonMarker, ButtonState},
};

use crate::components::helper::NoAction;
use crate::layout::VerticalStackBuilder;
use crate::theme::{UiColorPalette, UiTheme};

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

/// A flexible builder for creating and configuring UI buttons.
///
/// ## Overview
/// `ButtonBuilder<A>` allows creating buttons with various variants, sizes, and content
/// (text and/or icons). They can optionally be disabled to prevent clicks.
///
/// The builder is generic over the action type `A: Component + Clone + Send + Sync + 'static`,
/// allowing application-specific logic to be type-safely linked with button click events.
///
/// ## Generic Action Type
/// - By default, `A = NoAction` is used when no specific action is needed.
/// - Use `.action(action_instance)` to set an instance of `A`.
/// - On click, a `ButtonClickedEvent<A>` is triggered, with its field `action_id: Option<A>`
///   containing a clone of the provided action (or `None` if none was set).
///
/// ## Key Methods
/// - `ButtonBuilder::new()`  
///   Creates a builder for buttons without application-specific action (`NoAction`).
/// - `ButtonBuilder::<MyAction>::new_for_action()`  
///   Explicitly sets the builder to the action type `MyAction` (if `MyAction` doesn't implement `Default` etc.).
/// - `.variant(ButtonVariant)`  
///   Selects the appearance (e.g., `Solid`, `Outline`).
/// - `.size(ButtonSize)`  
///   Determines padding, minimum height, and font size.
/// - `.text("…")` / `.child(...)` / `.vertical_stack(...)`  
///   Adds content to the button.
/// - `.disabled(true)`  
///   Disables clicks and adjusts styling.
/// - `.border_radius(px)` / `.border_radius_val(Val)`  
///   Overrides the corner radius from the theme.
/// - `.width(Val)` / `.height(Val)`  
///   Sets explicit dimensions.
/// - `.add_marker(...)`  
///   Attaches arbitrary components or markers to the button entity.
///
/// ## Example
/// ```rust
/// use bevy::prelude::*;
/// use forge_ui::components::button::{
///     ButtonBuilder, ButtonVariant, ButtonSize, NoAction, ButtonClickedEvent
/// };
///
/// #[derive(Component, Clone, Debug, PartialEq, Eq)]
/// enum MyGameAction { StartGame, OpenSettings }
///
/// fn setup_ui(
///     mut commands: Commands,
///     theme: Res<UiTheme>,
///     font: Res<FontHandle>,
/// ) {
///     commands.spawn(NodeBundle::default()).with_children(|parent| {
///         // Standard button without action
///         ButtonBuilder::new()
///             .text("Standard")
///             .spawn(parent, &theme, &font);
///
///         // Button with a custom action
///         ButtonBuilder::<MyGameAction>::new_for_action()
///             .text("Start Game")
///             .variant(ButtonVariant::Solid)
///             .action(MyGameAction::StartGame)
///             .spawn(parent, &theme, &font);
///     });
/// }
///
/// fn on_click(mut events: EventReader<ButtonClickedEvent<MyGameAction>>) {
///     for evt in events.iter() {
///         if let Some(action) = &evt.action_id {
///             match action {
///                 MyGameAction::StartGame => info!("StartGame pressed"),
///                 MyGameAction::OpenSettings => info!("OpenSettings pressed"),
///             }
///         }
///     }
/// }
/// ```
///
/// See also [`NoAction`] and [`ButtonClickedEvent`].
pub struct ButtonBuilder<A: Component + Clone + Send + Sync + 'static = NoAction> {
    variant: ButtonVariant,
    color_palette: UiColorPalette,
    size: ButtonSize,
    disabled: bool,
    children_defs: Vec<ButtonChild>,
    action: Option<A>, // Speichert die Aktion vom Typ A
    width: Option<Val>,
    height: Option<Val>,
    border_radius: Option<Val>,
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

// Default-Implementierung für ButtonBuilder<NoAction>
impl Default for ButtonBuilder<NoAction> {
    /// Creates a `ButtonBuilder` with default values and action type `NoAction`.
    /// `action` is initially `None`, meaning no `NoAction` component is added to the
    /// entity by default, unless `.action(NoAction)` is explicitly called.
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(),
            size: ButtonSize::Default,
            disabled: false,
            children_defs: Vec::new(),
            action: None,
            width: None,
            height: None,
            border_radius: None,
            markers: Vec::new(),
        }
    }
}

impl<A: Component + Clone + Send + Sync + 'static> ButtonBuilder<A> {
    pub fn new() -> Self
    where
        ButtonBuilder<A>: Default,
    {
        Default::default()
    }
    /// Creates a new builder intended for a specific action type `A`.
    ///
    /// This is useful for explicitly setting the type signature for the builder
    /// when `A` is not `NoAction`, or when `A` doesn't implement `Default`.
    /// The actual action instance is set using the `.action()` method.
    pub fn new_for_action() -> Self {
        ButtonBuilder {
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(),
            size: ButtonSize::Default,
            disabled: false,
            children_defs: Vec::new(),
            action: None,
            width: None,
            height: None,
            border_radius: None,
            markers: Vec::new(),
        }
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

    /// Adds a custom closure as a child.
    pub fn child(
        mut self,
        f: impl FnOnce(&mut ChildSpawnerCommands) + Send + Sync + 'static,
    ) -> Self {
        self.children_defs.push(ButtonChild::Custom(Box::new(f)));
        self
    }

    /// Directly spawns a VerticalStackBuilder in the button.
    pub fn vertical_stack(mut self, vsb: VerticalStackBuilder) -> Self {
        self.children_defs
            .push(ButtonChild::Custom(Box::new(move |parent| {
                vsb.spawn(parent);
            })));
        self
    }
    /// Associates a specific action instance `A` with this button.
    ///
    /// This `action_instance` is added as a component to the button entity.
    /// On click, a clone of this instance is sent in the [`ButtonClickedEvent<A>`]
    /// in the `action_id` field.
    pub fn action(mut self, action_instance: A) -> Self {
        self.action = Some(action_instance);
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
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_family: &Handle<Font>,
    ) -> EntityCommands<'s> {
        let mut color_palette = self.color_palette.clone();

        if color_palette == UiColorPalette::default() {
            color_palette = theme.accent.clone();
        }

        // FocusPolicy für den Button selbst
        let button_focus_policy = if self.disabled {
            FocusPolicy::Pass
        } else {
            FocusPolicy::Block
        };

        //  Interaktionsstatus
        let interaction = if self.disabled {
            Interaction::None // Verhindert Hover-Effekte etc. direkt bei Deaktivierung
        } else {
            Interaction::default()
        };

        let button_style =
            ButtonStyle::new(self.variant, self.size, &color_palette, interaction, theme);

        let text_style = button_style.text_style.clone();
        let font_size = text_style.font_size;

        // Node aus dem Style übernehmen und optionale Overrides anwenden
        let mut node = button_style.node.clone();
        if let Some(w) = self.width {
            node.width = w;
        }
        if let Some(h) = self.height {
            node.height = h;
            node.min_height = h;
        }

        // Border-Radius ggf. überschreiben
        let border_radius = if let Some(radius) = self.border_radius {
            BorderRadius::all(radius)
        } else {
            button_style.border_radius
        };

        let mut cmd = parent.spawn((
            Button,
            node,
            button_style.background_color,
            button_style.border_color,
            border_radius,
            button_focus_policy,
            ButtonMarker,
            ButtonState {
                variant: self.variant,
                size: self.size,
                color_palette: color_palette.clone(),
                disabled: self.disabled,
            },
        ));

        // 5. OnClick Komponente
        if let Some(action_instance) = self.action {
            cmd.insert(action_instance);
        }

        // 6. Marker anwenden
        for marker_fn in self.markers {
            marker_fn(&mut cmd);
        }

        // 7. Kinder spawnen
        let children_defs_empty = self.children_defs.is_empty();
        let children_defs = self.children_defs;
        cmd.with_children(|cb| {
            for child_def in children_defs {
                match child_def {
                    ButtonChild::Text(text) => {
                        cb.spawn((
                            Text::new(text),
                            TextFont {
                                font: font_family.clone(),
                                font_size,
                                ..default()
                            },
                            ButtonStyle::text_color(&color_palette, self.variant),
                        ));
                    }

                    ButtonChild::Custom(func) => {
                        (func)(cb);
                    }
                }
            }

            // Fallback: Wenn keine Kinder definiert wurden, zumindest einen leeren Text hinzufügen
            if children_defs_empty {
                cb.spawn((
                    Text::new(""),
                    TextFont {
                        font: font_family.clone(),
                        font_size,
                        ..default()
                    },
                    ButtonStyle::text_color(&color_palette, self.variant),
                ));
            }
        });

        // Overlay für Disabled-Status
        if self.disabled {
            spawn_disabled_overlay(&mut cmd, theme, border_radius);
        }
        cmd
    }
}

/// Hilfsfunktion zum Spawnen eines Disabled-Overlays
fn spawn_disabled_overlay(cmd: &mut EntityCommands, theme: &UiTheme, border_radius: BorderRadius) {
    cmd.with_children(|parent| {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.color.black.step08),
            ButtonDisabledOverlayMarker,
            FocusPolicy::Block,
            Visibility::Visible,
            ZIndex(1),
            border_radius,
        ));
    });
}
