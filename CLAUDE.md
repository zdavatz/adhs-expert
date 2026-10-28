# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Erzeugt ein PDF, das alle Beiträge von [adhs.expert](https://adhs.expert)
auflistet, deren Text jetzt vollständig im Beitrag selbst steht. Ein zweites
Programm, `scannen`, digitalisiert Papiervorlagen von Vorträgen über den
Einzug des Epson GT-2500 nach `scan/`.

Der Anlass: viele Beiträge bestanden nur aus einem Audio-Player und einem
nackten Link auf ein PDF – für Suchmaschinen praktisch leere Seiten. Der
gesamte Inhalt lag im PDF, nicht im Beitrag. Der Text wurde 1:1 aus den PDFs
in die Beiträge übernommen; vier Quellen waren Scans ohne Textebene, eine
weitere ein PDF ohne verwertbare Textebene (siehe unten).

Kommentare, Dokumententexte und Commit-Messages sind auf Deutsch
(Schweizer Rechtschreibung: **ss statt ß**).

## Build und Ausführung

```bash
cargo run --release --bin uebersicht
cargo run --release --bin uebersicht -- --out /pfad/zum.pdf
```

```bash
brew install sane-backends tesseract tesseract-lang   # einmalig, für scannen
cargo run --release --bin scannen   # Stapel aus dem Einzug nach scan/
cargo run --release --bin scannen -- --aus bild1.jpg bild2.jpg   # ohne Scanner
cargo run --release --bin scannen -- --lesen bild1.jpg           # nur Erkennung
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

### Scannen (`src/scannen.rs`)

Ablauf: `scanimage --batch` (SANE, Backend `epson2`) holt den Stapel als JPEG,
Tesseract (`deu`, TSV-Ausgabe mit Wortkästen) liest jede Seite, der Stapel
wird in Vorträge geteilt,
Kontaktdaten werden aus Bild und Text entfernt, `lopdf` setzt je Vortrag ein
PDF mit Bild (DCTDecode, unverändert), unsichtbarer Textebene (Rendermodus 3,
Helvetica, per `Tz` auf die Kastenbreite gezogen) und Link-Annotationen.
Benutzerwünsche, die im Code stecken: Dateiname `Titel_TT.MM.JJJJ.pdf`,
Unterstriche, Datum am Schluss; 150 dpi Graustufen; ohne Datum im Text der
Scantag; jeder Scan wird committet **und** gepusht.

Was dabei Zeit gekostet hat:

- **Epson Scan 2 ist nicht skriptbar.** `-SCAN` existiert, das Format ist
  undokumentiert. Der Image-Capture-Treiber `/Library/Image Capture/Devices/
  EPSON Scanner.app` ist x86_64/i386 – ohne Rosetta (auf diesem Mac nicht
  installiert, ab macOS 28 ganz weg) startet ihn `icdd` nicht, ImageCaptureCore
  findet das Gerät, die Sitzung läuft nach 30 s in einen Timeout. SANE ist der
  Weg; dort heisst der GT-2500 «ES-H300».
- **Zwei Epson-Scanner.** `scanimage -L` meldet zeitweise auch
  `epson2:net:192.168.0.163` (ein Multifunktionsgerät im Netz), und zwar
  **zuerst**. Wer das erste `epson2:`-Gerät nimmt, scannt ins Leere und bekommt
  `Error during device I/O`. `epson_geraet()` wählt nach Modell, dann USB.
  Beim Aufruf von `scanimage` von Hand dasselbe beachten.
- **Nie ohne `--batch` scannen.** Ein Einzelscan (`-o`) lässt den Einzug das
  nächste Blatt vorladen und ungescannt auswerfen – der Benutzer muss den
  Stapel neu ordnen.
- **Papierstau mitten im Stapel:** `scanimage` endet mit Status 9, die bis
  dahin fertigen Seiten liegen da, das gestaute Blatt fehlt. Nach dem
  Beheben ab dem gestauten Blatt neu einlegen, den Rest scannen und mit
  `--aus` zusammensetzen. Hat der Vortrag Seitenzahlen, meldet
  `vortraege_trennen()` die Lücke; ohne Seitenzahlen nicht – dann den Benutzer
  nach der Seitenzahl fragen. Der erste Anlauf nach einem Stau meldet
  manchmal noch einmal einen Stau (ADF-Status `0xe4`), der zweite geht.
  `SANE_DEBUG_EPSON2=5` zeigt den ADF-Status (`0x80` bereit, `0xc0` Papier da,
  `0xe8` leer).
- **Stapeltrennung:** Die Seitenzahl unten ist das verlässliche Merkmal. Ohne
  sie gilt der Kopf – aber Vorlagen mit laufendem Briefkopf («Ganglion Frau
  Dr. med. Ursula Davatz – www.ganglion.ch …» auf jeder Seite) haben sonst
  jede Seite zum eigenen Vortrag gemacht, und «Dr. med. U. Davatz» /
  «www.ganglion.ch» auf zwei Zeilen wurde übersehen. Die Unterschrift am
  Textende («Dr. med. U. Davatz» ohne Adresse) darf nie trennen.
- **Titel:** Der Block direkt über der Autorenzeile ist der Titel, die Blöcke
  darüber sind Veranstalter («La vida Wohlen», «Praxis Frau Lutz»). Steht
  im Titel eine Trennung mit grossem Anschluss («Be-/Treuung»), wird sie
  nicht zusammengefügt – dann mit `--titel` korrigieren.
- **Vision lässt Zeilen aus – darum Tesseract.** Apples Vision lieferte aus
  dem Rust-Programm (objc2-vision) auf derselben Seite 15 statt 23 Zeilen,
  die ein Swift-Programm vollständig las – bei gleichen Einstellungen,
  Revision 3, gleichem Deployment-Target, auch mit `usesCPUOnly`. Die
  Ursache blieb offen. Fehlende Zeilen fehlen in der Textebene, und «alles
  markieren» verpasst Text; der Benutzer verlangt, dass **kein Buchstabe**
  fehlt. Tesseract liest alle Zeilen. Prüfen lässt sich das, indem man
  `tesseract <bild> - -l deu` mit `pdftotext` der fertigen Seite vergleicht.
  Tesseract-Eigenheiten: «www,» statt «www.» (wird beim Verlinken
  korrigiert), «l» statt «I», Logos als Buchstabensalat («Ga ngl ION»).
- **Tesseract liest das doppelt vergrösserte Bild.** Bei 150 dpi übersah es
  unterstrichenen Text samt der Zeile («www.ganglion.ch» unter der
  Autorenzeile, darum kein Link) und fast alle kleinen Seitenzahlen. Die
  Fusszeile (unterstes Achtel) wird zusätzlich für sich gelesen, einmal
  normal, einmal nur mit Ziffern und Strichen (`--psm 11`, Whitelist).
- **Fusszeilen-Zahlen nur als Hinweis.** Der Ziffern-Durchgang liest auf
  alten Seiten Flecken als «5» oder «0», und «- 5 -» wurde auch schon «9».
  Als er mitentscheiden durfte, verschmolz er zwei Vorträge. Getrennt wird
  darum nur nach der Seitenzahl im Seitentext (`seitenzahl`) und dem Kopf;
  die Fusszeilen-Zahlen lösen nur «fehlt eine Seite?» aus. Vom
  Ziffern-Durchgang zählt nur eine eingefasste Zahl («-5-»).
- **Der Benutzer nennt die Seitenzahl des Stapels** («5 Seiten»). Mit der
  Zahl der eingezogenen Blätter vergleichen – zweimal fehlte ein Blatt, ohne
  dass eine Lücke gemeldet wurde.
- **Kontaktdaten:** `heikle_stellen()` findet Mail-, Strassen- und
  Telefonangaben in der Zeile; die betroffenen Wörter fallen aus dem Text,
  ihre Tesseract-Wortkästen werden übermalt. Übermalt wird im JPEG
  selbst – ein weisses Rechteck im PDF liesse das Bild darunter auslesbar.
  Der Kasten reicht 15 % der Zeilenhöhe nach oben und 5 % nach unten; mehr
  schneidet die Umlautpunkte der nächsten Zeile ab.
- **Vor dem Push** die Texte prüfen: `pdftotext scan/<datei>.pdf - | grep -iE
  '@|strasse [0-9]|tel\.'` muss leer bleiben, und Seite 1 rendern
  (`pdftoppm -png -r 55 -f 1 -l 1`), um Datum und Titel anzusehen.

### Daten (`src/eintraege.rs`)

Generiert, nicht von Hand gepflegt. Ein Eintrag je Beitrag mit `datum`,
`titel`, `url`, `woerter`, `aufrufe` und `ocr`. Sortiert nach `aufrufe`,
meistgelesene zuerst. Das Feld `ocr` markiert die Fälle, deren
Quelle ein Scan ohne Textebene war; sie erscheinen im PDF an drei Stellen:
in den Kennzahlen der Titelseite, in einem eigenen Abschnitt und als Marke
in der Metazeile des Listeneintrags.

Beim Neuerzeugen der Datei die Reihenfolge beibehalten (nach `aufrufe`
absteigend) und `ocr` korrekt setzen – sonst stimmt die Zählung auf der Titelseite nicht.

**Wird ein Beitrag auf adhs.expert gelöscht, muss sein Eintrag hier heraus.**
Sonst führt das PDF einen Link auf eine 404-Seite. Alle Kennzahlen der
Titelseite – Zahl der Beiträge, Wörter, Aufrufe, OCR-Fälle – werden aus
`EINTRAEGE` berechnet und stimmen nach dem Neubau von selbst; nichts davon steht
fest verdrahtet. Danach `cargo run --release --bin uebersicht` laufen lassen und
prüfen, dass die Link-Assertion durchgeht. Bereits so entfernt: ein fünfter
OCR-Fall und, im September 2026, «Perfektionismus und Prokrastination»
(Teilnehmeraufruf statt Vortrag), «Jeder Hundertste erkrankt an Schizophrenie»
(dasselbe Interview wie «Emotionale Monsterwelle», dieselbe Zeitungsseite) und
«Psychotherapie und Coaching» (nicht ihr Text, sondern die Arbeitsbeschreibung
einer Kollegin samt Kontaktdaten). Im Oktober 2026 kamen «Ronni Rocket»
(Organisationsblatt einer Matinée – Zeitplan, Namensetiketten, Spesen – mit drei
fremden Mailadressen) und ein zweites Praxisprofil einer Kollegin (wieder
nicht ihr Text, mit Praxisadresse, Mailadresse und Handynummer) dazu, und am
23. Oktober vier Dubletten (siehe unten).

Gelöscht wird über die REST-API mit dem Anwendungspasswort aus
`~/.config/adhs-expert/credentials` (Muster in `tools/apply_one.py`). Ein
`DELETE /wp-json/wp/v2/posts/<id>` legt den Beitrag in den Papierkorb und ist
umkehrbar; erst `?force=true` löscht endgültig. Vorher den Beitrag als JSON
sichern – der Papierkorb wird nach 30 Tagen von WordPress selber geleert.

**Dubletten findet man über die Prüfsumme des Transkripts.** Im Oktober 2021
wurde eine Reihe ein zweites Mal als «Podcast:» veröffentlicht; ausserdem lag
eine Weiterbildung dreifach vor. Ein Durchgang über alle 1170 Beiträge – Text
nach `<h2>Transkript</h2>` normalisieren, SHA bilden, gruppieren – ergab vier
Gruppen mit zeichengleichem Text. Dabei zweierlei beachten: erstens, welcher
Beitrag von einem LinkedIn-Post verlinkt ist (der muss bleiben, sonst bricht der
Link), zweitens, ob die Beiträge dieselbe Audiodatei tragen. Bei «Prävention
durch richtig Handeln» war das nicht so – der Beitrag vom 15.3.2021 trägt die
unbearbeitete Aufnahme (441 MB), der vom 10.5.2021 die von Shelter Schweiz
bearbeitete (228 MB). Darum stehen dort bewusst zwei Einträge mit demselben
Text in `EINTRAEGE`.

**Die Vortragsliste auf ganglion.ch ist die Gegenprobe.**
`https://ganglion.ch/html/vortraege.php` führt über 450 Vorträge mit
Vortragsdatum und PDF; die Datei hängt an
`/html/php/download_vortrag.php?id=<id>&download=pdf` oder direkt unter
`https://ganglion.ch/pdf/<name>.pdf` (der Name folgt keinem Muster, ein
Verzeichnislisting gibt der Server nicht her). Die Titel weichen ab, darum über
`difflib` auf normalisierten Titeln abgleichen. **Das Datum dort ist das
Vortrags-, das auf WordPress das Publikationsdatum** – sie weichen bei neun von
zehn Beiträgen ab, meist um Tage. Gross und darum erklärungsbedürftig sind
«Cannabis und Schizophrenie» (Beitrag 2017, Vortrag 2003) und «Umgang mit der
Gefühlswelt in der Pubertät» Teil 1 und 2 (Beitrag 2021, Vortrag 2014).

**Eine kleine `woerter`-Zahl heisst nicht, dass der Beitrag nichts hergibt.**
Mehrere Einträge mit dreistelliger Wortzahl sind Vorträge, deren Volltext nie in
den Beitrag kam: Im Beitrag steht nur die Einladung, das Abstract oder das
Tagungsprogramm, während der Vortrag als Audio oder als Transkript-PDF daneben
liegt – zum Teil im Beitrag selbst verlinkt. Beim Durchgehen aller Einträge
unter 1200 Wörtern (Oktober 2026) traf das auf «Bindung – was die Welt
zusammenhält», «Rolle der Frau in der heutigen Gesellschaft», «Spannungsfeld
zwischen Psychologie, Religion und sozialer Verantwortung», «Jugendliche
zwischen Stuhl und Bank beim Berufseinstieg», «Einladung zum Jubiläumsanlass
20 Jahre VASK Schweiz» und «Diplom- und Abschlussfeier Mütterberaterinnen» zu.
Bevor so ein Beitrag gelöscht wird, prüfen, ob der Vortrag noch irgendwo liegt.

**Und: die Überschrift `<h2>Transkript</h2>` trägt nicht überall das
Transkript.** Bei «Jugendliche zwischen Stuhl und Bank» stand darunter das
Workshop-Handout aus dem Abstract-PDF – beim ursprünglichen Durchlauf war das
falsche PDF angehängt worden. Der Beitrag sah damit aus, als sei er erledigt,
und `tools/wp.py woerter()` zählte das Handout. Vor dem Nachtragen also
anschauen, was unter der Überschrift wirklich steht; das Handout bekommt eine
eigene Überschrift (dort «Unterlagen zum Workshop»), damit je Beitrag genau ein
`<h2>Transkript</h2>` bleibt. Bei «Rolle der Frau» und «Bindung» hat der
Benutzer die andere Variante gewählt: Einladung beziehungsweise Kurztext bleiben
zuvorderst **innerhalb** des Transkriptabschnitts, der Vortrag folgt darunter –
dann zählt `woerter()` beides. Das Nachtragen selbst macht
`tools/verbatim.py build(alt_html, pdf, heading="Transkript")`; die Assertion,
die den zurückverwandelten HTML-Text mit `pdftotext` vergleicht, muss durchgehen.

Nicht automatisch löschen, was dünn aussieht: `tools/wp.py woerter()` splittet
nur auf `<h2>Transkript</h2>` und zählt bei den vier Beiträgen mit der
Überschrift «Transkript (OCR)» den ganzen Beitrag mit.

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
- **Die Permalinks sind datumsbasiert** (`/JJJJ/MM/TT/slug/`). Wer über die
  REST-API das Feld `date` korrigiert, ändert damit die Adresse des Beitrags.
  WordPress leitet die alte mit 301 auf die neue um, solange der Slug eindeutig
  bleibt; der Eintrag in `src/eintraege.rs` muss aber von Hand nachgezogen
  werden. Die Jetpack-Zahlen hängen an der Post-ID und bleiben unberührt.

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
  Spaltensatz mit und erzeugt unlesbare Umbrüche mitten im Satz. Schlimmer:
  **es zieht die Nachbarspalten mit hinein.** Zwölf Beiträge trugen deshalb
  bis September 2026 fremde Inhalte — die Kolumne einer anderen Autorin mit
  sehr persönlichen Angaben, ein Polizeibericht über einen Raubüberfall, eine
  Medizinkolumne, Restaurant- und Wahlinserate mit Telefonnummern, ein
  Migros-Inserat. Wer solche Seiten übernimmt, muss die Spalten trennen:
  Wortkoordinaten aus `pdftotext -bbox`, Spalten über die senkrechten
  Leerstreifen, Absätze im Blocksatz über die kurze Schlusszeile und im
  Flattersatz über den Zeilenabstand. `tools/spalten.py` macht das, die
  Zuschnitte je Beitrag liegen in `tools/spaltensatz/`.
- **Welche Spalte zum Artikel gehört, sieht man nur im Bild.** Vor jedem
  Zuschnitt die Seite mit `pdftoppm -png -r 110` rendern und selber ansehen.
  Kein Textmerkmal unterscheidet zuverlässig zwischen dem eigenen Artikel und
  dem Nachbarartikel auf derselben Seite.
- **Das weiche Trennzeichen (U+00AD) hat zwei Bedeutungen.** Am Zeilenende
  trennt es ein Wort und muss beim Zusammenfügen verschwinden, mitten in der
  Zeile ist es der sichtbare Bindestrich und muss bleiben. Wer es pauschal
  entfernt, macht aus «Therapie-Familien» ein «TherapieFamilien».
- **Ligaturen können ganz aus dem Wort fallen.** Bei PDFs, die macOS Quartz
  erzeugt hat, tragen die Glyphen ff, fi, fl, ffi und ffl keine brauchbare
  Zuordnung; beim Auslesen bleibt an ihrer Stelle ein **Leerzeichen**
  («Begri » statt «Begriff», «häu g» statt «häufig», «Back sch» statt
  «Backfisch»), und die Glyphen selbst landen als verwaiste Zeilen am
  Seitenende. `pdffonts` zeigt davon nichts, der übrige Text ist sauber.
  `tools/ligatur.py` setzt sie wieder ein – gegen ein Wörterbuch aus allen
  anderen Beiträgen, in denen dieselben Wörter korrekt stehen. Erst die
  verwaisten Glyphen entfernen, dann die Lücken füllen. Betroffen waren im
  September 2026 zwei Beiträge; alle 1174 wurden geprüft.
- **Bei eng ausgetriebenem Blocksatz verschluckt `pdftotext` Wortabstände**
  und liefert `BaselMedizin,interessiertesich` als ein Token. Die Leerstellen
  fehlen im PDF selbst, `pdftohtml -xml` hilft also nicht. An Satzzeichen und
  klein-gross-Übergängen lässt sich das automatisch trennen, der Rest bleibt
  Handarbeit.
- PDFs brechen lange URLs am Bindestrich um. Diese Umbrüche sind Layout,
  nicht Text – ohne Zusammenfügen entstehen zerrissene, tote Links.
- **Eine Textebene heisst nicht, dass Text herauskommt.** Das PDF des
  Zeitungsartikels *Sie hat im Aargau Spuren hinterlassen* trug eine, aber
  alle Schriften waren subgesetzte CID-Fonts mit Identity-H-Kodierung **ohne
  ToUnicode-Tabelle**. `pdftotext` gibt dann Glyph-Nummern als Zeichen aus:
  `3! L%P!3D!8 P3!` statt Wörtern. Genau das stand jahrelang im Beitrag.
  `pdffonts <datei>` zeigt es sofort – Spalte `uni` auf `no`. Vor jeder
  Übernahme prüfen, sonst wandert Zeichensalat ungebremst in den Beitrag:
  Anteil der Wörter aus `[A-Za-zÄÖÜäöüß0-9…]` messen, sauber ist ~100 %.
  Der Ausweg ist derselbe wie beim Scan: `pdftoppm -png -r 400` mit
  `-x -y -W -H` spaltenweise ausschneiden und die Bilder selbst lesen. Beim
  Zusammenfügen der Silbentrennung echte Bindestriche stehen lassen
  (`Psychiatrie-Studium`, `80er-Jahren`).
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
- Scans, die versehentlich im Hauptverzeichnis landen (`/*_TT.MM.JJJJ.pdf`).
  Gewollt eingecheckt sind nur die Scans in `scan/` – mit entfernten
  Mail-, Strassen- und Telefonangaben (Wunsch des Benutzers).
- `target/`

Ein `.gitignore`-Eintrag ist eine Vorsichtsmassnahme, kein Schutz – ein
`git add -f` genügt. Vor jedem Commit `git status` prüfen; niemals
Zugangsdaten, Anwendungspasswörter oder private Mailadressen in eingecheckte
Dateien schreiben.

## Lizenz

GPL-3.0. Neue Quelldateien tragen einen GPL-3.0-verträglichen Kopf, und jede
Abhängigkeit muss mit GPL-3.0 vereinbar sein.
