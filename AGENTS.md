# Hinweise für Agenten

* Das Projekt ist in Rust geschrieben und verwendet bevy 0.16 als Game Engine. Damit die Implementierung der Componenten fehlerfrei funktionieren ist gewünscht für die gefordereten angewandten Mechanismen die Bevy-API Dokuemntation unter [docs.rs/bevy](https://docs.rs/bevy/0.16.0/bevy/) als Referenz zu nutzen. Die Bevy-API Dokumentation ist sehr umfangreich und bietet viele Beispiele, die dir helfen können, die Mechanismen zu verstehen und korrekt anzuwenden.

* Wenn du eine neue Componenten für die code base erstellst, gibt es einen Template-Datei an der du dich orierntieren sollst, damit gewährleistet ist dass die Componenten in der code base einheitlich sind. Die Template-Datei findest du unter `src/components/TEMPLATE.md`.

* Wenn du Änderungen an den bestehenden Componenten vornimmst, achte darauf, dass du die Template-Datei berücksichtigst, damit die Einheitlichkeit der Componenten gewahrt bleibt.
  
* Bei Änderungen oder neuen Componenten wird von dir ebenfalls erwartet, dass du die Documentation der Componenten aktualisierst die du geändert hast oder neu erstellt hast. Die Documentation für bei den bestehenden Compoenten in klassischer Rust-Documentation geschrieben.

* Optional (falls du noch Kapazität hast) kannst du auch noch abschließend in den Ordner `src/showcase/` die Komponente aktualisieren oder implementieren, die du erstellt oder aktualisiert hast. Die Showcase-Componenten sind dafür da, dass die Componenten in Aktion gesehen werden können. Es ist nicht zwingend notwendig, aber es wäre schön, wenn du es machen könntest. Dann kann ich schneller sehen ob alles wie gewünscht funktioniert.
