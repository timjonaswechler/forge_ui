use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_label_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    let mut variants_section = create_variant_section(parent, "Label Variants", theme, font);

    variants_section.with_children(|vc| {
        // Standard Label
        let _ = LabelBuilder::new("Standard Label").spawn(vc, theme, font);

        // Colored Label
        let _ = LabelBuilder::new("Colored Label")
            .color(theme.color.crimson.step10)
            .spawn(vc, theme, font);

        // Different Font Sizes
        let _ = LabelBuilder::new("Small Font")
            .font_size(theme.font.size.sm)
            .spawn(vc, theme, font);

        let _ = LabelBuilder::new("Large Font")
            .font_size(theme.font.size.xl)
            .spawn(vc, theme, font);

        // Text Alignment
        let _ = LabelBuilder::new("Left Aligned")
            .align(JustifyText::Left)
            .spawn(vc, theme, font);

        let _ = LabelBuilder::new("Right Aligned")
            .align(JustifyText::Right)
            .spawn(vc, theme, font);

        let _ = LabelBuilder::new("Center Aligned")
            .align(JustifyText::Center)
            .spawn(vc, theme, font);

        // Mit Margin
        let _ = LabelBuilder::new("Label with Margin")
            .margin(UiRect::all(Val::Px(10.0)))
            .spawn(vc, theme, font);
    });
}
