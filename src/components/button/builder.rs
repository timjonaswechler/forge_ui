// components/button/builder.rs
use super::enums::{ButtonChild, ButtonSize, ButtonVariant};
use super::style::{apply_size_style, base_style, get_button_style_def};
use super::{ButtonMarker, ButtonState}; // NoAction importieren
use crate::components::helper::NoAction;
use crate::theme::UiTheme;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

/// # ButtonBuilder
///
/// Ein flexibler Builder zum Erstellen und Konfigurieren von UI-Buttons.
///
/// Buttons können verschiedene visuelle Varianten (`ButtonVariant`), Größen (`ButtonSize`),
/// Text- und/oder Icon-Inhalte haben und optional deaktiviert sein.
///
/// ## Aktionen und Events
///
/// Der `ButtonBuilder` ist generisch über den Aktionstyp `A: Component + Clone + Send + Sync + 'static`.
/// Dies ermöglicht es, anwendungsspezifische Logik typsicher mit Button-Klicks zu verknüpfen.
///
/// - Verwenden Sie die Methode `.action(action_instance: A)`, um eine Aktions-Komponente
///   zur Button-Entität hinzuzufügen.
/// - Wenn ein Button geklickt wird, wird ein [`ButtonClickedEvent<A>`] gesendet.
/// - Das `action_id: Option<A>` Feld in diesem Event enthält ein Klon der Aktions-Komponente,
///   falls eine mit `.action()` auf dem Builder gesetzt wurde.
///
/// Für Buttons, die keine spezifische, anwendungsspezifische Aktion auslösen sollen,
/// wird der Standard-Typ `NoAction` für `A` verwendet. `ButtonBuilder::new()` erstellt
/// einen solchen Builder.
///
/// ## Verwendung
///
/// ```rust
/// use bevy::prelude::*;
/// use forge_ui::components::button::{ButtonBuilder, ButtonVariant, ButtonSize, NoAction, ButtonClickedEvent};
/// use forge_ui::theme::UiTheme; // Annahme: Theme und FontHandle sind Ressourcen
///
/// #[derive(Resource)]
/// struct FontHandle(Handle<Font>); // Beispiel für FontHandle
///
/// // Eigene Aktions-Enum
/// #[derive(Component, Clone, Debug, PartialEq, Eq)]
/// pub enum MyGameAction {
///     StartGame,
///     OpenSettings,
/// }
///
/// fn setup_ui_system(
///     mut commands: Commands,
///     theme: Res<UiTheme>,        // Theme wird als Ressource erwartet
///     font_handle: Res<FontHandle> // Font-Handle wird als Ressource erwartet
/// ) {
///     commands.spawn(NodeBundle::default()).with_children(|parent| {
///         // Button ohne spezifische Aktion
///         ButtonBuilder::new()
///             .text("Standard-Button")
///             .spawn(parent, &theme, &font_handle);
///
///         // Button mit einer benutzerdefinierten Aktion
///         ButtonBuilder::<MyGameAction>::new_for_action()
///             .text("Spiel starten")
///             .variant(ButtonVariant::Default) // Annahme: Primary ist eine Variante
///             .action(MyGameAction::StartGame)
///             .spawn(parent, &theme, &font_handle);
///     });
/// }
///
/// fn handle_button_clicks(mut events: EventReader<ButtonClickedEvent<MyGameAction>>) {
///     for event in events.read() {
///         if let Some(action) = &event.action_id {
///             match action {
///                 MyGameAction::StartGame => {
///                     info!("Button zum Starten des Spiels geklickt!");
///                 }
///                 MyGameAction::OpenSettings => {
///                     info!("Button zum Öffnen der Einstellungen geklickt!");
///                 }
///             }
///         }
///     }
/// }
///
/// // In der App-Konfiguration (vereinfacht):
///  App::new()
///      .insert_resource(ForgeUiPlugin)
///      .add_event::<ButtonClickedEvent<NoAction>>()
///      .add_event::<ButtonClickedEvent<MyGameAction>>()
///      .add_systems(Update, (
///          // Für Reaktion beim Drücken:
///          forge_ui::components::button::handle_button_press::<MyGameAction>,
///          // Für Klick beim Loslassen:
///           forge_ui::components::button::handle_button_release::<MyGameAction>,
///          handle_button_clicks, // Das eigene System
///      ))
///      .add_systems(Startup, setup_ui_system)
///      .run();
/// ```
///
/// Siehe auch [`NoAction`] für den Standard-Aktionstyp und [`ButtonClickedEvent`] für das gesendete Event.
pub struct ButtonBuilder<A: Component + Clone + Send + Sync + 'static = NoAction> {
    variant: ButtonVariant,
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
    /// Erstellt einen `ButtonBuilder` mit Standardwerten und dem Aktionstyp `NoAction`.
    /// `action` ist initial `None`, d.h. es wird standardmäßig keine `NoAction`-Komponente
    /// zur Entität hinzugefügt, es sei denn, `.action(NoAction)` wird explizit aufgerufen.
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            children_defs: Vec::new(),
            action: None, // Standardmäßig keine Aktion
            width: None,
            height: None,
            border_radius: None,
            markers: Vec::new(),
        }
    }
}

// new() Methode für den häufigsten Fall (Button ohne spezifische externe Aktion)
impl ButtonBuilder<NoAction> {
    /// Erstellt einen neuen `ButtonBuilder` für einen Button ohne spezifische anwendungsdefinierte Aktion.
    ///
    /// Der generische Aktionstyp `A` ist hier [`NoAction`].
    /// Um eine Aktion hinzuzufügen (selbst eine `NoAction`), verwenden Sie die `.action()` Methode.
    /// Ohne expliziten `.action(NoAction)`-Aufruf wird dem gesendeten `ButtonClickedEvent<NoAction>`
    /// im Feld `action_id` ein `None` übergeben.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Component + Clone + Send + Sync + 'static> ButtonBuilder<A> {
    /// Erstellt einen neuen Builder, der für eine spezifische Aktion `A` vorgesehen ist.
    ///
    /// Dies ist nützlich, um die Typsignatur für den Builder explizit festzulegen,
    /// wenn `A` nicht `NoAction` ist, oder wenn `A` nicht `Default` implementiert.
    /// Die eigentliche Aktionsinstanz wird mit der `.action()` Methode gesetzt.
    pub fn new_for_action() -> Self {
        ButtonBuilder {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            children_defs: Vec::new(),
            action: None, // Aktion wird über .action() gesetzt
            width: None,
            height: None,
            border_radius: None,
            markers: Vec::new(),
        }
    }

    /// Setzt die visuelle Variante des Buttons.
    /// Siehe [`ButtonVariant`] für verfügbare Optionen.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Setzt die Größe des Buttons.
    /// Beeinflusst Padding, Mindesthöhe und Schriftgröße.
    /// Siehe [`ButtonSize`] für verfügbare Optionen.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Deaktiviert den Button.
    ///
    /// Ein deaktivierter Button ändert sein Aussehen, ist nicht klickbar
    /// (hat `FocusPolicy::Pass`) und sendet keine [`ButtonClickedEvent`]s.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Überschreibt die Breite des Buttons.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Überschreibt die Höhe des Buttons.
    /// Dies beeinflusst auch die `min_height`.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    /// Fügt ein Icon als Inhalt des Buttons hinzu.
    ///
    /// Wenn die Button-Größe [`ButtonSize::Icon`] ist, wird jeglicher Textinhalt entfernt
    /// und nur dieses Icon angezeigt. Ansonsten wird das Icon dem Text vorangestellt oder
    /// zu anderen Icons hinzugefügt.
    pub fn icon(mut self, icon_handle: Handle<Image>) -> Self {
        if self.size == ButtonSize::Icon {
            self.children_defs.clear(); // Nur ein Icon bei ButtonSize::Icon
        }
        self.children_defs.push(ButtonChild::Icon(icon_handle));
        self
    }

    /// Fügt einen Text als Inhalt des Buttons hinzu.
    ///
    /// Wird nicht hinzugefügt, wenn die Button-Größe [`ButtonSize::Icon`] ist.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        if self.size != ButtonSize::Icon {
            self.children_defs.push(ButtonChild::Text(text.into()));
        }
        self
    }

    /// Verknüpft eine spezifische Aktionsinstanz `A` mit diesem Button.
    ///
    /// Diese `action_instance` wird als Komponente zur Entität des Buttons hinzugefügt.
    /// Beim Klick wird ein Klon dieser Instanz im [`ButtonClickedEvent<A>`]
    /// im Feld `action_id` mitgesendet.
    pub fn action(mut self, action_instance: A) -> Self {
        self.action = Some(action_instance);
        self
    }

    /// Setzt den Eckenradius des Buttons in Pixeln.
    /// Überschreibt den Standardradius aus dem [`UiTheme`].
    pub fn border_radius(mut self, radius_px: f32) -> Self {
        self.border_radius = Some(Val::Px(radius_px));
        self
    }

    /// Setzt den Eckenradius des Buttons mit einem [`Val`].
    /// Überschreibt den Standardradius aus dem [`UiTheme`].
    pub fn border_radius_val(mut self, radius: Val) -> Self {
        self.border_radius = Some(radius);
        self
    }

    /// Fügt eine benutzerdefinierte Funktion hinzu, die direkt auf die [`EntityCommands`]
    /// des Buttons nach dem Spawnen angewendet wird.
    ///
    /// Nützlich, um dem Button beliebige zusätzliche Komponenten oder Marker hinzuzufügen.
    /// Die Funktion wird einmalig ausgeführt.
    ///
    /// # Beispiel
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
    /// ButtonBuilder::new()
    ///     .text("Markierter Button")
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

    /// Erstellt die Button-Entität(en) als Kind des gegebenen `parent` und gibt
    /// [`EntityCommands`] für den Haupt-Button-Node zurück.
    ///
    /// Der `#[must_use]` Hinweis erinnert daran, dass `EntityCommands` in der Regel
    /// weiterverwendet werden (z.B. um die ID zu erhalten oder weitere Kinder anzuhängen).
    #[must_use]
    pub fn spawn<'w, 's>(
        // Geändert zu 's für Konsistenz mit Dialog-spawn Lifetime
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) -> EntityCommands<'s> {
        // 1. Style Def holen
        let style_def = get_button_style_def(self.variant, theme);

        // 2. Basis-Style vorbereiten und Grössen-Style anwenden
        let mut node_style = base_style();
        let font_size = apply_size_style(&mut node_style, self.size, theme);

        // Setze BorderRadius *nach* apply_size_style, falls Icon es überschreiben soll
        // node_style.border_radius = BorderRadius::all(Val::Px(theme.layout.radius.base));

        // 3. Width/Height Overrides
        if let Some(w) = self.width {
            node_style.width = w;
        }
        if let Some(h) = self.height {
            node_style.height = h;
            if let (Val::Px(current), Val::Px(h_px)) = (node_style.min_height, h) {
                if current < h_px {
                    node_style.min_height = h;
                }
            }
        }
        let final_radius = self
            .border_radius
            .unwrap_or_else(|| Val::Px(theme.layout.radius.base)); // <<< Radius Logik

        // 4. Button Entity spawnen
        let mut cmd = parent.spawn((
            Button,                                                 // Bevy Komponente
            Node { ..node_style },                                  // Angewendeter Style
            style_def.background(Interaction::None, self.disabled), // Initialer Hintergrund
            style_def.border(Interaction::None, self.disabled),     // Initialer Rand
            if self.disabled {
                FocusPolicy::Pass
            } else {
                FocusPolicy::Block
            },
            Interaction::default(),
            // Custom Components
            ButtonMarker,
            ButtonState {
                variant: self.variant,
                size: self.size,
                disabled: self.disabled,
                // style_def: style_def.clone(),
            },
            BorderRadius::all(final_radius), // <<< Setze den BorderRadius
        ));

        // 5. OnClick Komponente
        // Füge die benutzerdefinierte Aktion als Komponente hinzu
        if let Some(action_instance) = self.action {
            cmd.insert(action_instance);
        }

        // 6. Marker anwenden
        for marker_fn in self.markers {
            marker_fn(&mut cmd);
        }

        // 7. Kinder spawnen
        cmd.with_children(|cb| {
            for child_def in self.children_defs {
                match child_def {
                    ButtonChild::Text(text) => {
                        // Hole die korrekte initiale Textfarbe (ggf. Disabled)
                        let initial_text_color = style_def.text_color(self.disabled); // <<< Neu: text_color Methode nutzen
                        cb.spawn((
                            Text::new(text), // Besser als TextBundle hier, wenn TextColor separat gesetzt wird
                            TextFont {
                                font: font_handle.clone(),
                                font_size,
                                ..default()
                            },
                            TextColor(initial_text_color), // <<< Setze angepasste Farbe
                        ));
                    }
                    ButtonChild::Icon(handle) => {
                        let icon_size_val = match self.size {
                            ButtonSize::Small => theme.font.font_size.sm,
                            ButtonSize::Icon => theme.font.font_size.lg,
                            _ => theme.font.font_size.base,
                        };
                        let icon_size = Val::Px(icon_size_val);
                        // Hole die korrekte initiale Icon-Tint-Farbe (ggf. Disabled)
                        let initial_icon_tint = style_def.text_color(self.disabled);

                        cb.spawn((
                            Node {
                                width: icon_size,
                                height: icon_size,
                                margin: UiRect::axes(Val::Px(0.0), Val::Px(2.0)),
                                ..default()
                            },
                            ImageNode {
                                image: handle,
                                ..default()
                            },
                            BackgroundColor(initial_icon_tint),
                            FocusPolicy::Pass,
                            Visibility::Inherited,
                        ))
                        .insert(BackgroundColor(
                            style_def
                                .text_color
                                .with_alpha(if self.disabled { 0.6 } else { 1.0 }),
                        )); // Start tinted like text color
                    }
                }
            }
        });

        cmd
    }
}
