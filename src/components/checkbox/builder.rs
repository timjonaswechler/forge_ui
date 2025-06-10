// src/components/checkbox/builder.rs
use crate::assets::IconAssets;
use crate::components::checkbox::components::{CheckboxMarker, CheckboxState, CheckmarkIconEntity};
use crate::theme::UiTheme;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

/// # CheckboxBuilder
///
/// Ein flexibler Builder zum Erstellen und Konfigurieren von UI-Checkboxen.
///
/// Checkboxen können:
/// - Initialen Zustand (`checked`) setzen
/// - Deaktiviert (`disabled`) sein, um Interaktionen zu unterbinden
/// - Optional zusätzliche Marker oder Komponenten durch Closures hinzufügen
///
/// ## Methoden
///
/// - `CheckboxBuilder::new()` – Erstellt einen neuen Builder mit Standardwerten
/// - `.checked(checked: bool)` – Setzt den initialen Zustand der Checkbox
/// - `.disabled(disabled: bool)` – Deaktiviert die Checkbox optisch und funktional
/// - `.add_marker(func: impl FnOnce(&mut EntityCommands) + Send + Sync)` – Fügt nach dem Spawn benutzerdefinierte Änderungen an der Entity hinzu
/// - `.spawn(parent, &UiTheme, &Res<IconAssets>)` – Spawnt die Checkbox im angegebenen UI-Parent und gibt die daraus resultierenden `EntityCommands` zurück
///
/// ## Beispiel
///
/// ```rust
/// use bevy::prelude::*;
/// use forge_ui::components::checkbox::{CheckboxBuilder, CheckboxChangedEvent};
/// use forge_ui::theme::UiTheme;
///
/// fn setup_ui(
///     mut commands: Commands,
///     theme: Res<UiTheme>,
///     icons: Res<IconAssets>,
/// ) {
///     commands.spawn(NodeBundle::default()).with_children(|parent| {
///         // Standard Checkbox
///         CheckboxBuilder::new()
///             .spawn(parent, &theme, &icons);
///
///         // Vorgecheckte und deaktivierte Checkbox mit zusätzlichem Marker
///         CheckboxBuilder::new()
///             .checked(true)
///             .disabled(true)
///             .add_marker(|ec| { ec.insert(Name::new("DisabledCheckedCheckbox")); })
///             .spawn(parent, &theme, &icons);
///     });
/// }
///
/// fn handle_changes(
///     mut events: EventReader<CheckboxChangedEvent>
/// ) {
///     for ev in events.iter() {
///         info!("Checkbox {:?} is now {}", ev.checkbox_entity, ev.is_checked);
///     }
/// }
/// ```
pub struct CheckboxBuilder {
    checked: bool,
    disabled: bool,
    // Optional: ID für Label-Verknüpfung (HtmlFor)
    // id: Option<String>, // In Bevy weniger direkt relevant, aber für Struktur möglich
    markers: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl Default for CheckboxBuilder {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
            // id: None,
            markers: Vec::new(),
        }
    }
}

impl CheckboxBuilder {
    /// Erstellt einen neuen CheckboxBuilder mit Standardwerten.
    pub fn new() -> Self {
        Self::default()
    }

    /// Setzt den initialen Zustand (checked/un-checked).
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Deaktiviert (true) oder aktiviert (false) die Checkbox.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Fügt eine Closure hinzu, die nach dem Spawnen EntityCommands modifiziert.
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }

    /// Spawnt die Checkbox als Kind im UI-Parent.
    ///
    /// # Parameter
    /// - `parent`: Parent-Kommandos für UI-Children
    /// - `theme`: Eure `UiTheme`-Resource für Styling
    /// - `icons`: Zugriff auf geladene [`IconAssets`](crate::assets::IconAssets)
    ///
    /// # Rückgabe
    /// Gibt `EntityCommands` der ge-spawnten Checkbox zurück, um weitere Modifikationen zu ermöglichen.
    #[must_use = "Commands should generally be used"]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        icons: &Res<IconAssets>,
    ) -> EntityCommands<'a> {
        let checkbox_outer_size = Val::Px(16.0);
        let checkbox_padding = Val::Px(2.0);
        let checkmark_inner_size = Val::Px(12.0);
        let border_width = 1.0;

        let mut checkmark_entity = Entity::PLACEHOLDER;

        let mut checkbox_cmd = parent.spawn((
            CheckboxMarker,
            Button,
            Node {
                width: checkbox_outer_size,
                height: checkbox_outer_size,
                padding: UiRect::all(checkbox_padding),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(border_width)),
                ..default()
            },
            BorderRadius::all(Val::Px(theme.layout.radius.xs)),
            BackgroundColor(Color::NONE),
            BorderColor(theme.color.gray.step06),
            if self.disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            },
            Interaction::default(),
            CheckboxState {
                checked: self.checked,
                disabled: self.disabled,
            },
            CheckmarkIconEntity(checkmark_entity),
        ));

        checkbox_cmd.with_children(|builder| {
            checkmark_entity = builder
                .spawn((
                    Node {
                        width: checkmark_inner_size,
                        height: checkmark_inner_size,
                        margin: UiRect::axes(Val::Px(0.0), Val::Px(2.0)),
                        ..default()
                    },
                    ImageNode {
                        image: icons.0.get("check").expect("missing 'check' icon").clone(),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    FocusPolicy::Pass,
                    if self.checked {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                ))
                .id();
        });

        checkbox_cmd.insert(CheckmarkIconEntity(checkmark_entity));

        for marker_fn in self.markers {
            marker_fn(&mut checkbox_cmd);
        }

        checkbox_cmd
    }
}
