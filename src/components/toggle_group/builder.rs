use super::*;
use crate::components::helper::NoAction;
use crate::components::toggle::{ToggleBuilder, ToggleSize, ToggleVariant};
use bevy::prelude::*;
use std::collections::HashSet;

/// Builder for the ToggleGroup component
pub struct ToggleGroupBuilder<A: Component + Clone + Send + Sync + 'static = NoAction> {
    group_type: ToggleGroupType,
    orientation: ToggleGroupOrientation,
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
    disabled: bool,
    default_values: HashSet<String>,
    loop_navigation: bool,
    roving_focus: bool,
    action: Option<A>,
    items: Vec<ToggleGroupItemBuilder>,
}

/// Item configuration for the ToggleGroup
pub struct ToggleGroupItemBuilder {
    value: String,
    icon: Option<Handle<Image>>,
    aria_label: Option<String>,
    disabled: bool,
}

impl<A: Component + Clone + Send + Sync + 'static> ToggleGroupBuilder<A> {
    /// Constructor with explicit action type
    pub fn new_with_action_type() -> Self {
        Self {
            group_type: ToggleGroupType::default(),
            orientation: ToggleGroupOrientation::default(),
            variant: ToggleGroupVariant::default(),
            size: ToggleGroupSize::default(),
            disabled: false,
            default_values: HashSet::new(),
            loop_navigation: true,
            roving_focus: true,
            action: None,
            items: Vec::new(),
        }
    }

    /// Set the selection type (single or multiple)
    pub fn group_type(mut self, group_type: ToggleGroupType) -> Self {
        self.group_type = group_type;
        self
    }

    /// Set the orientation (horizontal or vertical)
    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size variant
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the group is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Add one default active value
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_values.insert(value.into());
        self
    }

    /// Set multiple default active values
    pub fn default_values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for value in values {
            self.default_values.insert(value.into());
        }
        self
    }

    /// Set whether keyboard navigation should loop
    pub fn loop_navigation(mut self, loop_nav: bool) -> Self {
        self.loop_navigation = loop_nav;
        self
    }

    /// Set whether to use roving tabindex focus pattern
    pub fn roving_focus(mut self, roving: bool) -> Self {
        self.roving_focus = roving;
        self
    }

    /// Set the action component
    pub fn action(mut self, action: A) -> Self {
        self.action = Some(action);
        self
    }

    /// Add a toggle item with a value
    pub fn add_item(mut self, value: impl Into<String>) -> Self {
        self.items.push(ToggleGroupItemBuilder {
            value: value.into(),
            icon: None,
            aria_label: None,
            disabled: false,
        });
        self
    }

    /// Add a toggle item with icon and value
    pub fn add_item_with_icon(mut self, value: impl Into<String>, icon: Handle<Image>) -> Self {
        self.items.push(ToggleGroupItemBuilder {
            value: value.into(),
            icon: Some(icon),
            aria_label: None,
            disabled: false,
        });
        self
    }

    /// Add a complete toggle item builder
    pub fn add_item_builder(mut self, builder: ToggleGroupItemBuilder) -> Self {
        self.items.push(builder);
        self
    }

    /// Spawns the toggle group as a child entity
    pub fn spawn_into<'w>(
        self,
        parent: &'w mut ChildSpawnerCommands<'w>,
        theme: &crate::theme::UiTheme,
    ) -> EntityCommands<'w> {
        let style_def = get_toggle_group_style_def(theme, self.variant, self.size);

        // Determine the appropriate toggle variant to match the group
        let toggle_variant = match self.variant {
            ToggleGroupVariant::Primary => ToggleVariant::Primary,
            ToggleGroupVariant::Secondary => ToggleVariant::Secondary,
            ToggleGroupVariant::Default => ToggleVariant::Default,
        };

        // Determine the appropriate toggle size to match the group
        let toggle_size = match self.size {
            ToggleGroupSize::Small => ToggleSize::Small,
            ToggleGroupSize::Medium => ToggleSize::Medium,
            ToggleGroupSize::Large => ToggleSize::Large,
        };

        // Main container node
        let mut cmd = parent.spawn((
            Node {
                display: Display::Flex,
                flex_direction: match self.orientation {
                    ToggleGroupOrientation::Horizontal => FlexDirection::Row,
                    ToggleGroupOrientation::Vertical => FlexDirection::Column,
                },
                column_gap: match self.orientation {
                    ToggleGroupOrientation::Horizontal => Val::Px(style_def.spacing),
                    ToggleGroupOrientation::Vertical => Val::Px(0.0),
                },
                row_gap: match self.orientation {
                    ToggleGroupOrientation::Horizontal => Val::Px(0.0),
                    ToggleGroupOrientation::Vertical => Val::Px(style_def.spacing),
                },
                padding: style_def.padding,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(style_def.border_width)),
                ..Default::default()
            },
            BorderColor(style_def.border_color),
            BackgroundColor(style_def.background_color),
            BorderRadius::all(Val::Px(style_def.border_radius)),
            ToggleGroupMarker,
            ToggleGroupState {
                group_type: self.group_type,
                orientation: self.orientation,
                disabled: self.disabled,
                variant: self.variant,
                size: self.size,
                active_values: self.default_values.clone(),
                loop_navigation: self.loop_navigation,
                roving_focus: self.roving_focus,
            },
            Name::new("ToggleGroup"),
        ));

        // Action component if provided
        if let Some(action) = self.action {
            cmd.insert(action);
        }

        // Add toggle items
        cmd.with_children(|parent| {
            for item in self.items {
                parent
                    .spawn((
                        Node {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ToggleGroupItemMarker,
                        ToggleGroupItemState {
                            value: item.value.clone(),
                            pressed: self.default_values.contains(&item.value),
                            disabled: item.disabled,
                        },
                        Name::new(format!("ToggleGroupItem-{}", item.value)),
                    ))
                    .with_children(|parent| {
                        // Create the actual toggle button inside each item
                        let _ = ToggleBuilder::<A>::new()
                            .pressed(self.default_values.contains(&item.value))
                            .disabled(self.disabled || item.disabled)
                            .variant(toggle_variant)
                            .size(toggle_size)
                            .icon(item.icon.unwrap_or_else(|| {
                                // Create an empty/transparent image handle if none provided
                                // This is a placeholder - you'd want to have a proper empty handle
                                Handle::default()
                            }))
                            .spawn_into(parent, theme);
                    });
            }
        });

        cmd
    }
}

impl ToggleGroupBuilder<NoAction> {
    /// Shortcut for `ToggleGroupBuilder<NoAction>`
    pub fn new() -> Self {
        Self::new_with_action_type()
    }
}

impl ToggleGroupItemBuilder {
    /// Create a new toggle group item builder
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            icon: None,
            aria_label: None,
            disabled: false,
        }
    }

    /// Set the icon for this item
    pub fn icon(mut self, icon: Handle<Image>) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set whether this specific item is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
