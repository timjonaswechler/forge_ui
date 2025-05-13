use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::*;
use crate::components::helper::NoAction;
///
/// # Toggle Module
///
/// The `Toggle` module provides functionality for creating toggleable UI components.
/// These components can be used to switch between two states, such as on/off.
///
/// ## Structures
///
/// - `ToggleBuilder`: A builder for creating toggle components.
/// - `ToggleMarker`: A marker component for identifying toggle entities.
/// - `ToggleState`: Represents the state of a toggle (e.g., on or off).
/// - `ToggleChangedEvent`: An event that is sent when the toggle state changes.
/// - `ToggleSize`: Defines the size of the toggle component.
/// - `ToggleVariant`: Defines the visual variant of the toggle component.
/// - `ToggleStyleDef`: Defines the style of the toggle component.
/// - `ToggleStateColors`: Defines the colors for different toggle states.
///
/// ## Enums
///
/// - `TogglePlugin`: A plugin for integrating toggle functionality into the application.
///
/// ## Examples
///
/// ```rust
/// use bevy::prelude::*;
/// use forge_ui::components::toggle::*;
///
/// fn setup_toggle(mut commands: Commands) {
///     ToggleBuilder::new()
///         .size(ToggleSize::Medium)
///         .variant(ToggleVariant::Default)
///         .build(&mut commands);
/// }
/// ```
///
/// ## Related Modules
///
/// - `button`: For creating button components.
/// - `checkbox`: For creating checkbox components.
pub struct ToggleBuilder<A: Component + Clone + Send + Sync + 'static = NoAction> {
    pressed: bool,
    disabled: bool,
    variant: ToggleVariant,
    size: ToggleSize,
    action: Option<A>,
    icon: Option<Handle<Image>>, // optionales Icon
}

impl<A: Component + Clone + Send + Sync + 'static> ToggleBuilder<A> {
    /// Konstruktion mit Defaults (Option 3: spezifischer Aktionstyp)
    pub fn new_with_action_type() -> Self {
        Self {
            pressed: false,
            disabled: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Medium,
            action: None,
            icon: None,
        }
    }

    /// Shortcut für `ToggleBuilder<NoAction>`
    pub fn new() -> ToggleBuilder<NoAction>
    where
        Self: Sized,
    {
        ToggleBuilder::<NoAction>::new_with_action_type()
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: Handle<Image>) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn action(mut self, action: A) -> Self {
        self.action = Some(action);
        self
    }

    /// Spawnt den Toggle als Child‑Entity
    pub fn spawn_into<'w>(
        self,
        parent: &mut ChildSpawnerCommands<'w>,
        theme: &crate::theme::UiTheme,
    ) {
        let style_def = get_toggle_style_def(theme, self.variant, self.size);

        // Haupt‑Button‑Node
        let mut cmd = parent.spawn((
            Node {
                width: Val::Px(style_def.size_px),
                height: Val::Px(style_def.size_px),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.gray_accent.step10),
            BorderRadius::all(Val::Px(style_def.border_radius)),
            FocusPolicy::Pass,
        ));

        cmd.insert((
            ToggleMarker,
            ToggleState {
                pressed: self.pressed,
                disabled: self.disabled,
                variant: self.variant,
                size: self.size,
            },
        ));

        // Aktions‑Komponente
        if let Some(action) = self.action {
            cmd.insert(action);
        }

        // Icon‑Kind (falls gesetzt)
        if let Some(icon_handle) = self.icon {
            cmd.with_children(|parent| {
                parent.spawn(ImageNode {
                    image: icon_handle.clone(),
                    ..default()
                });
            });
        }

        // Disabled‑Overlay
        if self.disabled {
            spawn_disabled_overlay(&mut cmd, theme);
        }
    }
}

/// Hilfsfunktion zum Spawnen eines Disabled‑Overlays (siehe template.md §2.2)
fn spawn_disabled_overlay<'w>(cmd: &mut EntityCommands<'w>, theme: &crate::theme::UiTheme) {
    cmd.with_children(|parent| {
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.color.black.step05),
            FocusPolicy::Pass,
        ));
    });
}
