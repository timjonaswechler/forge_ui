use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_dialog_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    _assets: &AssetServer,
) {
    let mut dialog_section = create_variant_section(parent, "Dialog Components", theme, font);

    dialog_section.with_children(|vc| {
        let dialog_id = DialogId::new_unique();

        let _ = DialogTriggerBuilder::new(dialog_id)
            .text("Open Dialog")
            .variant(ButtonVariant::Solid)
            .spawn(vc, theme, font);

        let content = DialogContentBuilder::new()
            .header(DialogHeaderBuilder::new().title("Example Dialog"))
            .body(DialogBodyBuilder::new().add_content(|p, theme, font| {
                p.spawn((
                    Text::new("This is a dialog body."),
                    TextFont {
                        font: font.clone(),
                        font_size: theme.font.size.base,
                        ..default()
                    },
                    TextColor(theme.color.slate.step12),
                ));
            }))
            .footer(
                DialogFooterBuilder::new().add_custom_content(move |p, theme, font| {
                    p.spawn(
                        ButtonBuilder::<DialogAction>::new("Close Button")
                            .text("Close")
                            .variant(ButtonVariant::Soft)
                            .action(DialogAction::Close(dialog_id))
                            .build(theme, font),
                    );
                }),
            );

        let mut cmds = vc.commands_mut();
        let _ = DialogBuilder::new(dialog_id)
            .content(content)
            .spawn(&mut cmds, theme, font, None);
    });

    // Optional: weitere Trigger-Beispiele
    let _triggers_section = create_variant_section(parent, "Dialog Triggers", theme, font);
}
