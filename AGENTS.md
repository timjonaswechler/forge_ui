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
2.  **Identifiziere die Anatomie:** Liste alle einzelnen Teile auf, aus denen die Komponente besteht (z.B. `Accordion.Root`, `Accordion.Item`, `Accordion.Trigger`). Jeder dieser Teile wird eine eigene Rust-Struktur, die dem `TEMPLATE.md` folgt.
3.  **Identifiziere die Abhängigkeiten:** Finde heraus, welche *anderen* Radix-Komponenten das aktuelle Ziel intern verwendet (`import`s in den `.tsx`-Dateien). Zum Beispiel importiert `AlertDialog` die Komponente `Dialog`.

#### **Schritt B: Abhängigkeits-Prüfung**
1.  **Prüfe JEDE Abhängigkeit** (aus Schritt A.3) gegen die `REGISTRY.md`-Datei. Die `REGISTRY.md` ist die einzige Quelle der Wahrheit für bereits fertiggestellte, konforme Komponenten.
2.  **Sind alle Abhängigkeiten im Registry vorhanden?**
    *   **JA:** Perfekt. Fahre fort mit **Schritt C: Implementierung**.
    *   **NEIN:** Eine oder mehrere Abhängigkeiten fehlen. Du musst diesen Algorithmus (Teil 2) nun rekursiv anwenden:
        1.  **Pausiere die Arbeit** am aktuellen Ziel.
        2.  **Nimm die erste fehlende Abhängigkeit als dein NEUES aktuelles Ziel.**
        3.  **Starte Teil 2 von vorne (Schritt A)** mit der Abhängigkeit als Ziel.
        4.  Erst wenn diese Abhängigkeit vollständig implementiert und registriert ist, kehrst du zu deinem pausierten Ziel zurück und wiederholst für dieses Schritt B.

#### **Schritt C: Implementierung**
Sobald alle Abhängigkeiten deines aktuellen Ziels erfüllt sind:
1.  **Erstelle/überschreibe das Rust-Modul** (z.B. `src/components/accordion/`). Bestehender Code wird ignoriert, da er als nicht-konform gilt und von dir neu geschrieben werden muss.
2.  **Implementiere jeden anatomischen Teil** nach den Vorgaben aus `TEMPLATE.md`.
3.  **Prüfe deinen Code:** Stelle sicher, dass `cargo check` ohne Fehler durchläuft.

#### **Schritt D: Registrierung & Abschluss**
1.  **Öffne `REGISTRY.md`** und füge den Namen des gerade fertiggestellten Komponenten-Moduls hinzu (z.B. `- [x] accordion`). Der Eintrag erfolgt alphabetisch.
2.  **Dies beendet die Bearbeitung deines aktuellen Ziels.** Wenn dies ein rekursiver Aufruf war (also eine Abhängigkeit), kehre nun zu deinem übergeordneten, pausierten Ziel zurück.
3.  **Ist dein ursprüngliches Hauptziel** (aus Teil 1) abgeschlossen, gehe zu `MASTER_TASK_LIST.md` und markiere den entsprechenden Eintrag mit `- [x]`.

**Damit ist der Auftrag "Next UI Element" vollständig abgeschlossen.**