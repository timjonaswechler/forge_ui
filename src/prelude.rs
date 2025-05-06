// src/prelude.rs

//! Prelude provides the most commonly used types and builders for easy import.

// Komponenten
pub use crate::components::{
    Button, ButtonSize, ButtonVariant, Card, CardBuilder, CheckboxBuilder, DialogBuilder,
    LabelBuilder, TabsBuilder,
};

// Layout
pub use crate::layout::{
    HorizontalStack, HorizontalStackBuilder, VerticalStack, VerticalStackBuilder,
};

// Theme
pub use crate::theme::UiTheme;
