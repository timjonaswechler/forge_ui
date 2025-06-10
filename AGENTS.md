# Hinweise für Agenten

## Projektkontext
Das Projekt ist in Rust geschrieben und verwendet Bevy 0.16 als Game Engine. Ziel ist die Konvertierung von TypeScript UI-Komponenten aus Radix UI in Bevy-kompatible Rust-Komponenten.

## Referenzen und Dokumentation
* Bevy-API Dokumentation: [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) - Primäre Referenz für alle Bevy-Mechanismen
* Radix UI GitHub: [github.com/radix-ui/](https://github.com/radix-ui/) - Inspiration für Komponentenverhalten und Styling
* Radix UI Website: [radix-ui.com](https://radix-ui.com) - Referenz für Komponentendesign und Funktionalität
* Template-Datei: `src/components/TEMPLATE.md` - Vorlage für einheitliche Komponentenstruktur

## allgemeiner Arbeitsablauf

* am Ende jeder Aufgabe prüfe mit `cargo check` ob der Code fehlerfrei ist, falls nicht
  korrigiere die Fehler und führe den Check erneut aus, gegebenenfalls in docs.rs nachsehen.

## Arbeitsablauf wenn es um UI-Komponenten geht

### Aufgabentyp 1: Komponentenerstellung oder -überarbeitung
Wenn eine Komponente erstellt oder überarbeitet werden soll:

1.  **Analyse**: Studiere die entsprechende Radix UI Komponente (GitHub)
2.  **Template (Neuerstellung)**: Orientiere dich bei der *Neuerstellung* einer Komponente an `src/components/TEMPLATE.md` für die Struktur.
3.  **Template-Prüfung (Überarbeitung)**: Wenn eine *bestehende* Komponente überarbeitet oder eine unvollständige Komponentenerstellung fortgesetzt wird, **prüfe zwingend**, ob der bisherige Code den Standards der `src/components/TEMPLATE.md` entspricht. Passe ihn bei Abweichungen an, um die einheitliche Struktur sicherzustellen.
4.  **Implementierung**: Konvertiere die TypeScript-Logik in Bevy 0.16 Rust-Code.
5.  **Dokumentation**: Erstelle oder aktualisiere die Rust-Dokumentation der Komponente.
6.  **Checkbox aktualisieren**: Hacke in der Agents.md die Checkbox "Komponente erstellt" ab.
7.  **Nächste Aufgabe**: Wechsle automatisch zu Aufgabentyp 2 für dieselbe Komponente.

### Aufgabentyp 2: Showcase-Erstellung
Nach erfolgreicher Komponentenerstellung:

1.  **Showcase implementieren**: Erstelle oder aktualisiere die entsprechende Datei in `src/showcase/`
2.  **Funktionstest**: Stelle sicher, dass die Komponente in der Showcase korrekt funktioniert
3.  **Checkbox aktualisieren**: Hacke in der Agents.md die Checkbox "Showcase erstellt" ab
4.  **Aufgabe abschließen**: Die Komponente ist vollständig abgeschlossen

## Arbeitsregeln

### Konzentration
*   Arbeite immer nur an einer Komponente zur Zeit
*   Schließe eine Komponente vollständig ab bevor du zur nächsten wechselst
*   Beide Checkboxen einer Komponente müssen abgehakt sein bevor die nächste begonnen wird

### Automatische Navigation
*   Erkenne selbständig welche Komponente als nächstes bearbeitet werden muss
*   Bei teilweise abgeschlossenen Komponenten: Vervollständige die fehlende Aufgabe

### Checkbox-Management
*   Aktualisiere die Agents.md Datei nach jedem abgeschlossenen Schritt
*   Verwende exakte Markdown-Syntax für Checkboxen: `- [x]` für abgehakt, `- [ ]` für offen
*   Ändere nur die relevante Checkbox, lasse alle anderen unverändert

### Code-Qualität
*   Befolge die Bevy 0.16 API exakt wie in der Dokumentation beschrieben
*   Halte die einheitliche Struktur gemäß Template ein (siehe Aufgabentyp 1, Punkte 2 & 3)
*   Verwende aussagekräftige Rust-Dokumentation mit Beispielen
*   Stelle sicher, dass Styling und Verhalten der Radix UI Vorlage entsprechen

### Fehlerbehandlung
*   Bei Unsicherheiten bezüglich Bevy-API: Konsultiere ausschließlich die offizielle Dokumentation
*   Bei Unklarheiten bezüglich Komponentenverhalten: Referenziere die Radix UI Implementierung
*   Dokumentiere alle Abweichungen von der Radix UI Vorlage mit Begründung

## Nächste Aktion
Identifiziere die erste unvollständige Komponente in der Liste und beginne mit der Implementierung oder vervollständige die fehlende Aufgabe.

## UI-Komponenten Liste

### Badge
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.tsx`
- [x] Komponente erstellt
- [x] Showcase erstellt
- [ ]

### Button
- Source:
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.props.ts`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.tsx`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.tsx`
- [x] Komponente erstellt
- [x] Showcase erstellt

### Checkbox
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/checkbox/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-checkbox.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-checkbox.props.ts`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.tsx`
- [x] Komponente erstellt
- [x] Showcase erstellt

### Checkbox Cards
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Checkbox Group
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-group.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-group.primitive.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-group.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-group.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Dialog
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/dialog/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-dialog.css`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Label
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/label/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Portal
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/portal/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/portal.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Radio
- Source:
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-radio.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-radio.props.ts`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.css` (Individual radio item styles)
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Radio Cards
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-cards.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-cards.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-cards.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Radio Group
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/radio-group/src`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Switch
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/switch/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toggle
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/toggle/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toggle Group
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/toggle-group/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt


### Accordion
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/accordion/src`
- [x] Komponente erstellt
- [x] Showcase erstellt

### Alert Dialog
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/alert-dialog/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/alert-dialog.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/alert-dialog.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/alert-dialog.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Avatar
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/avatar/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/avatar.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/avatar.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/avatar.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Blockquote
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/blockquote.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/blockquote.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/quote.css` (Related: Quote often used for blockquotes styling)
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Box
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/box.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/box.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/box.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Callout
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/callout.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/callout.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/callout.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Card
- Source:
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-card.css`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/card.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/card.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/card.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Code
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/code.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/code.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/code.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Collapsible
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/collapsible/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Collection
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/collection/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Container
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/container.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/container.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/container.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Context (Primitive)
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/context/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Context Menu
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/context-menu/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.css` (shared base)
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.props.ts` (shared base)
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/context-menu.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/context-menu.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Data List
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/data-list.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/data-list.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/data-list.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt



### Dropdown Menu
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/dropdown-menu/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.css` (shared base)
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.props.ts` (shared base)
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dropdown-menu.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dropdown-menu.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dropdown-menu.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Em (Emphasis)
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/em.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/em.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/em.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Flex
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/flex.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/flex.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/flex.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Grid
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/grid.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/grid.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/grid.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Heading
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/heading.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/heading.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/heading.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Hover Card
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/hover-card/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/hover-card.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/hover-card.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/hover-card.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Icon Button
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/icon-button.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/icon-button.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/icon-button.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Inset
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/inset.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/inset.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/inset.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Kbd (Keyboard Key)
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/kbd.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/kbd.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/kbd.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Label
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/label/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Link
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/link.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/link.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/link.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Menu (Primitive Base)
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/menu/src`
  - Themes (_internal_, shared for ContextMenu, DropdownMenu):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-menu.props.ts`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Navigation Menu
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/navigation-menu/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### One-Time Password Field
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/one-time-password-field/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Password Toggle Field
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/password-toggle-field/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Popover
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/popover/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/popover.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/popover.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/popover.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Portal
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/portal/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/portal.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Progress
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/progress/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/progress.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/progress.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/progress.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Quote
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/quote.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/quote.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/quote.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Scroll Area (Scroll Areas)
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/scroll-areas/src` (Note: plural "areas" in primitives)
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/scroll-area.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/scroll-area.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/scroll-area.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Section
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/section.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/section.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/section.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Segmented Control
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/segmented-control.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/segmented-control.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/segmented-control.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Select
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/select/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/select.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/select.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/select.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Separator
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/separator/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/separator.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/separator.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/separator.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Skeleton
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/skeleton.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/skeleton.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/skeleton.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Slider
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/slider/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/slider.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/slider.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/slider.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Slot
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/slot/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/slot.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Spinner
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/spinner.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/spinner.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/spinner.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Strong
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/strong.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/strong.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/strong.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt



### Tab Nav
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tab-nav.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tab-nav.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tab-nav.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-tab-list.css` (shared base)
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-tab-list.props.ts` (shared base)
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Table
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/table.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/table.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/table.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Tabs
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/tabs/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tabs.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tabs.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tabs.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-tab-list.css` (shared base)
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-tab-list.props.ts` (shared base)
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Text
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Text Area
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-area.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-area.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-area.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Text Field
- Source:
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-field.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-field.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/text-field.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toast
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/toast/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toolbar
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/toolbar/src`
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Tooltip
- Source:
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/tooltip/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tooltip.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tooltip.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/tooltip.tsx`
- [ ] Komponente erstellt
- [ ] Showcase erstellt
