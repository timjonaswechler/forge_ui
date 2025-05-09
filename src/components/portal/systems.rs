// In forge_ui/src/components/portal/systems.rs
use super::components::ForgeUiPortalRoot;
use bevy::prelude::*; // Pfad anpassen

pub(crate) fn setup_global_portal_root(mut commands: Commands) {
    let root_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute, // Nimmt keinen Platz im normalen Layout
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0), // Bedeckt den ganzen Bildschirm (optional, je nach Bedarf)
                height: Val::Percent(100.0),
                // Wichtig: display: Display::None, damit es nicht rendert, wenn leer,
                // oder Display::Flex/Grid, wenn es selbst ein Layout für portierte Elemente bieten soll.
                // Für reine "Anhängepunkte" ist keine spezifische Darstellung nötig.
                ..default()
            },
            // background_color: Color::NONE.into(), // Unsichtbar
            // z_index: GlobalZIndex::new(1000), // Sehr hoher Z-Index, falls nötig
            Name::new("GlobalUIPortalRoot"),
        ))
        .id();
    commands.insert_resource(ForgeUiPortalRoot(root_entity));
    info!("GlobalUIPortalRoot {:?} wurde initialisiert.", root_entity);
}
