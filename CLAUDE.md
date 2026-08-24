# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Erzeugt ein PDF, das alle Beiträge von [adhs.expert](https://adhs.expert)
auflistet, deren Text jetzt vollständig im Beitrag selbst steht.

Der Anlass: viele Beiträge bestanden nur aus einem Audio-Player und einem
nackten Link auf ein PDF – für Suchmaschinen praktisch leere Seiten. Der
gesamte Inhalt lag im PDF, nicht im Beitrag. Der Text wurde 1:1 aus den PDFs
in die Beiträge übernommen; fünf Quellen waren Scans ohne Textebene.

Kommentare, Dokumententexte und Commit-Messages sind auf Deutsch
(Schweizer Rechtschreibung: **ss statt ß**).

## Build und Ausführung

```bash
cargo run --release --bin uebersicht
cargo run --release --bin uebersicht -- --out /pfad/zum.pdf
```

Schriftverzeichnis über `$FONT_DIR` überschreibbar (Vorgabe: `./fonts`).
Der Build läuft offline, sofern `genpdf 0.2` und `lopdf 0.34` im
Cargo-Cache liegen.

## Architektur

### PDF-Erzeugung (`src/uebersicht.rs`)

Pure Rust, kein Chrome: `genpdf` schreibt über `printpdf`, die
DejaVu-Sans-Familie wird eingebettet. Dieselbe Pipeline wie in
[fundaziun-davaz](https://github.com/zdavatz/fundaziun-davaz).

**Hyperlinks sind der heikle Teil.** `genpdf` 0.2 kennt keine. Deshalb:

- URL-Zeilen sind die **einzigen** Zeilen des Dokuments in `LINK_FONT_SIZE`
  (8 pt). Nach dem Rendern läuft `add_links()` den Inhaltsstrom Seite für
  Seite durch, sammelt die Grundlinien aller `TJ`/`Tj` bei genau dieser
  Schriftgrösse ein und legt darüber `/Link`-Annotationen mit `lopdf`.
- Sobald irgendwo sonst 8 pt auftaucht, verschiebt sich die Zuordnung und
  jeder Link zeigt aufs falsche Ziel. Kopfzeile läuft deshalb auf 7 pt,
  die Metazeilen auf 9 pt, der Fliesstext auf 10 pt.
- `render()` bricht mit Fehler ab, wenn die Zahl der gefundenen 8-pt-Zeilen
  nicht der Zahl der URLs entspricht. Diese Assertion nie entfernen – sie
  ist die einzige Absicherung gegen still verschobene Links.
- `urls()` muss dieselbe Reihenfolge liefern wie der Satz: erst der
  OCR-Abschnitt, dann die vollständige Liste. Wer die Reihenfolge der
  Abschnitte in `render()` ändert, muss `urls()` mitziehen.
- `link_text()` kürzt lange Adressen in der Mitte; die Klickfläche wird nach
  der **angezeigten** Länge bemessen, verlinkt wird immer das Original.
  `MAX_LINK_CHARS = 88` garantiert, dass eine URL nie umbricht – ein Umbruch
  ergäbe zwei 8-pt-Zeilen und damit wieder eine Verschiebung.

**Seitenumbruch ist der zweite heikle Teil.** `genpdf` 0.2 kennt kein
"keep together": ein Eintrag, der nicht mehr auf die Seite passt, wird
umbrochen - die Kopfzeile steht dann mitten im Titel. `messen()` bestimmt
deshalb die Hoehe jedes Eintrags vorab, `push_liste()` setzt den Umbruch
selbst. Zwei Fallstricke, die dabei Zeit gekostet haben:

- Die Messstile muessen denselben `line_spacing` tragen wie das Dokument
  (`ZEILENABSTAND`). Sonst weicht die Schaetzung um Faktor 1.35 ab.
- `PageBreak` bricht **bedingungslos** um. Faellt unser Umbruch mit dem von
  genpdf zusammen, brechen beide hintereinander und es entsteht eine leere
  Seite. Die geschaetzte Kapazitaet haelt deshalb die Hoehe des groessten
  Eintrags frei, damit wir zuverlaessig *vorher* umbrechen.
- Nach dem letzten Eintrag darf kein `Break` mehr kommen - er erzwingt sonst
  eine leere Schlussseite.

`MASS_DEBUG=1` gibt die gemessenen Hoehen auf stderr aus.

### Daten (`src/eintraege.rs`)

Generiert, nicht von Hand gepflegt. Ein Eintrag je Beitrag mit `datum`,
`titel`, `url`, `woerter`, `aufrufe` und `ocr`. Sortiert nach `aufrufe`,
meistgelesene zuerst. Das Feld `ocr` markiert die Fälle, deren
Quelle ein Scan ohne Textebene war; sie erscheinen im PDF an drei Stellen:
in den Kennzahlen der Titelseite, in einem eigenen Abschnitt und als Marke
in der Metazeile des Listeneintrags.

Beim Neuerzeugen der Datei die Reihenfolge beibehalten (nach `aufrufe`
absteigend) und `ocr` korrekt setzen – sonst stimmt die Zählung auf der Titelseite nicht.

## Fachliches, das im Code nicht steht

Die Website läuft auf **WordPress.com Atomic** mit Jetpack. Wer die Beiträge
über die REST-API bearbeitet, sollte folgendes wissen:

- Plugins tragen dort `is_managed=True` und lassen sich über
  `/wp-json/wp/v2/plugins/<slug>` **nicht** schalten – der Endpoint liefert
  404 `rest_plugin_not_found`. Manche Plugins bieten stattdessen einen
  eigenen Options-Endpoint (AMP etwa `/wp-json/amp/v1/options`).
- Anwendungspasswörter authentifizieren nur REST und XML-RPC. `wp-admin`
  leitet damit auf den Login um; Plugin-Einstellungen, die keine REST-Route
  haben (Yoast etwa), gehen nur über die Oberfläche.
- WordPress' `wptexturize()` macht beim Ausliefern aus ` - ` ein ` – `. Der
  gespeicherte Inhalt bleibt unverändert, nur die Anzeige weicht ab.

**Messdaten:** Auf adhs.expert ist **kein Google Analytics** eingebunden -
kein `gtag`, kein Tag Manager, keine Mess-ID im Quelltext. Wer nach
Reichweite sortieren will, nimmt **Jetpack Stats**, erreichbar mit demselben
Anwendungspasswort:

- `/wp-json/jetpack/v4/stats-app/sites/<id>/stats/top-posts?period=year&num=20`
  liefert Jahresranglisten, ist aber bei 500 Zeilen je Jahr gekappt.
- `/wp-json/jetpack/v4/stats-app/sites/<id>/stats/post/<post-id>` liefert im
  Feld `views` den exakten Gesamtwert seit Beginn - fuer eine feste Menge von
  Beitraegen die verlaessliche Quelle.

Zur Textgewinnung aus den PDFs:

- **`pdftotext -layout` mit `<br />` je Zeile taugt nur für Transkripte mit
  Zeitmarken.** Bei mehrspaltig gesetzten Dokumenten schleppt es den
  Spaltensatz mit und erzeugt unlesbare Umbrüche mitten im Satz.
- PDFs brechen lange URLs am Bindestrich um. Diese Umbrüche sind Layout,
  nicht Text – ohne Zusammenfügen entstehen zerrissene, tote Links.
- **OCR-Ausgabe nie ungeprüft übernehmen.** Tesseract verlas unter anderem
  `http://schizo.li` zu `httpz//schizo.lij` und `ganglion.ch` zu
  `qanqlion.ch`, erkannte handschriftliche Stiftmarkierungen als Zeichen und
  produzierte bei grafisch gesetzten Plakaten fast nur Müll. Verlässlich ist
  nur: Scan mit `pdftoppm -png` rendern, das Bild selbst lesen, transkribieren.

## Vertraulichkeit

**Dieses Repository ist öffentlich.** Ausserhalb bleiben und in `.gitignore`
geführt:

- `send_uebersicht.py` – trägt die Mailadressen der Familie im Klartext.
  Gleiche Regel wie `send_mail*.py` in `fundaziun-davaz`: das Werkzeug bleibt
  lokal, nicht weil der Code geheim wäre, sondern weil die eingebetteten
  Daten es sind.
- `credentials`, `token*.json`, `client_secret_*.json`, `*.env`
- das erzeugte `adhs_expert_beitraege_mit_volltext.pdf` (reproduzierbar)
- `target/`

Ein `.gitignore`-Eintrag ist eine Vorsichtsmassnahme, kein Schutz – ein
`git add -f` genügt. Vor jedem Commit `git status` prüfen; niemals
Zugangsdaten, Anwendungspasswörter oder private Mailadressen in eingecheckte
Dateien schreiben.

## Lizenz

GPL-3.0. Neue Quelldateien tragen einen GPL-3.0-verträglichen Kopf, und jede
Abhängigkeit muss mit GPL-3.0 vereinbar sein.
