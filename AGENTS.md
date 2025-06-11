# Hinweise für Agenten - Hauptdatei

## Projektkontext
Das Projekt ist in Rust geschrieben und verwendet Bevy 0.16 als Game Engine. Ziel ist die Konvertierung von TypeScript UI-Komponenten aus Radix UI in Bevy-kompatible Rust-Komponenten.

## Referenzen und Dokumentation
* Bevy-API Dokumentation: [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) - Primäre Referenz für alle Bevy-Mechanismen
* Radix UI GitHub: [github.com/radix-ui/](https://github.com/radix-ui/) - Inspiration für Komponentenverhalten und Styling
* Radix UI Website: [radix-ui.com](https://radix-ui.com) - Referenz für Komponentendesign und Funktionalität
* Template-Datei: `src/components/TEMPLATE.md` - Vorlage für einheitliche Komponentenstruktur

## Allgemeiner Arbeitsablauf

* Am Ende jeder Aufgabe prüfe mit `cargo check` ob der Code fehlerfrei ist, falls nicht korrigiere die Fehler und führe den Check erneut aus, gegebenenfalls in docs.rs nachsehen.

## Aufgabentypen - Übersicht

### 🔄 Bestehende Komponenten überarbeiten
**Datei:** `BESTEHENDE_KOMPONENTEN.md`  
**Beschreibung:** Anpassung bereits vorhandener Komponenten an die aktuellen Guidelines und Template-Standards.

### ✨ Neue Komponenten erstellen  
**Datei:** `NEUE_KOMPONENTEN.md`  
**Beschreibung:** Vollständige Erstellung neuer UI-Komponenten von Grund auf inklusive Showcase.

### 🔧 [Zukünftige Aufgaben]
**Datei:** `[Noch nicht definiert]`  
**Beschreibung:** Platz für weitere Aufgabentypen wie Tests, Dokumentation, Performance-Optimierung, etc.

### 📦 [Integration & Deployment]
**Datei:** `[Noch nicht definiert]`  
**Beschreibung:** Aufgaben rund um Integration, Build-Prozesse und Deployment.

## Arbeitsregeln

### Konzentration
* Arbeite immer nur an einer Komponente zur Zeit
* Schließe eine Komponente vollständig ab bevor du zur nächsten wechselst
* Beide Checkboxen einer Komponente müssen abgehakt sein bevor die nächste begonnen wird

### Automatische Navigation
* Erkenne selbständig welche Komponente als nächstes bearbeitet werden muss
* Bei teilweise abgeschlossenen Komponenten: Vervollständige die fehlende Aufgabe

### Checkbox-Management
* Aktualisiere die entsprechende Aufgabendatei nach jedem abgeschlossenen Schritt
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
1. Prüfe `BESTEHENDE_KOMPONENTEN.md` für unvollständige Überarbeitungen
2. Falls keine vorhanden, prüfe `NEUE_KOMPONENTEN.md` für zu erstellende Komponenten
3. Identifiziere die erste unvollständige Komponente und beginne mit der Implementierung

## Status-Übersicht
- ⏳ **Bestehende Komponenten:** Siehe `BESTEHENDE_KOMPONENTEN.md`
- ⏳ **Neue Komponenten:** Siehe `NEUE_KOMPONENTEN.md`
- 📊 **Gesamt-Fortschritt:** [Wird automatisch aktualisiert]