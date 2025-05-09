// src/components/dialog/components.rs
use bevy::platform::collections::HashSet; // Korrekter Import für HashSet
use bevy::prelude::*;
use uuid::Uuid; // Import Uuid

// --- Ressourcen, die spezifisch für das Dialog-System sind ---
// DialogPortalContainer wird durch portal::ForgeUiPortalRoot ersetzt.
// Wenn du eine spezifische Root NUR für Dialoge willst, die unabhängig
// von der allgemeinen ForgeUiPortalRoot ist, könntest du sie hier definieren
// und ihr eigenes Setup-System haben. Für jetzt gehen wir davon aus,
// dass Dialoge den allgemeinen ForgeUiPortalRoot nutzen können.

#[derive(Resource, Default, Debug)]
pub struct ActiveDialogs {
    pub modals: HashSet<Entity>, // Entity-IDs der aktuell aktiven modalen Dialoge
}

// --- Komponenten ---

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct DialogRootMarker; // Einfacher Marker, ersetzt erstmal die alte DialogRoot struct-Logik teilweise

#[derive(Component, Debug, Clone)]
pub struct DialogConfig {
    pub id: DialogId,
    pub initially_open: bool,
    pub modal: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DialogId(pub Uuid);

impl DialogId {
    pub fn new_unique() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DialogId {
    fn default() -> Self {
        Self::new_unique() // Ein Default Dialog hat immer eine neue, einzigartige ID
    }
}

#[derive(Component, Debug, Default)]
pub struct DialogState {
    pub open: bool,
    // default_open wird eher für die initiale Konfiguration im Builder/DialogConfig genutzt
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogOverlay;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogContent; // Marker für den Haupt-Content-Node des Dialogs

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogCloseTrigger; // Marker für UI-Elemente, die den Dialog schließen

// Potentiell für Animationen oder Zustandsmanagement beim Ausblenden
#[derive(Component, Debug, Clone, Copy)]
pub struct KeepMounted(pub bool);

#[derive(Bundle, Clone, Default)]
pub struct DialogBundle {
    pub marker: DialogRootMarker,
    pub overlay: DialogOverlay,
    pub content_bundle: DialogContentBundle,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub transform: Transform,
    pub border_radius: BorderRadius,
    pub id: DialogId,
}

#[derive(Bundle, Clone, Default)]
pub struct DialogContentBundle {
    pub marker: DialogContent,
    // Verwende NodeBundle für alle UI-Basis-Komponenten
    pub node: Node,
    pub border_radius: BorderRadius,
    pub background_color: BackgroundColor,
    pub transform: Transform,
}

impl DialogContentBundle {
    // Konstruktor, um es einfacher zu machen
    pub fn new(
        node: Node,
        background_color: BackgroundColor,
        transform: Transform,
        border_radius: BorderRadius, // Nimm BorderRadius als Parameter
    ) -> Self {
        Self {
            marker: DialogContent,
            node: node.clone(),
            background_color: background_color,
            transform: transform,
            border_radius, // Setze es hier
        }
    }
}

// Der alte `DialogRoot` wird aufgeteilt in:
// - `DialogRootMarker` (nur ein Marker)
// - `DialogConfig` (enthält die Konfigurationsdaten wie id, initially_open, modal)
// - Die `NodeBundle` Komponenten kommen direkt in das Bundle, das beim Spawnen verwendet wird.
