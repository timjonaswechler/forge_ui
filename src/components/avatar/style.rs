use super::enums::AvatarSize;
use crate::theme::UiTheme;

/// Calculates node size for the given avatar size.
pub fn avatar_size_px(theme: &UiTheme, size: AvatarSize) -> f32 {
    match size {
        AvatarSize::Small => theme.font.size.sm,
        AvatarSize::Medium => theme.font.size.base * 2.0,
        AvatarSize::Large => theme.font.size.xl * 2.0,
    }
}
