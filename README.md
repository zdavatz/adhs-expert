# adhs-expert

Erzeugt ein PDF mit allen Beiträgen von [adhs.expert](https://adhs.expert),
deren Text jetzt vollständig im Beitrag selbst steht.

## Hintergrund

Viele Beiträge bestanden nur aus einem Audio-Player und einem nackten Link auf
ein PDF. Für Suchmaschinen waren das praktisch leere Seiten — der gesamte
Inhalt lag im PDF, nicht im Beitrag. Der Text wurde 1:1 aus den PDFs in die
Beiträge übernommen; fünf PDFs waren Scans ohne Textebene und mussten zuerst
per OCR gelesen werden. Diese Fälle sind im erzeugten PDF eigens ausgewiesen.

## Bauen und Ausführen

```bash
cargo run --release --bin uebersicht
cargo run --release --bin uebersicht -- --out /pfad/zum.pdf
```

Das Schriftverzeichnis ist über `$FONT_DIR` überschreibbar (Vorgabe: `./fonts`).
`MASS_DEBUG=1` gibt die gemessenen Zeilenhöhen aus.

Die Liste ist nach Aufrufen sortiert, meistgelesene zuerst. Grundlage sind die
Zahlen von Jetpack Stats über den gesamten erfassten Zeitraum seit 2010 —
Google Analytics ist auf adhs.expert nicht eingebunden.

## Aufbau

| Datei | Zweck |
|---|---|
| `src/uebersicht.rs` | Satz des PDFs und Link-Overlay |
| `src/eintraege.rs` | generierte Daten: 159 Beiträge mit Aufrufzahlen, Feld `ocr` markiert die OCR-Fälle |
| `fonts/` | DejaVu Sans, wird ins PDF eingebettet |

## Hyperlinks

`genpdf` 0.2 kennt keine Hyperlinks. Deshalb werden die URL-Zeilen als einzige
Zeilen des Dokuments in 8 pt gesetzt; nach dem Rendern legt `add_links()` mit
`lopdf` über jede solche Zeile eine Link-Annotation. Sobald irgendwo sonst im
Satz diese Schriftgrösse auftaucht, verschiebt sich die Zuordnung — Kopfzeile
läuft deshalb auf 7 pt, die Metazeilen auf 9 pt. Eine Assertion bricht ab,
wenn Zeilen- und URL-Zahl auseinanderlaufen.

Dieselbe Pipeline wie in [fundaziun-davaz](https://github.com/zdavatz/fundaziun-davaz).

## Lizenz

GPL-3.0
