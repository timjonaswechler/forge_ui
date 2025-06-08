use super::super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

pub fn show_dialog_variants(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
    assets: &AssetServer,
) {
    let dialog_section = create_variant_section(parent, "Dialog Components", theme, font);

    // parent.entity(dialog_section).with_children(|vc| {
    //     // Dialog ID für Demos
    //     let dialog_id = DialogId::new_unique();

    //     // Dialog Trigger Button
    //     DialogTriggerBuilder::new(dialog_id)
    //         .text("Open Dialog")
    //         .variant(ButtonVariant::Primary)
    //         .spawn(vc, theme, font);

    //     // Dialog selbst definieren
    //     let dialog_content = DialogContentBuilder::new()
    //         .header(
    //             DialogHeaderBuilder::new()
    //                 .title("Example Dialog")
    //                 .subtitle("This is a sample dialog component"),
    //         )
    //         .body(DialogBodyBuilder::new())
    //         .footer(DialogFooterBuilder::new().justify_content(JustifyContent::End));

    //     DialogBuilder::new(dialog_id)
    //         .content(dialog_content)
    //         .width(Val::Px(400.0))
    //         .height(Val::Auto)
    //         .overlay_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
    //         .spawn(vc, theme, font);
    // });

    // Verschiedene Dialog-Trigger-Varianten
    let triggers_section = create_variant_section(parent, "Dialog Triggers", theme, font);

    // parent.entity(triggers_section).with_children(|vc| {
    //     let dialog_id_1 = DialogId::new_unique();
    //     let dialog_id_2 = DialogId::new_unique();
    //     let dialog_id_3 = DialogId::new_unique();

    //     // Verschiedene Trigger-Varianten
    //     DialogTriggerBuilder::new(dialog_id_1)
    //         .text("Primary Trigger")
    //         .variant(ButtonVariant::Primary)
    //         .spawn(vc, theme, font);

    //     DialogTriggerBuilder::new(dialog_id_2)
    //         .text("Secondary Trigger")
    //         .variant(ButtonVariant::Secondary)
    //         .spawn(vc, theme, font);

    //     DialogTriggerBuilder::new(dialog_id_3)
    //         .text("Small Trigger")
    //         .variant(ButtonVariant::Ghost)
    //         .size(ButtonSize::Small)
    //         .spawn(vc, theme, font);

    //     // Die entsprechenden Dialoge spawnen (minimalistisch)
    //     for dialog_id in [dialog_id_1, dialog_id_2, dialog_id_3] {
    //         let content =
    //             DialogContentBuilder::new().header(DialogHeaderBuilder::new().title("Dialog"));

    //         DialogBuilder::new(dialog_id)
    //             .content(content)
    //             .width(Val::Px(300.0))
    //             .spawn(vc, theme, font);
    //     }
    // });
}
