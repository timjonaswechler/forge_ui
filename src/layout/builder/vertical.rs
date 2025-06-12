use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use super::base::BaseStackBuilder;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VerticalStack;

pub struct VerticalStackBuilder {
    base: BaseStackBuilder,
    name: Option<String>,
    children: Vec<Box<dyn FnOnce(&mut ChildSpawnerCommands) + Send + Sync>>,
    additional_children: Vec<Entity>,
}
impl Default for VerticalStackBuilder {
    fn default() -> Self {
        let mut base = BaseStackBuilder::default();
        // Sinnvolle Defaults für Vertical Stack überschreiben
        base.node.align_items = AlignItems::Center; // Zentriert horizontal, oft gewünscht
        Self {
            name: None,
            base,
            children: Vec::new(),
            additional_children: Vec::new(),
        }
    }
}

impl VerticalStackBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..Self::default()
        }
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.base.node.position_type = position_type;
        self
    }
    pub fn top(mut self, top: Val) -> Self {
        self.base.node.top = top;
        self
    }
    pub fn left(mut self, left: Val) -> Self {
        self.base.node.left = left;
        self
    }
    pub fn right(mut self, right: Val) -> Self {
        self.base.node.right = right;
        self
    }
    pub fn bottom(mut self, bottom: Val) -> Self {
        self.base.node.bottom = bottom;
        self
    }
    /// Setzt die Ausrichtung der Elemente entlang der Hauptachse (vertikal).
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.base.node.justify_content = justify;
        self
    }
    /// Setzt die Ausrichtung der Elemente entlang der Querachse (horizontal).
    pub fn align(mut self, align: AlignItems) -> Self {
        self.base.node.align_items = align;
        self
    }
    /// Setzt den vertikalen Abstand zwischen den Elementen.
    pub fn gap(mut self, gap: Val) -> Self {
        self.base.gap = gap;
        self
    }
    /// Setzt das Padding für alle Seiten.
    pub fn padding(mut self, padding: Val) -> Self {
        self.base.node.padding = UiRect::all(padding);
        self
    }
    /// Setzt das Padding spezifisch.
    pub fn padding_rect(mut self, padding: UiRect) -> Self {
        self.base.node.padding = padding;
        self
    }
    /// Setzt das Margin für alle Seiten.
    pub fn margin(mut self, margin: Val) -> Self {
        self.base.node.margin = UiRect::all(margin);
        self
    }
    /// Setzt das Margin spezifisch.
    pub fn margin_rect(mut self, margin: UiRect) -> Self {
        self.base.node.margin = margin;
        self
    }
    /// Setzt eine feste Breite.
    pub fn width(mut self, width: Val) -> Self {
        self.base.node.width = width;
        self
    }
    /// Setzt eine feste Höhe.
    pub fn height(mut self, height: Val) -> Self {
        self.base.node.height = height;
        self
    }

    /// Sets the overflow behaviour for the stack container.
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.base.node.overflow = overflow;
        self
    }
    /// Setzt eine Hintergrundfarbe.
    pub fn background(mut self, color: Color) -> Self {
        self.base.background = BackgroundColor(color);
        self
    }

    /// Fügt ein Kind-Element hinzu, das durch eine Closure gespawnt wird.
    /// Die Closure erhält einen `&mut ChildBuilder`.
    pub fn add_fn(
        mut self,
        child_builder: impl FnOnce(&mut ChildSpawnerCommands) + Send + Sync + 'static,
    ) -> Self {
        self.children.push(Box::new(child_builder));
        self
    }
    pub fn add_entity(mut self, entity: EntityCommands) -> Self {
        self.additional_children.push(entity.id());
        self
    }

    /// Spawnt den vertikalen Stack und seine Kinder.
    #[must_use]
    pub fn spawn<'w, 'a>(self, parent: &'a mut ChildSpawnerCommands<'w>) -> EntityCommands<'a> {
        let style = Node {
            position_type: self.base.node.position_type,
            top: self.base.node.top,
            left: self.base.node.left,
            right: self.base.node.right,
            bottom: self.base.node.bottom,

            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: self.base.node.justify_content,
            align_items: self.base.node.align_items,
            row_gap: self.base.gap,
            padding: self.base.node.padding,
            margin: self.base.node.margin,
            width: self.base.node.width,
            height: self.base.node.height,
            min_width: self.base.node.min_width,
            min_height: self.base.node.min_height,
            max_width: self.base.node.max_width,
            max_height: self.base.node.max_height,
            aspect_ratio: self.base.node.aspect_ratio, // Bevy erwartet Option<Val::aspect_ratio(f32)>
            overflow: self.base.node.overflow,
            ..default()
        };

        let mut cmd = parent.spawn((
            Name::new(self.name.unwrap_or_else(|| "VerticalStack".to_string())),
            VerticalStack, // Marker
            Node { ..style },
            self.base.background,
        ));
        // Kinder hinzufügen
        cmd.with_children(|builder| {
            for child_fn in self.children {
                (child_fn)(builder);
            }
        });
        // Füge die zusätzlich gespeicherten Kinder hinzu
        for child in self.additional_children {
            cmd.add_child(child);
        }

        cmd
    }
}
