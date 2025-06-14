# Anweisungen für Agenten - Hauptdatei

## 🎯 Projektziel

Das Projekt ist in Rust geschrieben und verwendet Bevy 0.16 als Game Engine. Ziel ist die Konvertierung von Radix UI TypeScript-Komponenten in Bevy-kompatible Rust-Komponenten. Du wirst Komponenten implementieren und, falls bereits eine Basisversion existiert, diese um die "Themed"-Funktionalität erweitern.

## Referenzen und Dokumentation

-   Bevy-API Dokumentation: [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) - Primäre Referenz für alle Bevy-Mechanismen.
-   Radix UI GitHub: [github.com/radix-ui/](https://github.com/radix-ui/) - Inspiration für Komponentenverhalten und Styling.
-   Template-Datei: `src/components/TEMPLATE.md` - Vorlage für einheitliche Komponentenstruktur.
-   **Master Task List:** `MASTER_TASK_LIST.md` - Deine **einzige Quelle der Wahrheit** für Aufgaben und Fortschritt.

---

## ⚙️ Der dynamische Arbeits-Algorithmus (Die Goldene Regel)

Deine Arbeit wird durch den Befehl **"Next UI Element"** ausgelöst. Daraufhin führst du den folgenden, zweiteiligen Prozess aus.

### **Teil 1: Finde dein Hauptziel**

1.  **Öffne `MASTER_TASK_LIST.md`**.
2.  **Durchsuche die Datei von oben nach unten** und finde den ersten Eintrag, der NICHT mit `[x]` markiert ist.
3.  **Dieses Element ist dein neues Hauptziel** (z.B. "Themed Components / Accessible-Icon").
4.  **Starte TEIL 2** dieses Algorithmus mit diesem Hauptziel.

### **Teil 2: Der rekursive Implementierungs-Prozess**

Dieser Prozess wird mit einem **aktuellen Ziel** gestartet.

#### **Schritt A: Analyse & Abhängigkeits-Prüfung**

1.  **Informationssammlung:** Finde dein aktuelles Ziel in der `MASTER_TASK_LIST.md`, um die URLs zur Dokumentation und den Quelldateien zu erhalten. Studiere diese Quellen gründlich.

2.  **Abhängigkeiten identifizieren:** Untersuche die `.tsx`-Quelldateien deines Ziels auf `import`-Anweisungen, die auf *andere* Radix-Komponenten verweisen (z.B. `AlertDialog` importiert `Dialog`). Lokale Hilfsdateien sind keine externen Abhängigkeiten.

3.  **Abhängigkeitsstatus prüfen:**
    -   Prüfe für **JEDE identifizierte Abhängigkeit**, ob der entsprechende Eintrag in der `MASTER_TASK_LIST.md` bereits mit `[x]` markiert ist.
    -   **Sind alle Abhängigkeiten markiert?**
        -   **JA:** Sehr gut. Fahre fort mit **Schritt B: Bestandsaufnahme und Ausführung**.
        -   **NEIN:** Eine oder mehrere Abhängigkeiten fehlen. Du musst diesen Algorithmus rekursiv anwenden:
            1.  **Pausiere die Arbeit** am aktuellen Ziel.
            2.  **Nimm die erste fehlende Abhängigkeit als dein NEUES aktuelles Ziel.**
            3.  **Starte Teil 2 von vorne (Schritt A)** mit dieser Abhängigkeit als Ziel.
            4.  Kehre erst zum pausierten Ziel zurück, wenn die Abhängigkeit abgeschlossen ist, und wiederhole die Abhängigkeits-Prüfung.

#### **Schritt B: Bestandsaufnahme und Ausführung**

**Die entscheidende Weiche:** Anhand des Dateisystems wird bestimmt, ob eine Neuimplementierung oder eine Erweiterung erforderlich ist.

1.  **Bestandsaufnahme:**
    a. Leite den Modulnamen aus deinem aktuellen Ziel ab (z.B. "Themed Components / Alert-Dialog" -> `alert_dialog`; "Badge" -> `badge`).
    b. **Prüfe, ob ein Verzeichnis mit diesem Namen bereits in `src/components/` existiert.**

2.  **Führe basierend auf der Prüfung einen der folgenden Schritte aus:**

    *   **Fall A: Das Modul existiert NICHT (Neuimplementierung)**
        1.  **Anatomie definieren:** Zerlege die Komponente gemäß den Quellen in ihre logischen Teile (z.B. `Badge.Root`). Jeder Teil wird eine eigene Rust-Struktur, die `src/components/TEMPLATE.md` folgt.
        2.  **Komponente implementieren:** Erstelle das Rust-Modul (z.B. `src/components/badge/`). Implementiere die **volle Funktionalität**, wie sie in den Quellen für dein Ziel definiert ist (inklusive aller Props, Varianten, Styling-Logik etc.).
        3.  **Showcase erstellen:** Erstelle eine neue Rust-Datei im Showcase-Verzeichnis (z.B. `src/showcase/badge.rs`), um die Kernfunktionalität und die verschiedenen Varianten der neuen Komponente zu demonstrieren.

    *   **Fall B: Das Modul existiert BEREITS (Erweiterung)**
        1.  **NICHT ÜBERSCHREIBEN:** Deine Aufgabe ist es, den bestehenden Code im vorhandenen Modul (z.B. `src/components/switch/`) zu **erweitern/verbessern**, nicht ihn zu ersetzen oder zu duplizieren.
        2.  **Differenzanalyse durchführen (KRITISCHER SCHRITT):**
            *   Lade den existierenden Rust-Code aus dem Modul.
            *   Studiere die Quelldateien (`.props.ts`, `.css`, `.tsx`, ...), die in der `MASTER_TASK_LIST.md` für dein **aktuelles Ziel** verlinkt sind.
            *   Vergleiche die Logik dieser Version akribisch mit dem bereits vorhandenen Rust-Code.
            *   **Identifiziere und liste alle Unterschiede auf:** neue Props (`variant`, `size`, `color`, `highContrast`...), geändertes Verhalten, zusätzliche CSS-Klassen, die in Bevy-Logik übersetzt werden müssen. Ziel ist es, die bestehende Komponente so zu modifizieren, dass sie dem Funktionsumfang der Radix-UI entspricht.
        3.  **Code modifizieren:** Integriere die identifizierten Änderungen in die **bestehenden** Rust-Codebase. Das bedeutet Beispeilsweise:
            *   Felder zu bestehenden Structs hinzufügen.
            *   Neue Marker-Komponenten oder Systeme für das erweiterte Styling implementieren.
            *   Bestehende Logik anpassen, um die neuen Props zu verarbeiten.
        4.  **Showcase anpassen:** Erweitere die zugehörige Showcase-Datei (z.B. `src/showcase/switch.rs`), um explizit die **neuen Funktionalitäten** und -Varianten zu demonstrieren.

#### **Schritt C: Validierung & Abschluss**

1.  **Projekt prüfen:** Stelle sicher, dass das gesamte Projekt nach deinen Änderungen mit `cargo check` ohne Fehler oder Warnungen kompiliert.
2.  **Ziel abschließen:** Öffne `MASTER_TASK_LIST.md` und markiere den Eintrag für dein **aktuelles Ziel** mit `- [x]`.
3.  Wenn du in einem rekursiven Aufruf warst, kehre nun zum pausierten, übergeordneten Ziel zurück und fahre dort fort.

**Wenn dein ursprüngliches Hauptziel in der `MASTER_TASK_LIST.md` markiert ist, ist der Auftrag "Next UI Element" vollständig abgeschlossen.**