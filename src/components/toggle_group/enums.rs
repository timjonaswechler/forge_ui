/// Selection mode of the toggle group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupType {
    /// Only one toggle can be active at a time
    Single,
    /// Multiple toggles can be active simultaneously
    Multiple,
}

impl Default for ToggleGroupType {
    fn default() -> Self {
        Self::Single
    }
}

/// Orientation of the toggle group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    /// Horizontal layout
    Horizontal,
    /// Vertical layout
    Vertical,
}

impl Default for ToggleGroupOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Visual variants for toggle groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupVariant {
    /// Passende Standard-Farbe aus dem Theme
    #[default]
    Default,
    /// Haupt-Akzentfarbe
    Primary,
    /// Graustufen etc.
    Secondary,
}

/// Size variations for toggle groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupSize {
    Small,
    #[default]
    Medium,
    Large,
}
