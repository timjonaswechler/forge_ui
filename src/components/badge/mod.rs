// src/components/badge/mod.rs
mod builder;
mod components;
mod enums;
mod utils;
mod style;

// Falls du trotzdem den Typ selbst brauchst:
pub use builder::BadgeBuilder;
pub use components::BadgeMarker;
pub use enums::BadgeVariant;
pub use style::BadgeStyle;
