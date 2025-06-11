# Bestehende Komponenten - Überarbeitung

## Arbeitsablauf für bestehende Komponenten

### Aufgabentyp: Komponentenüberarbeitung
Wenn eine **bestehende** Komponente überarbeitet werden soll:

1. **Analyse**: Studiere die entsprechende Radix UI Komponente (GitHub) und den bestehenden Rust-Code
2. **Template-Prüfung**: **Prüfe zwingend**, ob der bisherige Code den Standards der `src/components/TEMPLATE.md` entspricht. Passe ihn bei Abweichungen an, um die einheitliche Struktur sicherzustellen.
3. **Code-Überarbeitung**: Bringe den bestehenden Code auf den neuesten Stand:
   - Bevy 0.16 API Compliance
   - Template-Struktur-Compliance  
   - Code-Qualität und Dokumentation
4. **Plugin-Überarbeitung**: Überarbeite das bestehende Plugin falls notwendig
5. **Dokumentation**: Aktualisiere die Rust-Dokumentation der Komponente
6. **Checkbox aktualisieren**: Hacke die Checkbox "Bestehender Code an die Guidelines anpassen" ab
7. **Showcase prüfen**: Falls ein Showcase existiert, prüfe und aktualisiere ihn. Falls keiner existiert, erstelle einen.
8. **Showcase Checkbox**: Hacke "Showcase erstellt/aktualisiert" ab
9. **Aufgabe abschließen**: Die Komponente ist vollständig überarbeitet

## Komponenten-Liste

### Badge
- **Source:**
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/badge.tsx`
- [x] Bestehender Code an die Guidelines anpassen
- [x] Showcase erstellt/aktualisiert

### Button
- **Source:**
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.props.ts`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-button.tsx`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/button.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Checkbox
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/checkbox/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-checkbox.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-checkbox.props.ts`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Checkbox Cards
- **Source:**
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/checkbox-cards.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Dialog
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/dialog/src`
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-dialog.css`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/dialog.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Label
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/label/src`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Portal
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/portal/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/portal.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Radio
- **Source:**
  - Themes (_internal_):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-radio.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/_internal/base-radio.props.ts`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Radio Group
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/radio-group/src`
  - Themes (components):
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/radio-group.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Switch
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/switch/src`
  - Themes:
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.css`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.props.tsx`
    - `https://github.com/radix-ui/themes/tree/main/packages/radix-ui-themes/src/components/switch.tsx`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Toggle
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/toggle/src`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

### Accordion
- **Source:**
  - Primitives: `https://github.com/radix-ui/primitives/tree/main/packages/react/accordion/src`
- [ ] Bestehender Code an die Guidelines anpassen
- [ ] Showcase erstellt/aktualisiert

---

## Status-Übersicht
**Gesamt:** 12 Komponenten  
**Abgeschlossen:** 0 Komponenten  
**In Bearbeitung:** 0 Komponenten  
**Ausstehend:** 12 Komponenten