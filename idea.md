# Idee wie das Plugin ForgeUiPlugin in Bevy zu verwenden

## Initialize the Forge UI plugin with ThemeSettings ( other  then default settings )

```rs
.add_plugins(ForgeUiPlugin::new(ThemeSettings{
    apperance: ThemeAppearance::Light,
    accent_color: AccentColor::Blue || AccentColor::Costum("#FF0000") || AccentColor::Costum("meinefarbe"),
    panelBackground: PanelBackground::Solid || PanelBackground::Translucent,
    scaling: 1.0,
    radius: Radius::XSmall,
    high_contrast: false,
    font_size: 16.0, // REM
    ..default()
}));

// or

.add_plugins(ForgeUiPlugin::new(ThemeSettings{
    ..default()
}));

enum ThemeAppearance {
    Light,
    Dark,
}

enum PanelBackground {
    Solid,
    Translucent,
}

enum Radius {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    X2Large,
    X3Large,
    X4Large,
    Full,
    None, 
}
enum AccentColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Orange,
    Pink,
    Teal,
    Cyan,
    Brown,
    Gray,
    Black,
    White,
    Custom(String), // Hex color code
}
```

## Primitive UI Elements

### Button

```rs

enum ButtonColorVariant{
    Primary, // Accent color
    Secondary, // Gray Color
    Danger, // Red Color
    Success, // Green Color
    Warning, // Yellow Color
    Info, // Blue Color
}

enum ButtonStyleVariant {
    Solid,
    Soft,
    Outline,
    Surface,
    Ghost,
}

enum ButtonSize {
    Small,
    Medium,
    Large,
}
IconPosition {
    Left,
    Right,
}


ButtonBuilder::new()
    .text("Click me")
    .variant(ButtonColorVariant::Primary) 
    .color(AccentColor::Blue) // color overrides the variant  und gibt info das ColorVariant ignoriert werden kann wenn es gesetzt ist 
    .on_click(|_ctx, _button| {
        println!("Button clicked!");
    })
    .size(ButtonSize::Medium) // default
    .radius(Radius::Small)
    .icon(Handle<Image>::default()) // Handle<Image> or String
    .icon_position(IconPosition::Left) // default
    .disabled(false) // default is false
    .loading(false) // default is false
    .build()
```

### Dialog

```rs

let overlay = DialogOverlayBuilder::new()
    .color(AccentColor::Blue) // default is AccentColor::Black
    .opacity(0.5) // default is 0.5
    .build();

let title = DialogTitleBuilder::new()
    .text("Dialog Title")
    .icon(Handle<Image>::default()) // Handle<Image> or String
    .build();

let body = DialogBodyBuilder::new()
    .elements(vec![
        // Add your UI elements here
        ButtonBuilder::new()
            .text("Button 1")
            .on_click(|_ctx, _button| {
                println!("Button 1 clicked!");
            })
            .build(),
        ButtonBuilder::new()
            .text("Button 2")
            .on_click(|_ctx, _button| {
                println!("Button 2 clicked!");
            })
            .build(),
    ])
    .build();


let content = DialoContenttBuilder::new()
    .title("Dialog Title") || .title(title) // Added title parameter    
    .description("This is a message.")
    .body(body)
    .close_button(true) // default is true
    .on_accept(|_ctx, _dialog| {
        println!("Accepted!");
    })
    .on_cancel(|_ctx, _dialog| {
        println!("Cancelled!");
    })
    .accept_text("OK")
    .cancel_text("Cancel")
    .build();

```
