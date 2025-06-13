# Hinweise für Agenten - Hauptdatei

## 🎯 Projektziel

Das Projekt ist in Rust geschrieben und verwendet Bevy 0.16 als Game Engine. Ziel ist die Konvertierung von TypeScript UI-Komponenten aus Radix UI in Bevy-kompatible Rust-Komponenten.

## Referenzen und Dokumentation
* Bevy-API Dokumentation: [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) - Primäre Referenz für alle Bevy-Mechanismen
* Radix UI GitHub: [github.com/radix-ui/](https://github.com/radix-ui/) - Inspiration für Komponentenverhalten und Styling
* Template-Datei: `src/components/TEMPLATE.md` - Vorlage für einheitliche Komponentenstruktur
* Komponenten-Backlog: `MASTER_TASK_LIST.md` - Deine Quelle für neue Aufgaben.
* Komponenten-Registry: `REGISTRY.md` - Deine Wissensdatenbank über abgeschlossene Komponenten.

---

## ⚙️ Der dynamische Arbeits-Algorithmus (Die Goldene Regel)

Deine Arbeit wird durch den Befehl **"Next UI Element"** ausgelöst. Daraufhin führst du den folgenden, zweiteiligen Prozess aus.

### **Teil 1: Finde dein Hauptziel**

1.  **Öffne `MASTER_TASK_LIST.md`**.
2.  **Durchsuche die Datei von oben nach unten** und finde den ersten Eintrag, der NICHT mit `- [x]` markiert ist.
3.  **Dieses Element ist dein neues Hauptziel** (z.B. "Accessible Icon").
4.  **Starte TEIL 2** dieses Algorithmus mit diesem Hauptziel.

### **Teil 2: Der rekursive Implementierungs-Prozess**

Dieser Prozess wird mit einem **aktuellen Ziel** (z.B. "Accordion") gestartet.

#### **Schritt A: Analyse**
1.  **Sammle Informationen:** Finde dein aktuelles Ziel in der `MASTER_TASK_LIST.md`, um die URLs zur Dokumentation und den Quelldateien zu erhalten. Studiere diese Quellen.
2.  **Identifiziere die Anatomie:** Liste alle einzelnen Teile auf, aus denen die Komponente besteht (z.B. `Accordion.Root`, `Accordion.Item`, `Accordion.Trigger`). Jeder dieser Teile wird eine eigene Rust-Struktur, die dem `src/components/TEMPLATE.md` folgt.
3.  **Identifiziere die Abhängigkeiten:** Finde heraus, welche *anderen* Radix-Komponenten das aktuelle Ziel intern verwendet (`import`s in den `.tsx`-Dateien). Zum Beispiel importiert `AlertDialog` die Komponente `Dialog`.

#### **Schritt B: Abhängigkeits-Prüfung**
1.  **Prüfe JEDE Abhängigkeit** (aus Schritt A.3) gegen die `REGISTRY.md`-Datei. Die `REGISTRY.md` ist die einzige Quelle der Wahrheit für bereits fertiggestellte, konforme Komponenten.
2.  **Sind alle Abhängigkeiten im Registry vorhanden?**
    *   **JA:** Perfekt. Fahre fort mit **Schritt C: Implementierung & Showcase**.
    *   **NEIN:** Eine oder mehrere Abhängigkeiten fehlen. Du musst diesen Algorithmus (Teil 2) nun rekursiv anwenden:
        1.  **Pausiere die Arbeit** am aktuellen Ziel.
        2.  **Nimm die erste fehlende Abhängigkeit als dein NEUES aktuelles Ziel.**
        3.  **Starte Teil 2 von vorne (Schritt A)** mit der Abhängigkeit als Ziel.
        4.  Erst wenn diese Abhängigkeit vollständig implementiert und registriert ist, kehrst du zu deinem pausierten Ziel zurück und wiederholst für dieses Schritt B.
        
#### **Schritt C: Implementierung oder Erweiterung & Showcase**
Sobald alle Abhängigkeiten deines aktuellen Ziels erfüllt sind:

1.  **Bestandsaufnahme:** Prüfe, ob für dein aktuelles Ziel (z.B. `Switch`) bereits ein Komponenten-Modul im Verzeichnis `src/components/` existiert (z.B. `src/components/switch/`).

2.  **Führe einen der folgenden Schritte aus:**

    *   **Fall A: Das Modul existiert NICHT (Neuimplementierung).**
        1.  **Komponente implementieren:** Erstelle das Rust-Modul (z.B. `src/components/switch/`). Implementiere jeden anatomischen Teil strikt nach den Vorgaben aus `TEMPLATE.md` und den Quellen aus der `MASTER_TASK_LIST.md`.
        2.  **Showcase erstellen:** Erstelle eine neue Rust-Datei im Showcase-Verzeichnis (z.B. `src/showcase/switch.rs`), um die grundlegende Funktionalität der Komponente zu demonstrieren.

    *   **Fall B: Das Modul existiert BEREITS (Erweiterung).**
        1.  **NICHT ÜBERSCHREIBEN:** Deine Aufgabe ist es, den bestehenden Code zu **erweitern**, nicht ihn zu ersetzen oder eine Kopie anzulegen (z.B. KEIN `switch_themed.rs`).
        2.  **Analyse der Erweiterung:** Studiere die Quellen aus `MASTER_TASK_LIST.md` die für dein Ziel angegeben sind und vergleiche sie mit den Code des bereits implementerierten Elements. Identifiziere die Unterschiede: neue Props, geändertes Verhalten, zusätzliche CSS-Klassen oder Styling-Logik usw.
        3.  **Code modifizieren:** Lade die existierenden Rust-Dateien (z.B. `src/components/switch/mod.rs`) und integriere die identifizierten Erweiterungen. Das kann bedeuten:
            *   Felder zu bestehenden Structs hinzufügen.
            *   Neue Marker-Komponenten oder Systeme implementieren.
            *   Bestehende Logik anpassen, um neue Varianten oder Props zu unterstützen.
        4.  **Showcase anpassen:** Erweitere die zugehörige Showcase-Datei (z.B. `src/showcase/switch.rs`), um die neuen, "themed" Funktionalitäten und Varianten zu demonstrieren.

3.  **Gesamtprojekt prüfen:** Stelle sicher, dass das gesamte Projekt (inklusive der neuen oder modifizierten Komponente und des Showcases) mit `cargo check` ohne Fehler und Warnungen durchläuft.

#### **Schritt D: Registrierung & Abschluss**
1.  **Registriere die Komponente:** Falls die Komponente *neu* war, öffne `REGISTRY.md` und füge den Namen des Komponenten-Moduls hinzu (z.B. `- [x] switch`). Der Eintrag erfolgt alphabetisch. Falls die Komponente nur erweitert wurde, ist dieser Schritt nicht nötig, da sie bereits registriert sein sollte.
2.  **Schließe das Ziel ab:** Öffne `MASTER_TASK_LIST.md` und markiere den Eintrag für dein **aktuelles Ziel** (egal ob neu oder erweitert) mit `- [x]`.
3.  Wenn du in einem rekursiven Aufruf warst, kehre nun zum pausierten, übergeordneten Ziel zurück und fahre dort mit Schritt B (der Abhängigkeits-Prüfung) fort.

**Wenn dein Hauptziel in der `MASTER_TASK_LIST.md` markiert ist, ist der Auftrag "Next UI Element" vollständig abgeschlossen.**
