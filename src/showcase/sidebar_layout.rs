//! Permanentes Sidebar-Layout + dynamischer Content-Container
//! Wird vom Showcase-Plugin eingebunden.

use super::components::*;
use super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

/// Erzeugt beim Start die Root-Nodes: Sidebar (links) + Content-Container (rechts).
pub fn setup_ui(mut commands: Commands, theme: Res<UiTheme>, font: Res<FontAssets>) {
    let root = UiRoot::spawn(&mut commands, &theme);
    commands
        .entity(root)
        .insert((ShowcaseMarker, Name::new("Showcase UI")));

    // Sidebar als Kind hinzufügen
    commands.entity(root).with_children(|parent| {
        let _sidebar = build_sidebar(parent, &theme, &font.default);
    });

    // Content-Container als Kind der Root hinzufügen (statt als separates Root-Element)
    commands.entity(root).with_children(|parent| {
        let _content_container = build_content_container(parent, &theme);
    });
}

/// Linke Spalte: feste Breite + Menü-Buttons
pub fn build_sidebar(
    parent: &mut ChildSpawnerCommands,
    theme: &UiTheme,
    font: &Handle<Font>,
) -> Entity {
    VerticalStackBuilder::new("Sidebar")
        .position_type(PositionType::Absolute)
        .top(Val::Px(0.0))
        .left(Val::Px(0.0))
        .width(Val::Px(240.0))
        .height(Val::Percent(100.0))
        .background(theme.color.tomato.step06)
        .gap(Val::Px(2.0))
        .padding(Val::Px(12.0))
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Checkbox")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Checkbox))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Checkbox Cards")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::CheckboxCards))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Radio Cards")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::RadioCards))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Checkbox-Group")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::CheckboxGroup))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Radio-Group")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::RadioGroup))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Toggle")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Toggle))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Toggle-Group")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::ToggleGroup))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Button")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Button))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Label")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Label))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Dialog")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Dialog))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Portal")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Portal))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Badge")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Badge))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Accordion")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Accordion))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Switch")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Switch))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Avatar")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Avatar))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Card")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Card))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Box")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Box))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Blockquote")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Blockquote))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Callout")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Callout))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Code")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Code))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Collapsible")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Collapsible))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Collection")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Collection))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Container")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Container))
                .spawn(parent, theme, font),
        )
        .add_entity(
            ButtonBuilder::new_for_action()
                .text("Context")
                .action(ShowcaseAction::ShowElement(ShowcaseElement::Context))
                .spawn(parent, theme, font),
        )
        .spawn(parent)
        .id()
}

/// Rechte Spalte: leerer Platzhalter für das jeweils gewählte Demo-Element
pub fn build_content_container(parent: &mut ChildSpawnerCommands, theme: &UiTheme) -> Entity {
    let container = UiContainer::spawn_as_child(parent, theme);

    parent.commands().entity(container).insert((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(240.0),
            ..default()
        },
        ContentContainer,
        BackgroundColor(theme.color.gray.step03.into()),
        Name::new("Content Container"),
    ));
    container
}

/// Reagiert auf Button-Clicks und füllt den Container neu
pub fn handle_select_element(
    mut commands: Commands,
    mut events: EventReader<ButtonClickedEvent<ShowcaseAction>>,
    container_q: Query<Entity, With<ContentContainer>>,
    theme: Res<UiTheme>,
    font: Res<FontAssets>,
    icons: Res<IconAssets>,
    assets: Res<AssetServer>,
    global_portal_root: Res<ForgeUiPortalRoot>,
) {
    let container = match container_q.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    for ev in events.read() {
        if let Some(ShowcaseAction::ShowElement(elem)) = &ev.action_id {
            let cross_icon_handle = icons
                .0
                .get("cross-1")
                .expect("missing 'cross-1' icon")
                .clone();
            let check_icon_handle = icons.0.get("check").expect("missing 'check' icon").clone();

            // Aktuellen Inhalt löschen
            commands.entity(container).despawn_related::<Children>();

            let elem = elem.clone();

            let theme = theme.clone();
            let assets = assets.clone();
            let check_icon_handle = check_icon_handle.clone();
            let cross_icon_handle = cross_icon_handle.clone();

            commands.entity(container).with_children(|parent| {
                // Title für die Showcase
                parent.spawn((
                    Text::new(format!("{:?} Showcase", elem)),
                    TextFont {
                        font: font.default.clone(),
                        font_size: theme.font.size.xl,
                        ..default()
                    },
                    TextColor(theme.color.slate.step12),
                    Name::new(format!("{:?} Title", elem)),
                ));

                // Container für alle Varianten
                parent
                    .spawn((
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            margin: UiRect::vertical(Val::Px(20.0)),
                            padding: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        Name::new("Variants Container"),
                    ))
                    .with_children(|vc| match elem {
                        ShowcaseElement::Button => show_button_variants(vc, &theme, &font.default),
                        ShowcaseElement::Checkbox => show_checkbox_variants(vc, &theme, &icons),
                        ShowcaseElement::CheckboxCards => {
                            show_checkbox_card_variants(vc, &theme, &font.default, &icons)
                        }
                        ShowcaseElement::RadioCards => {
                            show_radio_card_variants(vc, &theme, &font.default)
                        }
                        ShowcaseElement::CheckboxGroup => {
                            show_checkbox_group_variants(vc, &theme, &font.default, &icons)
                        }
                        ShowcaseElement::Switch => show_switch_variants(vc, &theme, &icons),
                        ShowcaseElement::RadioGroup => {
                            show_radio_group_variants(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Toggle => show_toggle_variants(vc, &theme, &font.default),
                        ShowcaseElement::ToggleGroup => show_toggle_group_variants(
                            vc,
                            &theme,
                            &font.default,
                            check_icon_handle,
                            cross_icon_handle,
                        ),
                        ShowcaseElement::Accordion => {
                            show_accordion_variants(vc, &theme, &font.default);
                        }
                        ShowcaseElement::Badge => {
                            show_badge_variants(vc, &theme, &font.default, &icons)
                        }
                        ShowcaseElement::Dialog => {
                            show_dialog_variants(vc, &theme, &font.default, &assets)
                        }
                        ShowcaseElement::Portal => show_portal_example(
                            vc,
                            &theme,
                            &font.default,
                            Res::clone(&global_portal_root),
                        ),
                        ShowcaseElement::Label => show_label_variants(vc, &theme, &font.default),
                        ShowcaseElement::Avatar => {
                            show_avatar_examples(vc, &theme, &font.default, &icons)
                        }
                        ShowcaseElement::Card => show_card_example(vc, &theme, &font.default),
                        ShowcaseElement::Box => show_box_example(vc, &theme, &font.default),
                        ShowcaseElement::Blockquote => {
                            show_blockquote_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Callout => show_callout_example(vc, &theme, &font.default),
                        ShowcaseElement::Code => show_code_example(vc, &theme, &font.default),
                        ShowcaseElement::Collapsible => {
                            show_collapsible_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Collection => {
                            show_collection_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Container => {
                            show_container_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Context => show_context_example(vc, &theme, &font.default),
                    });
            });
        }
    }
}
