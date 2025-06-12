use crate::prelude::*;
use bevy::prelude::*;

/// Marker für den rechten Content-Bereich
#[derive(Component)]
pub struct ContentContainer;

/// Marker für das gesamte Showcase-UI (Root-Entity)
#[derive(Component)]
pub struct ShowcaseMarker;

/// Alle möglichen Aktionen, die Buttons im Showcase auslösen können
#[derive(Clone, Debug, PartialEq, Eq, Component)]
pub enum ShowcaseAction {
    /// Komplettes Showcase-UI ein-/ausblenden
    Toggle,
    /// Ein bestimmtes Demo-Element zeigen
    ShowElement(ShowcaseElement),
}

/// Die Demo-Elemente, aus denen die Sidebar wählt
#[derive(Clone, Debug, PartialEq, Eq, Component)]
pub enum ShowcaseElement {
    BaseCard,
    BaseButton,
    BaseCheckbox,
    BaseDialog,
    BaseMenu,
    BaseRadio,
    BaseTabList,
    Button,
    Accordion,
    Checkbox,
    CheckboxCards,
    RadioCards,
    CheckboxGroup,
    RadioGroup,
    Toggle,
    Label,
    Dialog,
    AlertDialog,
    AspectRatio,
    Portal,
    Slot,
    DirectionProvider,
    Badge,
    Switch,
    ToggleGroup,
    Avatar,
    Card,
    Blockquote,
    Box,
    Callout,
    Code,
    Collapsible,
    Collection,
    Container,
    Context,
    VisuallyHidden,
    AccessibleIcon,
    AccessibleIconTheme,
    HoverCard,
    Menubar,
    NavigationMenu,
    Popover,
    Progress,
    ScrollArea,
    Select,
    Separator,
    Slider,
    Tabs,
    Toast,
    Toolbar,
    Tooltip,
    OneTimePasswordField,
    PasswordToggleField,
}

// Hilfsfunktion zur Erstellung von beschrifteten Varianten
pub fn create_variant_section<'a>(
    parent: &'a mut ChildSpawnerCommands,
    title: &str,
    theme: &UiTheme,
    font: &Handle<Font>,
) -> EntityCommands<'a> {
    let mut cmd = parent.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(8.0)),
            margin: UiRect::vertical(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(theme.color.slate.step01),
        BorderColor(theme.color.slate.step04),
        Name::new(format!("{} Section", title)),
    ));
    // Überschrift
    cmd.with_children(|cb| {
        cb.spawn((
            Text::new(title),
            TextFont {
                font: font.clone(),
                font_size: theme.font.size.lg,
                ..default()
            },
            TextColor(theme.color.slate.step11),
            Name::new(format!("{} Title", title)),
        ));
        cb.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Auto,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(theme.color.slate.step02),
            BorderColor(theme.color.slate.step04),
            Name::new(format!("{} Container", title)),
        ));
    });
    cmd
}
