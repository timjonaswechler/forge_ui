**Phase 1: Theming System Kern**

*   [x] Enum `Appearance` definieren (`Light`, `Dark`)
*   [x] Enum/Struct `ColorPaletteRef` definieren (für Akzent-/Graufarben-Auswahl)
*   [x] Enum `RadiusSetting` definieren (`None`, `Small`, `Medium`, `Large`, `Full`)
*   [x] Structs/Konstanten für Farbpaletten definieren (z.B. `IndigoPalette { step1: Color, ... }`, inspiriert von Radix Colors)
*   [x] Haupt-`Theme` Resource definieren (`pub struct Theme { ... }`)
*   [🔥] Grundlegende Felder zur `Theme` Resource hinzufügen (`appearance`, `high_contrast`, `scaling`)
*   [x] Felder für Farbpaletten-Referenzen zur `Theme` Resource hinzufügen (`accent_color: ColorPaletteRef`, `gray_color: ColorPaletteRef`)
*   [x] Feld für `RadiusSetting` zur `Theme` Resource hinzufügen (`radius`)
*   [x] Felder für Schriftart-Handles zur `Theme` Resource hinzufügen (`default_font: Handle<Font>`, `code_font: Handle<Font>`)
*   [x] Theme-Resource mit Standardwerten im Plugin registrieren (`app.init_resource::<Theme>()`)
*   [x] Komponente `ThemeColor` definieren (Enum/Struct, z.B. `Accent(u8)`, `Gray(u8)`, `Background`, `Surface`, `TextHighContrast`, etc.)
*   [ ] System `apply_theme_colors` erstellen
*   [ ] System `apply_theme_colors`: `Theme` Resource lesen
*   [ ] System `apply_theme_colors`: Über Entities mit `ThemeColor` und `BackgroundColor` / `BorderColor` iterieren
*   [ ] System `apply_theme_colors`: Basierend auf `Theme` und `ThemeColor` die tatsächliche Farbe berechnen (inkl. `Appearance`, `high_contrast` Logik)
*   [ ] System `apply_theme_colors`: Bevy-Komponenten (`BackgroundColor.0`, `BorderColor.0`) aktualisieren
*   [ ] System `apply_theme_colors`: Change Detection hinzufügen (`Changed<Theme>`, `Added<ThemeColor>`, `Changed<ThemeColor>`)
*   [ ] System `apply_theme_colors` zum Plugin hinzufügen
*   [ ] Helper-Funktion `spawn_themed_box` erstellen (spawnt `NodeBundle` + `ThemeColor` für Hintergrund)
*   [ ] `spawn_themed_box` in einem Beispiel testen

**Phase 2: Typografie**

*   [ ] Mechanismus zum Laden von Standard-/Code-Schriftarten implementieren (z.B. beim Plugin-Start oder über Konfig)
*   [ ] Geladene Schriftart-Handles in der `Theme` Resource speichern/aktualisieren
*   [ ] Enum `TextSize` definieren (`Size1` bis `Size9`)
*   [ ] Enum `FontWeight` definieren (`Light`, `Regular`, `Medium`, `Bold`)
*   [ ] `ThemedTextBundle` (oder Wrapper/Komponente) definieren (mit `TextBundle`, `TextSize`, `FontWeight`, `ThemeColor`)
*   [ ] System `apply_text_styles` erstellen
*   [ ] System `apply_text_styles`: `Theme` Resource lesen
*   [ ] System `apply_text_styles`: Über Entities mit `ThemedTextBundle` (oder den relevanten Komponenten) und `Text` iterieren
*   [ ] System `apply_text_styles`: `Text.sections[...].style.font` basierend auf `Theme` setzen (Default vs. Code)
*   [ ] System `apply_text_styles`: `Text.sections[...].style.font_size` basierend auf `TextSize` und `Theme.scaling` berechnen und setzen
*   [ ] System `apply_text_styles`: `Text.sections[...].style.color` basierend auf `ThemeColor` und Theme-Einstellungen setzen (ggf. Integration mit `apply_theme_colors`)
*   [ ] System `apply_text_styles`: Mapping von `FontWeight` implementieren (Best-Effort)
*   [ ] System `apply_text_styles`: Change Detection hinzufügen
*   [ ] System `apply_text_styles` zum Plugin hinzufügen
*   [ ] Helper-Funktion `spawn_themed_text` erstellen
*   [ ] `spawn_themed_text` in einem Beispiel testen

**Phase 3: Layout & Responsiveness**

*   [ ] Enum `Breakpoint` definieren (`Initial`, `Xs`, ...) mit zugehörigen `min_width` Werten
*   [ ] Resource `CurrentBreakpoint` definieren und initialisieren
*   [ ] System `update_breakpoint_system` erstellen
*   [ ] System `update_breakpoint_system`: Auf `WindowResized` Events hören
*   [ ] System `update_breakpoint_system`: Aktuelle Fensterbreite lesen
*   [ ] System `update_breakpoint_system`: Höchsten passenden `Breakpoint` bestimmen
*   [ ] System `update_breakpoint_system`: `CurrentBreakpoint` Resource aktualisieren (nur bei Änderung)
*   [ ] System `update_breakpoint_system` zum Plugin hinzufügen
*   [ ] Generische Struktur `Responsive<T>` definieren (z.B. `struct Responsive<T> { initial: Option<T>, sm: Option<T>, ... }`)
*   [ ] Komponente `ResponsiveStyle` definieren (enthält `Responsive<T>` für diverse Style-Props wie `padding`, `margin`, `flex_direction`, `display`, etc.)
*   [ ] System `apply_responsive_styles` erstellen
*   [ ] System `apply_responsive_styles`: Auf Änderungen von `CurrentBreakpoint` und `ResponsiveStyle` reagieren
*   [ ] System `apply_responsive_styles`: Für jede Entity mit `ResponsiveStyle` den effektiven Wert für jede Eigenschaft basierend auf `CurrentBreakpoint` ermitteln (Fallback-Logik)
*   [ ] System `apply_responsive_styles`: Entsprechende Bevy-Komponenten (`Style`, etc.) mit effektiven Werten aktualisieren
*   [ ] System `apply_responsive_styles` zum Plugin hinzufügen
*   [ ] Builder/Bundle `spawn_box` mit `ResponsiveStyle` erstellen
*   [ ] Builder/Bundle `spawn_flex` mit `ResponsiveStyle` erstellen
*   [ ] Builder/Bundle `spawn_grid` mit `ResponsiveStyle` erstellen
*   [ ] Responsive Layout in einem Beispiel testen

**Phase 4: UI-Komponenten-Primitives**

*   **Button:**
    *   [x] `ThemedButtonBundle` definieren (inkl. `ButtonBundle`, `ThemeColor`, `ThemeRadius` (Platzhalter), `ResponsiveStyle`)
    *   [x] Builder `spawn_themed_button` erstellen
    *   [x] Varianten (`Solid`, `Soft`, `Outline`, `Ghost`, `Classic`) im Styling-System implementieren (über `ThemeColor` / `Style`)
    *   [x] Größen (`Size1` bis `Size4`) im Styling-System implementieren (über `ResponsiveStyle` für Padding, `TextSize`)
    *   [x] Button in Beispiel testen (mit Varianten/Größen)
*   **Text / Heading:**
    *   [ ] `ThemedTextBundle` ggf. verfeinern
    *   [ ] `ThemedHeadingBundle` erstellen (semantisch anders, visuell über size/weight gesteuert)
    *   [ ] Builder `spawn_themed_heading` erstellen
*   **Card:**
    *   [ ] `ThemedCardBundle` definieren (`NodeBundle`, `ThemeColor`, `ThemeRadius` (Platzhalter), `ResponsiveStyle` für Padding)
    *   [ ] Builder `spawn_themed_card` erstellen
    *   [ ] Varianten (`Surface`, `Classic`, `Ghost`) implementieren
*   **Checkbox:**
    *   [ ] `ThemedCheckboxBundle` definieren (evtl. mit `Checked(bool)` Komponente)
    *   [ ] Interaktionssystem für Checkbox erstellen (reagiert auf `Interaction`, ändert `Checked` und Aussehen)
    *   [ ] Visuelles Styling für Checked/Unchecked implementieren (Farben, evtl. Icon)
*   **Radio Button:**
    *   [ ] `ThemedRadioBundle` definieren (benötigt Gruppierungslogik!)
    *   [ ] Komponente für Radiogruppe (`RadioGroup { id: ... }`)
    *   [ ] Interaktionssystem für Radio Buttons (ändert Zustand innerhalb der Gruppe)
    *   [ ] Visuelles Styling implementieren
*   **Switch:**
    *   [x] `ThemedSwitchBundle` definieren (mit `Checked(bool)` Komponente)
    *   [x] Interaktionssystem für Switch erstellen
    *   [x] Visuelles Styling für On/Off implementieren (Thumb-Position, Farben)
*   **Slider:**
    *   [ ] `ThemedSliderBundle` definieren (mit `Value(f32)` Komponente)
    *   [ ] Interaktionssystem für Slider (Drag & Drop Logik)
    *   [ ] Visuelles Styling implementieren (Track, Thumb, Range)
*   **TextField / TextArea:**
    *   [ ] `ThemedTextFieldBundle` / `ThemedTextAreaBundle` definieren
    *   [ ] Integration mit Bevy's Texteingabe-Events (`ReceivedCharacter`)
    *   [ ] Zustand für Textinhalt verwalten (`Value(String)`)
    *   [ ] Grundlegendes Styling implementieren (Hintergrund, Border, Text)
*   **Radius-Anwendung:**
    *   [ ] Komponente `ThemeRadius` definieren
    *   [ ] Implementierungsstrategie für Radius wählen (Ignorieren, 9-Patch, Custom Rendering)
    *   [ ] (Optional) `apply_radius` System / 9-Patch Assets / Custom Shader implementieren
*   **High-Contrast Verfeinerung:**
    *   [ ] High-Contrast-Logik in `apply_theme_colors` und `apply_text_styles` überprüfen und verfeinern
    *   [ ] Sicherstellen, dass alle Komponenten korrekt auf `Theme.high_contrast` reagieren

**Phase 5: Fortgeschrittene Komponenten & Features**

*   **Overlays (Allgemein):**
    *   [ ] Strategie für Overlay-Management überlegen (Separate UI Kamera? Z-Index?)
*   **Dialog:**
    *   [x] `ThemedDialogBundle` (Root, Trigger, Content, Title, Description, Close)
    *   [x] System für Sichtbarkeit/Zustand implementieren
    *   [x] Overlay/Modal-Verhalten implementieren (blockiert Hintergrund?)
    *   [x] Positionierung (zentriert)
*   **Popover:**
    *   [ ] `ThemedPopoverBundle` (Root, Trigger, Content, Close)
    *   [ ] System für Sichtbarkeit/Zustand
    *   [ ] Positionierungslogik relativ zum Trigger (komplex!)
*   **Tooltip:**
    *   [ ] `ThemedTooltipBundle` (Trigger, Content)
    *   [ ] Sichtbarkeit basierend auf Hover
    *   [ ] Positionierung
*   **Dropdown Menu:**
    *   [ ] `ThemedDropdownMenuBundle` (Root, Trigger, Content, Item, Separator, Sub, ...)
    *   [ ] Sichtbarkeit/Zustand, Navigation, Schließen-Logik
    *   [ ] Positionierung
*   **Context Menu:**
    *   [ ] `ThemedContextMenuBundle` (ähnlich Dropdown, aber anderer Trigger)
    *   [ ] Trigger auf Rechtsklick/Long Press
*   **Layout Utilities:**
    *   [ ] `AspectRatio` Komponente/Bundle implementieren
    *   [ ] `Separator` Komponente/Bundle implementieren
    *   [ ] `Spacer` Komponente/Bundle implementieren (Flexbox Grow/Shrink Box)
*   **Accessibility (Basis):**
    *   [ ] (Optional) Semantische Komponenten wie `Role`, `AriaLabel` hinzufügen
*   **API & Doku:**
    *   [ ] Builder-Funktionen überprüfen und ergonomischer gestalten
    *   [ ] Inline-Dokumentation (Doc Comments) hinzufügen
    *   [ ] Umfassende Beispiele für jede Komponente erstellen
*   **Performance:**
    *   [ ] Systeme auf unnötige Iterationen prüfen
    *   [ ] Change Detection in allen relevanten Systemen sicherstellen
    *   [ ] (Bei Bedarf) UI mit `bevy_dev_tools` oder ähnlichem profilen

Diese Liste ist umfangreich. Du kannst die Reihenfolge anpassen und Aufgaben weiter unterteilen, wenn nötig. Viel Erfolg beim Abhaken!


accordion
alert-dialog
arrow
aspect-ratio
avatar
checkbox
collapsible
collection
compose-refs
context-menu
context
dialog
direction
dismissable-layer
dropdown-menu
focus-guards
focus-scope
form
hover-card
id
label
menu
menubar
navigation-menu
one-time-password-field
password-toggle-field
popover
popper
portal
presence
primitive
progress
radio-group
radix-ui
roving-focus
scroll-area
select
separator
slider
slot
switch
tabs
toast
toggle-group
toggle

accordion
alert-dialog
announce
arrow
aspect-ratio
avatar
checkbox
collapsible
collection
compose-refs
context-menu
context
dialog
direction
dismissable-layer
dropdown-menu
focus-guards
focus-scope 
hover-card
id
label
menu
menubar
navigation-menu
one-time-password-field
password-toggle-field
popover
popper
portal
presence
primitive
progress

radio-group
radix-ui
roving-focus
scroll-area
select
separator
slider
slot

tabs
toast
toggle-group
toggle