// components/button/builder.rs
use super::enums::{ButtonChild, ButtonSize, ButtonVariant};
use super::style::ButtonStyle;
use super::{ButtonMarker, ButtonState}; // NoAction importieren
use crate::components::helper::NoAction;
use crate::layout::VerticalStackBuilder;
use crate::theme::{UiColorPalette, UiTheme};
use bevy::ecs::error::info;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::text::cosmic_text::ttf_parser::colr;
use bevy::ui::FocusPolicy;

/// Ein flexibler Builder zur Erstellung und Konfiguration von UI-Buttons.
///
/// ## Überblick
/// Mit `ButtonBuilder<A>` lassen sich Buttons mit verschiedenen Varianten, Größen und Inhalten
/// (Text und/oder Icons) erzeugen. Optional können sie deaktiviert werden, um Klicks zu unterbinden.
///
/// Der Builder ist generisch über den Aktionstyp `A: Component + Clone + Send + Sync + 'static`,
/// sodass anwendungsspezifische Logik typsicher mit Button-Klick-Events verknüpft werden kann.
///
/// ## Generischer Aktionstyp
/// - Standardmäßig wird `A = NoAction` verwendet, wenn keine spezifische Aktion nötig ist.
/// - Über `.action(action_instance)` kann eine Instanz von `A` gesetzt werden.
/// - Beim Klick wird ein `ButtonClickedEvent<A>` ausgelöst, dessen Feld `action_id: Option<A>`
///   den Klon der übergebenen Aktion enthält (oder `None`, falls keine gesetzt wurde).
///
/// ## Wichtige Methoden
/// - `ButtonBuilder::new()`  
///   Erstellt einen Builder für Buttons ohne Anwendungsspezifische Aktion (`NoAction`).
/// - `ButtonBuilder::<MyAction>::new_for_action()`  
///   Stellt den Builder explizit auf den Aktionstyp `MyAction` ein (falls `MyAction` nicht `Default` o. ä.).
/// - `.variant(ButtonVariant)`  
///   Wählt das Aussehen (z. B. `Solid`, `Outline`).
/// - `.size(ButtonSize)`  
///   Bestimmt Padding, Mindesthöhe und Schriftgröße.
/// - `.text("…")` / `.child(...)` / `.vertical_stack(...)`  
///   Fügt Inhalt zum Button hinzu.
/// - `.disabled(true)`  
///   Deaktiviert Klicks und passt das Styling an.
/// - `.border_radius(px)` / `.border_radius_val(Val)`  
///   Überschreibt den Eckradius aus dem Theme.
/// - `.width(Val)` / `.height(Val)`  
///   Setzt explizite Abmessungen.
/// - `.add_marker(...)`  
///   Hängt beliebige Komponenten oder Marker an die Button-Entität.
///
/// ## Beispiel
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
///         // Standard-Button ohne Aktion
///         ButtonBuilder::new()
///             .text("Standard")
///             .spawn(parent, &theme, &font);
///
///         // Button mit einer benutzerdefinierten Aktion
///         ButtonBuilder::<MyGameAction>::new_for_action()
///             .text("Spiel starten")
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
///                 MyGameAction::StartGame => info!("StartGame gedrückt"),
///                 MyGameAction::OpenSettings => info!("OpenSettings gedrückt"),
///             }
///         }
///     }
/// }
/// ```
///
/// Siehe auch [`NoAction`] und [`ButtonClickedEvent`].
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
    /// Erstellt einen `ButtonBuilder` mit Standardwerten und dem Aktionstyp `NoAction`.
    /// `action` ist initial `None`, d.h. es wird standardmäßig keine `NoAction`-Komponente
    /// zur Entität hinzugefügt, es sei denn, `.action(NoAction)` wird explizit aufgerufen.
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(), // TODO: Default-Implementierung für UiColorPalette anpassen, dass eine wirklich Farbe zurückgegeben wird
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
// impl ButtonBuilder<NoAction> {
//     /// Erstellt einen neuen `ButtonBuilder` für einen Button ohne spezifische anwendungsdefinierte Aktion.
//     ///
//     /// Der generische Aktionstyp `A` ist hier [`NoAction`].
//     /// Um eine Aktion hinzuzufügen (selbst eine `NoAction`), verwenden Sie die `.action()` Methode.
//     /// Ohne expliziten `.action(NoAction)`-Aufruf wird dem gesendeten `ButtonClickedEvent<NoAction>`
//     /// im Feld `action_id` ein `None` übergeben.
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

impl<A: Component + Clone + Send + Sync + 'static> ButtonBuilder<A> {
    pub fn new() -> Self {
        ButtonBuilder {
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(), // TODO: Default-Implementierung für UiColorPalette anpassen, dass eine wirklich Farbe zurückgegeben wird
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
    /// Erstellt einen neuen Builder, der für eine spezifische Aktion `A` vorgesehen ist.
    ///
    /// Dies ist nützlich, um die Typsignatur für den Builder explizit festzulegen,
    /// wenn `A` nicht `NoAction` ist, oder wenn `A` nicht `Default` implementiert.
    /// Die eigentliche Aktionsinstanz wird mit der `.action()` Methode gesetzt.
    pub fn new_for_action() -> Self {
        ButtonBuilder {
            variant: ButtonVariant::Solid,
            color_palette: UiColorPalette::default(), // TODO: Default-Implementierung für UiColorPalette anpassen, dass eine wirklich Farbe zurückgegeben wird
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

    pub fn color(mut self, color: UiColorPalette) -> Self {
        self.color_palette = color;
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

    /// Fügt einen Text als Inhalt des Buttons hinzu.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.children_defs.push(ButtonChild::Text(text.into()));
        self
    }

    /// Fügt eine benutzerdefinierte Closure als Child hinzu.
    pub fn child(
        mut self,
        f: impl FnOnce(&mut ChildSpawnerCommands) + Send + Sync + 'static,
    ) -> Self {
        self.children_defs.push(ButtonChild::Custom(Box::new(f)));
        self
    }

    /// Spawnt direkt einen VerticalStackBuilder im Button.
    pub fn vertical_stack(mut self, vsb: VerticalStackBuilder) -> Self {
        self.children_defs
            .push(ButtonChild::Custom(Box::new(move |parent| {
                vsb.spawn(parent);
            })));
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
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font_family: &Handle<Font>,
    ) -> EntityCommands<'s> {
        let mut color_palette = self.color_palette.clone();

        if color_palette == UiColorPalette::default() {
            info!("Warning: Color palette is default. This may not be intended.");
            color_palette = theme.accent.clone();
        }
        let button_style = ButtonStyle::new(
            self.variant,
            self.size,
            &color_palette,
            if self.disabled {
                Interaction::None
            } else {
                Interaction::default()
            },
            theme,
        );

        let text_style = button_style.text_style.clone();
        let font_size = text_style.font_size;

        info!(
            "Button background color: {:?}",
            button_style.background_color
        );

        let mut cmd = parent.spawn((
            Button, // Bevy Komponente
            Node {
                ..button_style.node
            },
            button_style.background_color,
            button_style.border_color,
            button_style.border_radius,
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
        let children_defs = self.children_defs; // take ownership so we can move out of Box<dyn FnOnce>
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
        cmd
    }
}
