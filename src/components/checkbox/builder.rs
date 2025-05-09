use crate::components::checkbox::components::{CheckboxMarker, CheckboxState, CheckmarkIconEntity};
use crate::theme::UiTheme;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

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
    pub fn new() -> Self {
        Self::default()
    }

    /// Setzt den initialen Zustand der Checkbox.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Deaktiviert die Checkbox (visuell und funktional).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    // Optional: Methode zum Hinzufügen einer ID
    // pub fn id(mut self, id: impl Into<String>) -> Self {
    //     self.id = Some(id.into());
    //     self
    // }

    /// Fügt eine generische Closure hinzu, um die EntityCommands nach dem Spawnen zu modifizieren.
    pub fn add_marker(
        mut self,
        func: impl FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    ) -> Self {
        self.markers.push(Box::new(func));
        self
    }

    /// Spawnt die Checkbox als Kind des UI-Parents.
    #[must_use = "Commands should generally be used"]
    pub fn spawn<'w, 'a>(
        self,
        parent: &'a mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        checkmark_icon_handle: &Handle<Image>,
    ) -> EntityCommands<'a> {
        // --- Styling-Konstanten ANPASSEN ---
        let checkbox_outer_size = Val::Px(16.0); // Gesamtgröße der Box
        let checkbox_padding = Val::Px(2.0); // Innenabstand
                                             // Icon-Größe = Gesamtgröße - 2 * Padding (ungefähr, damit es passt)
        let checkmark_inner_size = Val::Px(12.0); // = 16 - 2*2
        let border_width = 1.0;
        // --- Ende Anpassung ----

        // Entität für das Checkmark-Icon (wird gleich gespawnt)
        let mut checkmark_entity = Entity::PLACEHOLDER;

        let mut checkbox_cmd = parent.spawn((
            CheckboxMarker, // Marker
            Button,         // Damit sie klickbar ist
            Node {
                width: checkbox_outer_size,
                height: checkbox_outer_size,
                padding: UiRect::all(checkbox_padding),
                // Wichtig für Icon-Zentrierung
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // Rand
                border: UiRect::all(Val::Px(border_width)),
                // Rundung (angepasst von 'rounded-sm')
                ..default()
            },
            BorderRadius::all(Val::Px(theme.layout.radius.xs)), // Kleinerer Radius als bei Buttons
            // Start-Styling (wird im System aktualisiert)
            BackgroundColor(Color::NONE),
            BorderColor(theme.color.gray.step06),
            // FocusPolicy
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
            // Anfangs Platzhalter, wird unten aktualisiert
            CheckmarkIconEntity(checkmark_entity),
        ));

        // Spawn Checkmark Icon als Kind
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
                        image: checkmark_icon_handle.clone(),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    FocusPolicy::Pass,
                    // Startet unsichtbar, wenn nicht checked
                    if self.checked {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                ))
                .id();
        });

        // Update CheckmarkIconEntity mit der echten Entity ID
        // TODO: Wenn hier ein Fehler auftritt, könnte es sein, dass die Entity nicht existiert oder so einen Playeholder ID nutzen.

        checkbox_cmd.insert(CheckmarkIconEntity(checkmark_entity));

        // Marker hinzufügen
        for marker_fn in self.markers {
            marker_fn(&mut checkbox_cmd);
        }

        checkbox_cmd
    }
}
