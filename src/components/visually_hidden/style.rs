use bevy::prelude::*;

/// Style bundle hiding content from the screen while keeping it accessible.
#[derive(Bundle, Clone, Debug)]
pub struct VisuallyHiddenStyle {
    pub node: Node,
    pub visibility: Visibility,
}

impl Default for VisuallyHiddenStyle {
    fn default() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Absolute,
                width: Val::Px(1.0),
                height: Val::Px(1.0),
                left: Val::Px(-1.0),
                top: Val::Px(-1.0),
                overflow: Overflow::clip(),
                ..default()
            },
            visibility: Visibility::Hidden,
        }
    }
}
