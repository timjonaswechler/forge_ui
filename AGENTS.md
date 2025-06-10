# Hinweise für Agenten

## Projektkontext
Das Projekt ist in Rust geschrieben und verwendet Bevy 0.16 als Game Engine. Ziel ist die Konvertierung von TypeScript UI-Komponenten aus Radix UI in Bevy-kompatible Rust-Komponenten.

## Referenzen und Dokumentation
* Bevy-API Dokumentation: [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) - Primäre Referenz für alle Bevy-Mechanismen
* Radix UI GitHub: [github.com/radix-ui/](https://github.com/radix-ui/) - Inspiration für Komponentenverhalten und Styling
* Radix UI Website: [radix-ui.com](https://radix-ui.com) - Referenz für Komponentendesign und Funktionalität
* Template-Datei: `src/components/TEMPLATE.md` - Vorlage für einheitliche Komponentenstruktur

## Arbeitsablauf

### Aufgabentyp 1: Komponentenerstellung oder -überarbeitung
Wenn eine Komponente erstellt oder überarbeitet werden soll:

1. **Analyse**: Studiere die entsprechende Radix UI Komponente (GitHub Source und Website)
2. **Template**: Orientiere dich an `src/components/TEMPLATE.md` für die Struktur
3. **Implementierung**: Konvertiere die TypeScript-Logik in Bevy 0.16 Rust-Code
4. **Dokumentation**: Erstelle oder aktualisiere die Rust-Dokumentation der Komponente
5. **Checkbox aktualisieren**: Hacke in der Agents.md die Checkbox "Komponente erstellt" ab
6. **Nächste Aufgabe**: Wechsle automatisch zu Aufgabentyp 2 für dieselbe Komponente

### Aufgabentyp 2: Showcase-Erstellung
Nach erfolgreicher Komponentenerstellung:

1. **Showcase implementieren**: Erstelle oder aktualisiere die entsprechende Datei in `src/showcase/`
2. **Funktionstest**: Stelle sicher, dass die Komponente in der Showcase korrekt funktioniert
3. **Checkbox aktualisieren**: Hacke in der Agents.md die Checkbox "Showcase erstellt" ab
4. **Aufgabe abschließen**: Die Komponente ist vollständig abgeschlossen

## Arbeitsregeln

### Konzentration
* Arbeite immer nur an einer Komponente zur Zeit
* Schließe eine Komponente vollständig ab bevor du zur nächsten wechselst
* Beide Checkboxen einer Komponente müssen abgehakt sein bevor die nächste begonnen wird

### Automatische Navigation
* Erkenne selbständig welche Komponente als nächstes bearbeitet werden muss
* Priorisiere Komponenten mit keinen abgehakten Checkboxen
* Bei teilweise abgeschlossenen Komponenten: Vervollständige die fehlende Aufgabe

### Checkbox-Management
* Aktualisiere die Agents.md Datei nach jedem abgeschlossenen Schritt
* Verwende exakte Markdown-Syntax für Checkboxen: `- [x]` für abgehakt, `- [ ]` für offen
* Ändere nur die relevante Checkbox, lasse alle anderen unverändert

### Code-Qualität
* Befolge die Bevy 0.16 API exakt wie in der Dokumentation beschrieben
* Halte die einheitliche Struktur gemäß Template ein
* Verwende aussagekräftige Rust-Dokumentation mit Beispielen
* Stelle sicher, dass Styling und Verhalten der Radix UI Vorlage entsprechen

### Fehlerbehandlung
* Bei Unsicherheiten bezüglich Bevy-API: Konsultiere ausschließlich die offizielle Dokumentation
* Bei Unklarheiten bezüglich Komponentenverhalten: Referenziere die Radix UI Implementierung
* Dokumentiere alle Abweichungen von der Radix UI Vorlage mit Begründung

## Nächste Aktion
Identifiziere die erste unvollständige Komponente in der Liste und beginne mit der Implementierung oder vervollständige die fehlende Aufgabe.

## UI-Komponenten Liste

### Accordion
- Website: https://www.radix-ui.com/primitives/docs/components/accordion
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/accordion/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Alert Dialog
- Website: https://www.radix-ui.com/primitives/docs/components/alert-dialog
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/alert-dialog/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Avatar
- Website: https://www.radix-ui.com/primitives/docs/components/avatar
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/avatar/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Checkbox
- Website: https://www.radix-ui.com/primitives/docs/components/checkbox
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/checkbox/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Collapsible
- Website: https://www.radix-ui.com/primitives/docs/components/collapsible
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/collapsible/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Collection
- Website: https://www.radix-ui.com/primitives/docs/components/collection
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/collection/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Context
- Website: https://www.radix-ui.com/primitives/docs/components/context
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/context/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Context Menu
- Website: https://www.radix-ui.com/primitives/docs/components/context-menu
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/context-menu/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Dialog
- Website: https://www.radix-ui.com/primitives/docs/components/dialog
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/dialog/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Dropdown Menu
- Website: https://www.radix-ui.com/primitives/docs/components/dropdown-menu
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/dropdown-menu/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Hover Card
- Website: https://www.radix-ui.com/primitives/docs/components/hover-card
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/hover-card/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Label
- Website: https://www.radix-ui.com/primitives/docs/components/label
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/label/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Menu
- Website: https://www.radix-ui.com/primitives/docs/components/menu
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/menu/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Navigation Menu
- Website: https://www.radix-ui.com/primitives/docs/components/navigation-menu
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/navigation-menu/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### One-Time Password Field
- Website: https://www.radix-ui.com/primitives/docs/components/one-time-password-field
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/one-time-password-field/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Password Toggle Field
- Website: https://www.radix-ui.com/primitives/docs/components/password-toggle-field
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/password-toggle-field/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Popover
- Website: https://www.radix-ui.com/primitives/docs/components/popover
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/popover/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Portal
- Website: https://www.radix-ui.com/primitives/docs/components/portal
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/portal/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Progress
- Website: https://www.radix-ui.com/primitives/docs/components/progress
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/progress/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Radio Group
- Website: https://www.radix-ui.com/primitives/docs/components/radio-group
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/radio-group/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Scroll Areas
- Website: https://www.radix-ui.com/primitives/docs/components/scroll-areas
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/scroll-areas/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Select
- Website: https://www.radix-ui.com/primitives/docs/components/select
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/select/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Separator
- Website: https://www.radix-ui.com/primitives/docs/components/separator
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/separator/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Slider
- Website: https://www.radix-ui.com/primitives/docs/components/slider
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/slider/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Slot
- Website: https://www.radix-ui.com/primitives/docs/components/slot
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/slot/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Switch
- Website: https://www.radix-ui.com/primitives/docs/components/switch
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/switch/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Tabs
- Website: https://www.radix-ui.com/primitives/docs/components/tabs
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/tabs/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toast
- Website: https://www.radix-ui.com/primitives/docs/components/toast
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/toast/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toggle
- Website: https://www.radix-ui.com/primitives/docs/components/toggle
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/toggle/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toggle Group
- Website: https://www.radix-ui.com/primitives/docs/components/toggle-group
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/toggle-group/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Toolbar
- Website: https://www.radix-ui.com/primitives/docs/components/toolbar
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/toolbar/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt

### Tooltip
- Website: https://www.radix-ui.com/primitives/docs/components/tooltip
- Source: https://github.com/radix-ui/primitives/tree/main/packages/react/tooltip/src
- [ ] Komponente erstellt
- [ ] Showcase erstellt