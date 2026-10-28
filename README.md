# adhs-expert

Erzeugt ein PDF mit allen Beiträgen von [adhs.expert](https://adhs.expert),
deren Text jetzt vollständig im Beitrag selbst steht.

## Hintergrund

Viele Beiträge bestanden nur aus einem Audio-Player und einem nackten Link auf
ein PDF. Für Suchmaschinen waren das praktisch leere Seiten — der gesamte
Inhalt lag im PDF, nicht im Beitrag. Der Text wurde 1:1 aus den PDFs in die
Beiträge übernommen; vier PDFs waren Scans ohne Textebene und mussten zuerst
per OCR gelesen werden. Diese Fälle sind im erzeugten PDF eigens ausgewiesen.
Ein fünfter Scan wurde ebenfalls erkannt, sein Beitrag liegt inzwischen aber im
Papierkorb und zählt darum nicht mehr mit.

Dasselbe gilt für gelöschte Beiträge allgemein: was auf adhs.expert im
Papierkorb liegt, gehört aus `src/eintraege.rs` heraus, sonst zeigt das PDF auf
eine 404-Seite. Im September 2026 traf das drei Beiträge:

- «Perfektionismus und Prokrastination» — kein Vortrag, sondern ein
  Teilnehmeraufruf von 2015 für eine Studie der Universität Würzburg.
- «Jeder Hundertste erkrankt an Schizophrenie» — dasselbe Interview auf
  derselben Zeitungsseite wie «Emotionale Monsterwelle». Der behaltene Beitrag
  trug das falsche Datum und liegt jetzt auf dem 29. März 2015; weil die
  Permalinks datumsbasiert sind, hat er dadurch eine neue Adresse.
- «Psychotherapie und Coaching» — nicht ihr Text, sondern die
  Arbeitsbeschreibung einer Kollegin samt Kontaktdaten.

Im Oktober 2026 kamen zwei weitere dazu, beide aus demselben Grund — fremde
Kontaktdaten in einem Text, der nicht von ihr stammt:

- «Ronni Rocket – Wie das ADHS siegt und wie es scheitert» — kein Vortrag,
  sondern das Organisationsblatt einer Matinée im Kino Odeon Brugg: Zeitplan,
  Namensetiketten, Spesenrückerstattung, mögliche Podiumsfragen, dazu drei
  Mailadressen dritter Personen im Klartext.
- Ein zweiter Beitrag mit dem Praxisprofil einer Kollegin — wie
  «Psychotherapie und Coaching» nicht ihr Text, sondern eine fremde
  Arbeitsbeschreibung mit Praxisadresse, Mailadresse und Handynummer.

Vier weitere sind am 23. Oktober 2026 als Dubletten in den Papierkorb gewandert.
Ein Vergleich der Transkripte aller 1170 Beiträge über ihre Prüfsumme ergab vier
Gruppen mit zeichengleichem Text, alle aus dem Jahr 2021, als eine Reihe im
Oktober ein zweites Mal als «Podcast:» veröffentlicht wurde. Behalten wurde
jeweils der Beitrag mit dem besseren Titel und Lead:

- «Prävention durch richtig Handeln im kritischen Augenblick» vom 14.9.2021 —
  dieselbe Weiterbildung lag dreifach vor. Der Beitrag vom 15.3.2021 ist
  geblieben, obwohl auch sein Text doppelt ist: Er trägt die unbearbeitete
  Aufnahme, der behaltene vom 10.5.2021 die von Shelter Schweiz bearbeitete.
- «Was braucht es für die Ausbildung unserer nächsten Generation» vom 5.10.2021
- «Podcast: Welche Schulen brauchen die Kinder von heute?»
- «Podcast: Woher kommt die Schizophrenie?»

Beim Durchgehen aller Einträge unter 1200 Wörtern im Oktober 2026 zeigte sich
umgekehrt, dass eine kleine Wortzahl meist nicht Leere bedeutet, sondern einen
Beitrag, dessen Volltext nie übernommen wurde: Im Beitrag steht nur die
Einladung, das Abstract oder das Tagungsprogramm, während der Vortrag als Audio
oder als Transkript-PDF daneben liegt. Bei «Jugendliche zwischen Stuhl und Bank
beim Berufseinstieg» ist das am 23. Oktober 2026 nachgeholt worden — aus 275
wurden 8220 Wörter. Dort stand unter der Überschrift «Transkript» bis dahin das
Workshop-Handout statt des Transkripts; es heisst jetzt «Unterlagen zum
Workshop». Am selben Tag sind drei weitere nachgetragen worden: «Spannungsfeld
zwischen Psychologie, Religion und sozialer Verantwortung» (209 → 9541 Wörter),
«Rolle der Frau in der heutigen Gesellschaft» (102 → 8982) und «Bindung – was
die Welt zusammenhält» (144 → 4547). Die Transkripte lagen in allen Fällen
bereits als PDF im Beitrag verlinkt oder auf ganglion.ch bereit; dort führt
`https://ganglion.ch/pdf/<name>.pdf` direkt auf die Datei.

Bei drei Beiträgen ist nichts zu holen: Das PDF zum Jubiläumsanlass von VASK
Schweiz ist ein Bildscan ohne Textebene, zur Diplomfeier der Mütterberaterinnen
gibt es nur das Handout, das schon im Beitrag steht, und «Gewalt in Familie und
Beziehungen» ist eine Kursausschreibung ohne Vortragsaufnahme — kurz, aber
vollständig.

Zwölf weitere Beiträge trugen den Spaltensatz einer Zeitungs- oder
Zeitschriftenseite: `pdftotext -layout` hatte die Nachbarspalten mitgezogen, in
fünf Fällen samt fremder Artikel und Inserate. Sie wurden im September 2026
spaltenweise neu aufgebaut; die Wortzahlen in `src/eintraege.rs` sind
entsprechend kleiner geworden.

Zwei Beiträge trugen einen anderen Schaden: Ihre Quell-PDFs stammen von macOS
Quartz, dessen Ligaturglyphen (ff, fi, fl, ffi, ffl) keine brauchbare Zuordnung
tragen. Beim Auslesen war an ihrer Stelle ein Leerzeichen geblieben — «Begri »
statt «Begriff», «häu g» statt «häufig» —, und die Glyphen selbst standen als
verwaiste Zeilen im Text. Auch das ist im September 2026 behoben worden.

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

## Vorträge scannen

`scannen` holt Papiervorlagen über den automatischen Einzug des Epson GT-2500
und legt ein PDF an, das Datum und Titel des Vortrags trägt — im Dateinamen
(`2014-03-05 Titel.pdf`) und in den Dokumenteigenschaften.

```bash
brew install sane-backends
cargo run --release --bin scannen -- --ordner ~/Documents
cargo run --release --bin scannen -- --datum 5.3.2014 --titel "Titel" --ordner ~/Documents
```

Gescannt wird einseitig, A4, 300 dpi in Graustufen (`--farbe` für Farbe,
`--dpi` für eine andere Auflösung). Datum und Titel liest die Texterkennung von
macOS (Vision) aus den Seiten: der Titel ist der Kopf der ersten Seite bis zur
Autorenzeile, das Datum das erste im Kopf der ersten Seite, sonst das letzte im
Dokument. `--datum` und `--titel` überschreiben die Erkennung; fehlt ein Datum,
heisst die Datei «ohne Datum …». Dieselbe Erkennung legt eine unsichtbare
Textebene über die Seitenbilder, damit sich das PDF markieren und durchsuchen
lässt, und macht erkannte Adressen (`www.…`, `http…`, Mailadressen) anklickbar.
Die Erkennung kann sich verlesen, ein Link kann darum auch einmal falsch zeigen.
`--lesen BILD…` zeigt nur, was erkannt wird,
`--aus BILD…` baut das PDF aus schon gescannten JPEGs.

Der Scanner läuft über SANE (Backend `epson2`, dort als ES-H300 geführt), nicht
über Epson Scan 2: Dessen Argument `-SCAN` ist nicht dokumentiert, und der
Image-Capture-Treiber «EPSON Scanner.app» ist nur für Intel gebaut und startet
auf Apple Silicon ohne Rosetta nicht. Immer im Batch-Modus scannen — ein
Einzelscan lässt den Einzug das nächste Blatt vorladen und ungescannt auswerfen.

## Aufbau

| Datei | Zweck |
|---|---|
| `src/uebersicht.rs` | Satz des PDFs und Link-Overlay |
| `src/scannen.rs` | Vorträge über den Einzug des GT-2500 scannen, Datum und Titel erkennen |
| `src/eintraege.rs` | generierte Daten: 150 Beiträge mit Aufrufzahlen, Feld `ocr` markiert die OCR-Fälle |
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
