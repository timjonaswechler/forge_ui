//! Permanentes Sidebar-Layout + dynamischer Content-Container
//! Wird vom Showcase-Plugin eingebunden.

use super::components::*;
use super::helpers::*;
use crate::prelude::*;
use bevy::prelude::*;

/// Erzeugt beim Start die Root-Nodes: Sidebar (links) + Content-Container (rechts).
pub fn setup_ui(mut commands: Commands, theme: Res<UiTheme>, font: Res<FontAssets>) {
    let root = commands.spawn(UiRoot::bundle(&theme)).id();
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
        .overflow(Overflow::scroll_y())
        .background(theme.color.tomato.step06)
        .gap(Val::Px(2.0))
        .padding(Val::Px(12.0))
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Checkbox")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Checkbox))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Checkbox Cards")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::CheckboxCards))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Radio Cards")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::RadioCards))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Checkbox-Group")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::CheckboxGroup))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Radio-Group")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::RadioGroup))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Toggle")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Toggle))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Toggle-Group")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::ToggleGroup))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Button")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Button))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Label")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Label))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Dialog")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Dialog))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Alert Dialog")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::AlertDialog))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Aspect Ratio")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::AspectRatio))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Portal")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Portal))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Button")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseButton))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Card")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseCard))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Checkbox")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseCheckbox))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Dialog")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseDialog))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Menu")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseMenu))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Radio")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseRadio))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Base Tab List")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::BaseTabList))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Slot")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Slot))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Direction Provider")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::DirectionProvider))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Badge")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Badge))
        //         .spawn(parent, theme, font),
        // )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("accordion")
                    .text("Accordion")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Accordion))
                    .build(theme, font),
            ),
        )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Switch")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Switch))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Avatar")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Avatar))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Card")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Card))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Box")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Box))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Blockquote")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Blockquote))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Callout")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Callout))
        //         .spawn(parent, theme, font),
        // )
        // .add_entity(
        //     ButtonBuilder::new_for_action()
        //         .text("Code")
        //         .action(ShowcaseAction::ShowElement(ShowcaseElement::Code))
        //         .spawn(parent, theme, font),
        // )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("collapsible")
                    .text("Collapsible")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Collapsible))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("collection")
                    .text("Collection")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Collection))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("container")
                    .text("Container")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Container))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("context")
                    .text("Context")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Context))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("visually_hidden")
                    .text("Visually Hidden")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::VisuallyHidden))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("accessible_icon")
                    .text("Accessible Icon")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::AccessibleIcon))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("hover_card")
                    .text("Hover Card")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::HoverCard))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("menubar")
                    .text("Menubar")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Menubar))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("navigation_menu")
                    .text("Navigation Menu")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::NavigationMenu))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("popover")
                    .text("Popover")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Popover))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("progress")
                    .text("Progress")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Progress))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("scroll_area")
                    .text("Scroll Area")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::ScrollArea))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("select")
                    .text("Select")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Select))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("separator")
                    .text("Separator")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Separator))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("slider")
                    .text("Slider")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Slider))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("tabs")
                    .text("Tabs")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Tabs))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("toast")
                    .text("Toast")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Toast))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("toolbar")
                    .text("Toolbar")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Toolbar))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("tooltip")
                    .text("Tooltip")
                    .action(ShowcaseAction::ShowElement(ShowcaseElement::Tooltip))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("otp_field")
                    .text("OTP Field")
                    .action(ShowcaseAction::ShowElement(
                        ShowcaseElement::OneTimePasswordField,
                    ))
                    .build(theme, font),
            ),
        )
        .add_entity(
            parent.spawn(
                ButtonBuilder::new("password_toggle")
                    .text("Password Toggle")
                    .action(ShowcaseAction::ShowElement(
                        ShowcaseElement::PasswordToggleField,
                    ))
                    .build(theme, font),
            ),
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
                        ShowcaseElement::BaseCard => {
                            show_base_card_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::BaseButton => {
                            show_base_button_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::BaseCheckbox => {
                            show_base_checkbox_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::BaseDialog => {
                            show_base_dialog_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::BaseMenu => {
                            show_base_menu_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::BaseRadio => {
                            show_base_radio_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::BaseTabList => {
                            show_base_tab_list_example(vc, &theme, &font.default);
                        }
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
                        ShowcaseElement::AlertDialog => {
                            show_alert_dialog_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::AspectRatio => {
                            show_aspect_ratio_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::Portal => show_portal_example(
                            vc,
                            &theme,
                            &font.default,
                            Res::clone(&global_portal_root),
                        ),
                        ShowcaseElement::Slot => {
                            show_slot_example(vc, &theme, &font.default);
                        }
                        ShowcaseElement::DirectionProvider => {
                            show_direction_provider_example(vc, &theme, &font.default);
                        }
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
                        ShowcaseElement::VisuallyHidden => {
                            show_visually_hidden_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::AccessibleIcon => {
                            show_accessible_icon_example(vc, &theme, &font.default, &icons)
                        }
                        ShowcaseElement::HoverCard => {
                            show_hover_card_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Menubar => show_menubar_example(vc, &theme, &font.default),
                        ShowcaseElement::NavigationMenu => {
                            show_navigation_menu_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Popover => show_popover_example(vc, &theme, &font.default),
                        ShowcaseElement::Progress => {
                            show_progress_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::ScrollArea => {
                            show_scroll_area_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Select => show_select_example(vc, &theme, &font.default),
                        ShowcaseElement::Separator => {
                            show_separator_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::Slider => show_slider_example(vc, &theme, &font.default),
                        ShowcaseElement::Tabs => show_tabs_example(vc, &theme, &font.default),
                        ShowcaseElement::Toast => show_toast_example(vc, &theme, &font.default),
                        ShowcaseElement::Toolbar => show_toolbar_example(vc, &theme, &font.default),
                        ShowcaseElement::Tooltip => show_tooltip_example(vc, &theme, &font.default),
                        ShowcaseElement::OneTimePasswordField => {
                            show_one_time_password_field_example(vc, &theme, &font.default)
                        }
                        ShowcaseElement::PasswordToggleField => {
                            show_password_toggle_field_example(vc, &theme, &font.default)
                        }
                    });
            });
        }
    }
}
