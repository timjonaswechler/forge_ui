use bevy::prelude::*;

use super::components::AvatarMarker;
use super::enums::AvatarSize;
use crate::theme::UiTheme;

/// Fluent builder for an avatar element.
///
/// Avatars display a square or circular image. When no image is
/// provided, optional text can be shown as fallback.
pub struct AvatarBuilder {
    image: Option<Handle<Image>>,
    initials: Option<String>,
    size: AvatarSize,
}

impl AvatarBuilder {
    /// Creates a new avatar builder without image.
    pub fn new() -> Self {
        Self { image: None, initials: None, size: AvatarSize::default() }
    }

    /// Sets the image handle used for the avatar.
    pub fn image(mut self, handle: Handle<Image>) -> Self {
        self.image = Some(handle);
        self
    }

    /// Sets fallback initials text shown when no image is available.
    pub fn initials(mut self, text: impl Into<String>) -> Self {
        self.initials = Some(text.into());
        self
    }

    /// Sets the avatar size variant.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Spawns the avatar under the given parent.
    #[must_use]
    pub fn spawn<'w, 's>(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Entity {
        let size_px = match self.size {
            AvatarSize::Small => theme.font.size.sm,
            AvatarSize::Medium => theme.font.size.base * 2.0,
            AvatarSize::Large => theme.font.size.xl * 2.0,
        };

        let mut cmd = parent.spawn((
            AvatarMarker,
            Node {
                width: Val::Px(size_px),
                height: Val::Px(size_px),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme.color.gray.step04),
            BorderRadius::all(Val::Px(size_px / 2.0)),
            Name::new("Avatar"),
        ));

        cmd.with_children(|parent| {
            if let Some(img) = self.image {
                parent.spawn(ImageNode::new(img));
            } else if let Some(text) = self.initials.clone() {
                parent.spawn((
                    Text::new(text),
                    TextFont {
                        font: font.clone(),
                        font_size: size_px * 0.5,
                        ..default()
                    },
                    TextColor(theme.color.gray.step12),
                ));
            }
        });

        cmd.id()
    }
}
