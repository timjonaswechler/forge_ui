// src/components/dialog/components.rs
use bevy::platform::collections::HashSet; // Korrekter Import für HashSet
use bevy::prelude::*;
use uuid::Uuid; // Import Uuid

#[derive(Resource, Default, Debug)]
pub struct ActiveDialogs {
    pub modals: HashSet<Entity>, // Entity-IDs der aktuell aktiven modalen Dialoge
}

// --- Komponenten ---
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum DialogAction {
    Open(DialogId),
    Close(DialogId),
}
impl Default for DialogAction {
    fn default() -> Self {
        DialogAction::Close(DialogId::default())
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct DialogRootMarker;

#[derive(Component, Debug, Clone)]
pub struct DialogConfig {
    pub id: DialogId,
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
        Self::new_unique()
    }
}

#[derive(Component, Debug, Default)]
pub struct DialogState {
    pub open: bool,
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogOverlay;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DialogContent;

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
}

impl DialogContentBundle {
    // Konstruktor, um es einfacher zu machen
    #[allow(dead_code)]
    pub fn new(
        node: Node,
        background_color: BackgroundColor,
        border_radius: BorderRadius, // Nimm BorderRadius als Parameter
    ) -> Self {
        Self {
            marker: DialogContent,
            node: node.clone(),
            background_color: background_color,
            border_radius, // Setze es hier
        }
    }
}

// Der alte `DialogRoot` wird aufgeteilt in:
// - `DialogRootMarker` (nur ein Marker)
// - `DialogConfig` (enthält die Konfigurationsdaten wie id, initially_open, modal)
// - Die `NodeBundle` Komponenten kommen direkt in das Bundle, das beim Spawnen verwendet wird.
