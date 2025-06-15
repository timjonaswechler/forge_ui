# Template Code Snippet for UI Components in Bevy

## Builder.rs

```rust
// builder.rs - Standard Template
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

use crate::components::helper::NoAction;
use crate::theme::{UiColorPalette, UiTheme};
// Importe für die eigene Komponente anpassen
use super::{
    enums::{ComponentSize, ComponentVariant},
    style::ComponentStyle,
    {ComponentMarker, ComponentState, spawn_disabled_overlay}, // Helper importieren
};

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
    // BEST PRACTICE: BUTTON-Ansatz für maximale Flexibilität übernehmen
    // Statt separaten `text` und `icon` Feldern ist eine Liste von "Child-Definitionen" mächtiger.
    // Das vereinfacht den Builder und verlagert die Komplexität in einen dedizierten Enum.
    children_defs: Vec<ComponentChild>, // Beispiel-Enum, siehe enums.rs

    // ─── Extensions ───
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

// Default-Implementierung für ComponentBuilder<NoAction>
impl Default for ComponentBuilder<NoAction> {
    /// Erstellt einen Builder mit Standardwerten.
    /// Action ist None. Um eine NoAction-Komponente explizit hinzuzufügen,
    /// kann .action(NoAction) verwendet werden.
    fn default() -> Self {
        Self {
            variant: ComponentVariant::default(),
            size: ComponentSize::default(),
            disabled: false,
            // CHANGE: Action ist standardmäßig None, nicht NoAction.
            // Der Button-Ansatz war hier etwas inkonsistent. Keine Action zu haben ist der bessere Default.
            action: None, 
            width: None,
            height: None,
            color_palette: None,
            border_radius: None,
            children_defs: Vec::new(),
            markers: Vec::new(),
        }
    }
}


impl<A: Component + Clone + Send + Sync + 'static> ComponentBuilder<A> {
    /// Standard-Konstruktor für Komponenten ohne spezifische Action.
    pub fn new() -> ComponentBuilder<NoAction> {
        ComponentBuilder::default()
    }
    
    /// Konstruktor für einen Builder mit einem spezifischen Action-Typ.
    pub fn new_for_action() -> Self {
        // REFINED: Wie Button.new_for_action()
        Self::default()
    }
    
    // ─── STANDARD API METHODS (weitgehend wie in deinem Template, aber verfeinert) ───
    
    pub fn variant(mut self, variant: ComponentVariant) -> Self {
        self.variant = variant; self
    }
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size; self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled; self
    }
    pub fn action(mut self, action: A) -> Self {
        self.action = Some(action); self
    }
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width); self
    }
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height); self
    }
    pub fn color(mut self, palette: UiColorPalette) -> Self {
        self.color_palette = Some(palette); self
    }
    pub fn border_radius(mut self, radius_px: f32) -> Self {
        self.border_radius = Some(Val::Px(radius_px)); self
    }
    
    // REFINED: Content-Methoden nach Button-Vorbild für mehr Flexibilität
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.children_defs.push(ComponentChild::Text(text.into())); self
    }
    pub fn icon(mut self, icon: Handle<Image>) -> Self {
        self.children_defs.push(ComponentChild::Icon(icon)); self
    }
    
    pub fn add_marker(mut self, func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static) -> Self {
        self.markers.push(Box::new(func)); self
    }
    
    /// STANDARD SPAWN SIGNATURE (kombiniert Button- und Switch-Logik)
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> EntityCommands<'s> {
        // BEST PRACTICE (von Button): Farbpalette bestimmen
        let color_palette = self.color_palette.clone().unwrap_or_else(|| theme.accent.clone());
        
        let interaction = if self.disabled { Interaction::None } else { Interaction::default() };

        // BEST PRACTICE (von Button): Style zentral erstellen
        let mut component_style = ComponentStyle::new(
            self.variant,
            self.size,
            &color_palette, // Direkt als Referenz übergeben
            interaction,
            theme,
        );

        // REFINED: Style-Overrides aus dem Builder anwenden
        if let Some(w) = self.width { component_style.node.width = w; }
        if let Some(h) = self.height { component_style.node.height = h; }
        if let Some(r) = self.border_radius { component_style.border_radius = BorderRadius::all(r); }

        // Haupt-Komponente spawnen
        let mut cmd = parent.spawn((
            Button, // PFLICHT für Bevy UI Interaction
            ComponentMarker,
            ComponentState {
                variant: self.variant,
                size: self.size,
                disabled: self.disabled,
                color_palette: color_palette.clone(), // Geklonte Palette speichern
                checked: false, // Component-spezifisch anpassen
            },
            // Style-Bundle direkt anwenden
            component_style.clone(),
            // Focus Policy
            if self.disabled { FocusPolicy::Pass } else { FocusPolicy::Block },
        ));

        if let Some(action) = self.action {
            cmd.insert(action);
        }

        // Children spawnen
        cmd.with_children(|child_parent| {
            // BEST PRACTICE (von Button): Durch Child-Definitionen iterieren
            for child_def in self.children_defs {
                match child_def {
                    ComponentChild::Text(text) => {
                        child_parent.spawn((
                            Text::new(text, component_style.text_style(font)),
                            component_style.text_color,
                        ));
                    },
                    ComponentChild::Icon(handle) => {
                        // Icon-spezifisches Spawning
                    },
                }
            }

            // BEST PRACTICE (von Switch): Disabled-Overlay spawnen
            if self.disabled {
                spawn_disabled_overlay(child_parent, theme, component_style.border_radius);
            }
        });

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
```

## Components.rs

```rust
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
    pub variant: ComponentVariant,
    pub size: ComponentSize,
    pub disabled: bool,
    pub color_palette: UiColorPalette,
    pub checked: bool, // Beispiel
}

/// OPTIONAL: Marker für Sub-Components
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ComponentIconMarker;

// BEST PRACTICE (von Switch): Eigener Marker für das Overlay
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
```

## Events.rs

```rust
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
```

## Systems.rs

```rust
// systems.rs - Standard Template
use bevy::{prelude::*, utils::HashMap};
use super::{style::ComponentStyle, *}; 
use crate::theme::UiTheme;

/// PFLICHT: Visual Update System (kombiniert Button- & Switch-Logik)
pub fn update_component_visuals(
    // Haupt-Query für die Komponente selbst
    mut component_query: Query<
        (
            &ComponentState,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Or<(Changed<ComponentState>, Changed<Interaction>)>,
            With<ComponentMarker>
        ),
    >,
    // Separate Query für das Overlay-Child
    mut overlay_query: Query<(&mut Visibility, &mut BackgroundColor), (With<ComponentOverlayMarker>, Without<ComponentMarker>)>,
    // Ggf. weitere Queries für andere Kinder wie Icons...
) {
    for (state, interaction, mut bg_color, mut border_color, children) in component_query.iter_mut() {
        // BEST PRACTICE (von Button): Interaktion für Styling normalisieren
        // Ein deaktiviertes Element verhält sich visuell wie im `None`-Zustand,
        // der eigentliche "disabled"-Look kommt vom Overlay.
        let current_interaction = if state.disabled { Interaction::None } else { *interaction };

        // Farben über die zentrale Style-Logik neu berechnen und setzen
        *bg_color = ComponentStyle::background(&state.color_palette, state.variant, current_interaction);
        *border_color = ComponentStyle::border(&state.color_palette, state.variant, current_interaction);
        
        // BEST PRACTICE (von Switch): Sub-Komponenten steuern
        for &child in children.iter() {
            // Overlay-Sichtbarkeit aktualisieren
            if let Ok((mut overlay_vis, _)) = overlay_query.get_mut(child) {
                *overlay_vis = if state.disabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }

            // Hier könnte auch die Sichtbarkeit von z.B. einem Haken (Icon)
            // basierend auf `state.checked` gesteuert werden.
        }
    }
}

/// PFLICHT: Interaction Handler (direkt vom Button übernommen)
/// Generisch über Action-Type
pub fn handle_component_release<A: Component + Clone + Send + Sync + 'static>(
    mut writer: EventWriter<ComponentChangedEvent<A>>,
    mut prev_interactions: Local<HashMap<Entity, Interaction>>,
    // Query muss den State und optional die Action-Komponente lesen
    mut query: Query<(Entity, &Interaction, &mut ComponentState, Option<&A>), With<ComponentMarker>>,
) {
    for (entity, interaction, mut state, action_opt) in query.iter_mut() {
        if state.disabled {
            prev_interactions.remove(&entity);
            continue;
        }

        let last_interaction = *prev_interactions.get(&entity).unwrap_or(&Interaction::None);
        
        // Logik für "Click" (Release auf dem Element)
        if last_interaction == Interaction::Pressed && (*interaction == Interaction::Hovered || *interaction == Interaction::None) {
            // BEISPIEL: Toggle `checked` state für Komponenten wie Switch/Checkbox
            state.checked = !state.checked;

            writer.send(ComponentChangedEvent {
                source_entity: entity,
                is_checked: state.checked, // Den neuen Zustand senden
                action_id: action_opt.cloned(),
            });
        }
        prev_interactions.insert(entity, *interaction);
    }
}

/// STANDARD: Helper für Disabled-Overlay Spawning (aus deinem Template übernommen, leicht verfeinert)
pub fn spawn_disabled_overlay(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    // Radius vom Parent übernehmen für perfekte Passform
    border_radius: BorderRadius,
) {
    parent.spawn((
        ComponentOverlayMarker,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: theme.color.black.step08.into(), // Halbtransparentes Overlay
            // WICHTIG: Blockiert Klicks auf die darunterliegenden Elemente
            focus_policy: FocusPolicy::Block, 
            // Initial unsichtbar, wird vom System gesteuert
            visibility: Visibility::Hidden, 
            ..default()
        },
        border_radius, // Radius anpassen
    ));
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
```

## Style.rs

```rust
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
```

## Plugin.rs

```rust
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

```

## Enums.rs

```rust
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
