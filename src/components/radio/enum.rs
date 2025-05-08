// src/components/radio/enums.rs

/// Variants of the radio appearance
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RadioVariant {
    Primary,
    Secondary,
    // add more as needed
}

/// Sizes for the radio (e.g., diameter of the circle)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RadioSize {
    Small,
    Medium,
    Large,
}
