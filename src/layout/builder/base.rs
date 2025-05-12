use bevy::prelude::*;

#[derive(Component, Default)]
pub(crate) struct BaseStackBuilder {
    pub(crate) node: Node,
    pub(crate) gap: Val,
    pub(crate) background: BackgroundColor,
}
