# Template Code Snippet for UI Components in Bevy

```rust
//builder.rs - Standard Template
// Template für alle Component Builder
use bevy::prelude::*;
use crate::components::helper::NoAction;
use crate::theme::UiTheme;

/// Standard Builder Template für UI Components
pub struct ComponentBuilder<A: Component + Clone + Send + Sync + 'static = NoAction> {
    // ─── Core Properties ───
    variant: ComponentVariant,
    size: ComponentSize,
    disabled: bool,
    
    // ─── Interaction ───
    action: Option<A>,
    
    // ─── Styling ───
    width: Option<Val>,
    height: Option<Val>,
    color_palette: Option<UiColorPalette>,
    border_radius: Option<Val>,
    
    // ─── Content ───
    text: Option<String>,
    icon: Option<Handle<Image>>,
    
    // ─── Extensions ───
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl<A: Component + Clone + Send + Sync + 'static> ComponentBuilder<A> {
    /// Standard constructor für alle Components
    pub fn new() -> ComponentBuilder<NoAction> {
        ComponentBuilder::<NoAction>::new_with_action_type()
    }
    
    /// Explicit action type constructor
    pub fn new_with_action_type() -> Self {
        Self {
            variant: ComponentVariant::default(),
            size: ComponentSize::default(),
            disabled: false,
            action: None,
            width: None,
            height: None,
            color_palette: None,
            border_radius: None,
            text: None,
            icon: None,
            markers: Vec::new(),
        }
    }
    
    // ─── STANDARD API METHODS ───
    
    /// Set visual variant
    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant;
        self
    }
    
    /// Set size variant
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
    
    /// Enable/disable component
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    /// Set action component
    pub fn action(mut self, action: A) -> Self {
        self.action = Some(action);
        self
    }
    
    /// Set text content
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    
    /// Set icon
    pub fn icon(mut self, icon: Handle<Image>) -> Self {
        self.icon = Some(icon);
        self
    }
    
    /// Override width
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }
    
    /// Override height
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }
    
    /// Set color palette
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.color_palette = Some(palette);
        self
    }
    
    /// Set border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = Some(Val::Px(radius));
        self
    }
    
    /// Add custom marker/component
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }
    
    /// STANDARD SPAWN SIGNATURE für alle Components
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>, // Immer mit Font für Konsistenz
    ) -> EntityCommands<'s> {
        // BUTTON-METHODIK: Style über ComponentStyle::new() erstellen
        let color_palette = self.color_palette.unwrap_or_else(|| theme.accent.clone());
        
        // Initiale Interaction für Style-Berechnung
        let interaction = if self.disabled {
            Interaction::None // Disabled components haben keine Interaction
        } else {
            Interaction::default()
        };

        // Style mit Button-Methodik erstellen
        let component_style = ComponentStyle::new(
            self.variant,
            self.size,
            Some(&color_palette),
            interaction,
            theme,
        );

        // Haupt-Component spawnen
        let mut cmd = parent.spawn((
            ComponentMarker,
            Button, // Für Interaction (falls interaktiv)
            ComponentState {
                variant: self.variant,
                size: self.size,
                disabled: self.disabled,
                color_palette: Some(color_palette),
                checked: false, // Component-spezifisch anpassen
            },
            // Style-Bundle direkt anwenden
            component_style.node,
            component_style.background_color,
            component_style.border_color,
            component_style.border_radius,
            // Focus Policy
            if self.disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            },
        ));

        // Action component hinzufügen
        if let Some(action) = self.action {
            cmd.insert(action);
        }

        // Children spawnen
        cmd.with_children(|parent| {
            // Text spawnen (falls vorhanden)
            if let Some(text) = self.text {
                parent.spawn((
                    Text::new(text),
                    component_style.text_style,
                    component_style.text_color,
                ));
            }

            // Icon spawnen (falls vorhanden)
            if let Some(icon_handle) = self.icon {
                parent.spawn((
                    ComponentIconMarker,
                    ImageNode {
                        image: icon_handle,
                        ..default()
                    },
                    Node {
                        width: Val::Px(16.0), // Size-abhängig
                        height: Val::Px(16.0),
                        ..default()
                    },
                    Visibility::Hidden, // Initial versteckt, wird über State gesteuert
                ));
            }

            // SWITCH-DISABLE-SYSTEM: Disabled-Overlay spawnen
            if self.disabled {
                spawn_disabled_overlay(
                    parent, 
                    theme
                );
            }
        });

        // Custom Marker anwenden
        for marker_fn in self.markers {
            marker_fn(&mut cmd);
        }

        cmd
    }
}

// Default implementation nur für NoAction
impl Default for ComponentBuilder<NoAction> {
    fn default() -> Self {
        Self::new()
    }
}

// components.rs - Standard Template
use bevy::prelude::*;
use super::enums::{ComponentVariant, ComponentSize};
use crate::theme::UiColorPalette;

/// PFLICHT: Marker Component für jede UI Component
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ComponentMarker;

/// PFLICHT: State Component mit allen relevanten Zustandsdaten
#[derive(Component, Debug, Clone)]
pub struct ComponentState {
    /// Visual variant (PFLICHT)
    pub variant: ComponentVariant,
    /// Size variant (PFLICHT)
    pub size: ComponentSize,
    /// Disabled state (PFLICHT)
    pub disabled: bool,
    /// Optional: Color override
    pub color_palette: Option<UiColorPalette>,
    /// Component-specific state
    pub checked: bool, // Beispiel für component-spezifische Eigenschaften
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            variant: ComponentVariant::default(),
            size: ComponentSize::default(),
            disabled: false,
            color_palette: None,
            checked: false,
        }
    }
}

/// OPTIONAL: Zusätzliche Marker für Sub-Components (z.B. Icons, Overlays)
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ComponentIconMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ComponentOverlayMarker;

/// STANDARD: State-Update Helper
impl ComponentState {
    pub fn is_interactive(&self) -> bool {
        !self.disabled
    }
    
    pub fn toggle_checked(&mut self) {
        if self.is_interactive() {
            self.checked = !self.checked;
        }
    }
}

// events.rs - Standard Template
use bevy::prelude::*;
use std::fmt::Debug;

/// STANDARD: Hauptevent für Component-Interaktionen
/// Generisch über Action-Type für Konsistenz mit Button-Pattern
#[derive(Event, Clone)]
pub struct ComponentChangedEvent<A: Component + Clone + Send + Sync + 'static> {
    /// Entity der Component (PFLICHT)
    pub source_entity: Entity,
    /// Neuer Zustand (component-spezifisch anpassen)
    pub is_checked: bool,
    /// Optional: Action von der Component
    pub action_id: Option<A>,
}

/// STANDARD: Debug implementation die nicht versucht A zu debuggen
impl<A: Component + Clone + Send + Sync + 'static> Debug for ComponentChangedEvent<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentChangedEvent")
            .field("source_entity", &self.source_entity)
            .field("is_checked", &self.is_checked)
            .field("action_id", &self.action_id.as_ref().map(|_| "Some<A>"))
            .finish_non_exhaustive()
    }
}

/// OPTIONAL: Zusätzliche Events für komplexe Components
#[derive(Event, Debug, Clone)]
pub struct ComponentFocusEvent {
    pub entity: Entity,
    pub focused: bool,
}

/// STANDARD: Event-Constructor Helper
impl<A: Component + Clone + Send + Sync + 'static> ComponentChangedEvent<A> {
    pub fn new(entity: Entity, is_checked: bool, action: Option<A>) -> Self {
        Self {
            source_entity: entity,
            is_checked,
            action_id: action,
        }
    }
}

// systems.rs - Standard Template
use bevy::prelude::*;
use super::*;
use crate::theme::UiTheme;

/// PFLICHT: Visual Update System für jede interaktive Component
/// Verwendet Button-Style-Methodik mit Switch-Disable-System
/// Reagiert auf State-Änderungen und Interaction-Änderungen
pub fn update_component_visuals(
    theme_opt: Option<Res<UiTheme>>,
    mut query: Query<
        (
            &ComponentState,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&Children>, // Für Sub-Components
        ),
        (
            Or<(Changed<ComponentState>, Changed<Interaction>)>,
            With<ComponentMarker>
        ),
    >,
    // Separate Query für Sub-Components (z.B. Icons)
    mut icon_query: Query<&mut Visibility, With<ComponentIconMarker>>,
    mut overlay_query: Query<&mut Visibility, With<ComponentOverlayMarker>>,
) {
    // Guard für Theme
    let Some(theme) = theme_opt else {
        warn!("UiTheme nicht verfügbar für update_component_visuals");
        return;
    };

    for (state, interaction, mut bg_color, mut border_color, children_opt) in query.iter_mut() {
        // BUTTON-METHODIK: Style über statische Methoden bestimmen
        let color_palette = state.color_palette.as_ref().unwrap_or(&theme.accent);

        // Disabled-state wird NICHT über Farbänderung gehandhabt!
        // Stattdessen: Switch-Disable-System mit Overlay
        let current_interaction = if state.disabled { 
            Interaction::None // Disabled components zeigen normale Farben
        } else { 
            *interaction 
        };

        // Farben über Button-Style-Methoden setzen
        *bg_color = ComponentStyle::background(color_palette, state.variant, current_interaction);
        *border_color = ComponentStyle::border(color_palette, state.variant, current_interaction);

        // Sub-Components aktualisieren
        if let Some(children) = children_opt {
            for &child in children {
                // Icon Visibility basierend auf checked state
                if let Ok(mut icon_vis) = icon_query.get_mut(child) {
                    *icon_vis = if state.checked {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }

                // SWITCH-DISABLE-SYSTEM: Overlay-Visibility für Disabled-State
                if let Ok(mut overlay_vis) = overlay_query.get_mut(child) {
                    *overlay_vis = if state.disabled {
                        Visibility::Visible  // Overlay wird sichtbar und blockiert Interaktion
                    } else {
                        Visibility::Hidden   // Overlay versteckt, normale Interaktion möglich
                    };
                }
            }
        }
    }
}

/// PFLICHT: Interaction Handler für jede interaktive Component
/// Generisch über Action-Type
pub fn handle_component_interaction<A: Component + Clone + Send + Sync + 'static>(
    mut query: Query<
        (Entity, &Interaction, &mut ComponentState, Option<&A>),
        (Changed<Interaction>, With<ComponentMarker>),
    >,
    mut events: EventWriter<ComponentChangedEvent<A>>,
) {
    for (entity, interaction, mut state, action_opt) in query.iter_mut() {
        if *interaction == Interaction::Pressed && state.is_interactive() {
            // State ändern (component-spezifisch anpassen)
            state.toggle_checked();
            
            // Event senden
            events.write(ComponentChangedEvent::new(
                entity,
                state.checked,
                action_opt.cloned(),
            ));
        }
    }
}

/// OPTIONAL: Keyboard Navigation für komplexe Components
pub fn handle_component_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut ComponentState), With<ComponentMarker>>,
    mut events: EventWriter<ComponentChangedEvent<crate::components::helper::NoAction>>,
) {
    // Implementation für Keyboard-Handling
    // z.B. Space/Enter für Aktivierung
}
// style.rs - Standard Template (basierend auf Button-Methodik)
use bevy::prelude::*;
use super::enums::{ComponentVariant, ComponentSize};
use crate::theme::{UiTheme, UiColorPalette};

/// STANDARD: Vollständiges Style-Bundle (nach Button-Vorbild)
/// Kombiniert alle visuellen Eigenschaften einer Component
#[derive(Bundle, Clone, Debug)]
pub struct ComponentStyle {
    /// Layout-Eigenschaften wie Padding, Alignment, Spacing
    pub node: Node,
    /// Hintergrundfarbe der Component
    pub background_color: BackgroundColor,
    /// Randfarbe der Component
    pub border_color: BorderColor,
    /// Rand-Radius für die Ecken
    pub border_radius: BorderRadius,
    /// Schrift und Größe für Text-Inhalte
    pub text_style: TextFont,
    /// Textfarbe
    pub text_color: TextColor,
}

impl ComponentStyle {
    /// STANDARD: Erstellt komplettes Style-Bundle (nach Button.new() Methodik)
    /// 
    /// Generiert alle Style-Komponenten basierend auf den gegebenen Parametern
    /// und stellt sicher, dass die Component dem Design-System entspricht.
    ///
    /// # Parameter
    /// * `variant` - Visuelle Variante der Component (Primary, Secondary, etc.)
    /// * `size` - Größenvariante der Component (Small, Medium, Large)
    /// * `palette` - Farbpalette für die UI (kann None sein für Standard-Theme-Farben)
    /// * `interaction` - Aktueller Interaktionszustand
    /// * `theme` - UI-Theme mit Layout- und Typografie-Definitionen
    ///
    /// # Returns
    /// Vollständiges `ComponentStyle` Bundle mit allen nötigen Komponenten
    pub fn new(
        variant: ComponentVariant,
        size: ComponentSize,
        palette: Option<&UiColorPalette>,
        interaction: Interaction,
        theme: &UiTheme,
    ) -> Self {
        // Basis-Layout bestimmen
        let mut node = Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };

        // Size-basierte Eigenschaften (nach Button-Muster)
        let font_size = match size {
            ComponentSize::ExtraSmall => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.xs), Val::Px(2.0));
                theme.font.size.xs
            }
            ComponentSize::Small => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.sm), Val::Px(4.0));
                theme.font.size.sm
            }
            ComponentSize::Medium => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.base), Val::Px(8.0));
                theme.font.size.base
            }
            ComponentSize::Large => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.lg), Val::Px(12.0));
                theme.font.size.lg
            }
            ComponentSize::ExtraLarge => {
                node.padding = UiRect::axes(Val::Px(theme.layout.padding.xl), Val::Px(16.0));
                theme.font.size.xl
            }
        };

        // Farbpalette bestimmen (Standard vs. override)
        let color_palette = palette.unwrap_or(&theme.accent);

        // Farben basierend auf Variante und Interaktion (nach Button-Muster)
        let background_color = Self::background(color_palette, variant, interaction);
        let border_color = Self::border(color_palette, variant, interaction);
        let text_color = Self::text_color(color_palette, variant);

        ComponentStyle {
            node,
            background_color,
            border_color,
            border_radius: BorderRadius::all(Val::Px(theme.layout.radius.base)),
            text_style: TextFont {
                font_size,
                ..default()
            },
            text_color,
        }
    }

    /// STANDARD: Bestimmt Hintergrundfarbe (nach Button-Methodik)
    pub fn background(
        palette: &UiColorPalette,
        variant: ComponentVariant,
        interaction: Interaction,
    ) -> BackgroundColor {
        match (variant, interaction) {
            // Primary/Solid Varianten
            (ComponentVariant::Primary, Interaction::None) => BackgroundColor(palette.step09),
            (ComponentVariant::Primary, Interaction::Hovered) => BackgroundColor(palette.step10),
            (ComponentVariant::Primary, Interaction::Pressed) => BackgroundColor(palette.step11),

            // Secondary/Soft Varianten
            (ComponentVariant::Secondary, Interaction::None) => BackgroundColor(palette.step03),
            (ComponentVariant::Secondary, Interaction::Hovered) => BackgroundColor(palette.step04),
            (ComponentVariant::Secondary, Interaction::Pressed) => BackgroundColor(palette.step05),

            // Outline Varianten
            (ComponentVariant::Outline, Interaction::None) => BackgroundColor(Color::NONE),
            (ComponentVariant::Outline, Interaction::Hovered) => BackgroundColor(palette.step02),
            (ComponentVariant::Outline, Interaction::Pressed) => BackgroundColor(palette.step03),

            // Ghost Varianten
            (ComponentVariant::Ghost, Interaction::None) => BackgroundColor(Color::NONE),
            (ComponentVariant::Ghost, Interaction::Hovered) => BackgroundColor(palette.step02),
            (ComponentVariant::Ghost, Interaction::Pressed) => BackgroundColor(palette.step03),

            // Default Varianten
            (ComponentVariant::Default, Interaction::None) => BackgroundColor(palette.step01),
            (ComponentVariant::Default, Interaction::Hovered) => BackgroundColor(palette.step02),
            (ComponentVariant::Default, Interaction::Pressed) => BackgroundColor(palette.step03),

            // Destructive Varianten
            (ComponentVariant::Destructive, Interaction::None) => BackgroundColor(palette.step09),
            (ComponentVariant::Destructive, Interaction::Hovered) => BackgroundColor(palette.step10),
            (ComponentVariant::Destructive, Interaction::Pressed) => BackgroundColor(palette.step11),
        }
    }

    /// STANDARD: Bestimmt Randfarbe (nach Button-Methodik)
    pub fn border(
        palette: &UiColorPalette,
        variant: ComponentVariant,
        interaction: Interaction,
    ) -> BorderColor {
        match variant {
            // Nur Outline zeigt Ränder
            ComponentVariant::Outline => BorderColor(palette.step07),
            // Alle anderen: keine sichtbaren Ränder
            _ => BorderColor(Color::NONE),
        }
    }

    /// STANDARD: Bestimmt Textfarbe (nach Button-Methodik)
    pub fn text_color(palette: &UiColorPalette, variant: ComponentVariant) -> TextColor {
        match variant {
            // Primary/Destructive: Heller Text auf dunklem Hintergrund
            ComponentVariant::Primary | ComponentVariant::Destructive => {
                TextColor(palette.step01)
            }
            // Alle anderen: Dunkler Text
            _ => TextColor(palette.step11),
        }
    }
}

/// STANDARD: Disabled-Overlay System (nach Switch-Vorbild)
/// 
/// Das Switch-Component demonstriert die beste Praxis für Disabled-Styling:
/// - Erstellt ein absolut positioniertes Overlay als Child der Haupt-Component
/// - Das Overlay ist transparent/halbtransparent und blockiert Interaktionen
/// - Sichtbarkeit wird über das State-System gesteuert
/// - Erhält `FocusPolicy::Block` um Clicks abzufangen
/// 
/// # Implementierung in Builder:
/// ```rust
/// if self.disabled {
///     cmd.with_children(|parent| {
///         parent.spawn((
///             ComponentOverlayMarker,
///             Node {
///                 position_type: PositionType::Absolute,
///                 left: Val::Px(0.0),
///                 top: Val::Px(0.0),
///                 width: Val::Percent(100.0),
///                 height: Val::Percent(100.0),
///                 ..default()
///             },
///             BackgroundColor(theme.color.black.step08), // Semi-transparent overlay
///             BorderRadius::all(Val::Px(border_radius)), // Matched parent radius
///             FocusPolicy::Block, // Blockiert Interaktionen
///             Visibility::Visible,
///         ));
///     });
/// }
/// ```
/// 
/// # Vorteile dieses Approaches:
/// - Visuell konsistent: Overlay-Effekt statt Farbänderung
/// - Funktional robust: Blockiert alle Interaktionen zuverlässig
/// - Flexibel: Overlay kann nach Bedarf gestylt werden
/// - Performance: Nur gespawnt wenn needed, visibility-gesteuert
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ComponentOverlayMarker;

/// STANDARD: Helper für Disabled-Overlay Spawning
pub fn spawn_disabled_overlay(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    border_radius: f32,
) {
    parent.spawn((
        ComponentOverlayMarker,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(theme.color.black.step08),
        BorderRadius::all(Val::Px(border_radius)),
        FocusPolicy::Block,
        Visibility::Visible,
    ));
}
// plugin.rs - Standard Template
use bevy::prelude::*;
use std::marker::PhantomData;

use super::*;
use crate::components::helper::NoAction;
use crate::plugin::UiState;

/// STANDARD: Generisches Plugin für jede UI Component
pub struct ComponentPlugin<A: Component + Clone + Send + Sync + 'static = NoAction>(
    PhantomData<A>
);

impl<A: Component + Clone + Send + Sync + 'static> Default for ComponentPlugin<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<A: Component + Clone + Send + Sync + 'static> Plugin for ComponentPlugin<A> {
    fn build(&self, app: &mut App) {
        app
            // Events registrieren
            .add_event::<ComponentChangedEvent<A>>()
            
            // Systeme registrieren
            .add_systems(
                Update,
                (
                    // Interaction Handling
                    handle_component_interaction::<A>,
                    // Visual Updates
                    update_component_visuals,
                    // Optional: Keyboard Handling
                    // handle_component_keyboard,
                )
                .run_if(in_state(UiState::Ready))
            );
    }
}

/// CONVENIENCE: Standard Plugin für NoAction
pub type ComponentNoActionPlugin = ComponentPlugin<NoAction>;

// VERWENDUNG in main.rs oder plugin-Registrierung:
// app.add_plugins(ComponentPlugin::<MyAction>::default())
// oder
// app.add_plugins(ComponentNoActionPlugin::default())

// enums.rs - Standard Template
use bevy::prelude::*;

/// PFLICHT: Variant-Enum für visuelle Stile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentVariant {
    /// Standard-Variante (Standard)
    #[default]
    Default,
    /// Primär-Akzent (wichtige Aktionen)
    Primary,
    /// Sekundär (weniger wichtige Aktionen)
    Secondary,
    /// Destruktiv (Lösch-/Warnaktionen)
    Destructive,
    /// Nur Umrandung (subtile Aktionen)
    Outline,
    /// Geist-Variante (sehr subtil)
    Ghost,
}

/// PFLICHT: Size-Enum für Größenvarianten
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentSize {
    /// Extra kleine Größe
    ExtraSmall,
    /// Kleine Größe
    Small,
    /// Standard-Größe
    #[default]
    Medium,
    /// Große Größe
    Large,
    /// Extra große Größe
    ExtraLarge,
}

/// OPTIONAL: Component-spezifische Enums
/// Beispiel für Orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// OPTIONAL: Component-spezifische Selection-Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentSelectionType {
    #[default]
    Single,
    Multiple,
}

/// STANDARD: Helper-Traits für Enums
impl ComponentVariant {
    /// Gibt zurück ob die Variante visuell hervorgehoben ist
    pub fn is_prominent(&self) -> bool {
        matches!(self, Self::Primary | Self::Destructive)
    }
    
    /// Gibt zurück ob die Variante subtil ist
    pub fn is_subtle(&self) -> bool {
        matches!(self, Self::Ghost | Self::Outline)
    }
}

impl ComponentSize {
    /// Multiplier für size-basierte Berechnungen
    pub fn scale_factor(&self) -> f32 {
        match self {
            Self::ExtraSmall => 0.7,
            Self::Small => 0.85,
            Self::Medium => 1.0,
            Self::Large => 1.2,
            Self::ExtraLarge => 1.5,
        }
    }
}
```
