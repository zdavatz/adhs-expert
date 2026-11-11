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
und legt je Vortrag ein durchsuchbares PDF in `scan/` ab. Die PDFs werden mit
dem Repo committet.

```bash
brew install sane-backends tesseract tesseract-lang
cargo run --release --bin scannen
cargo run --release --bin scannen -- --datum 5.3.2014 --titel "Titel"
```

**Dateiname:** `Titel_des_Vortrags_05.03.2014.pdf` – Titel zuerst,
Unterstriche statt Leerschlägen, Datum am Schluss, höchstens 60 Zeichen Titel,
an einer Wortgrenze gekürzt. In den Namen kommt der Titelblock direkt über der
Autorenzeile; darüber stehen meist Datum und Veranstalter («FDP Villmergen»,
«Elternbildung Reinach»). Die Dokumenteigenschaften tragen den ganzen Kopf.
Gleiche Namen bekommen `_2`, `_3`.

**Datum und Titel** liest die Texterkennung (Tesseract, Deutsch) aus den Seiten:
das Datum im Kopf der ersten Seite, sonst das letzte im Dokument (oft steht am
Schluss «6. August 2003»). Steht keines im Text, gilt der Scantag; in den
Dokumenteigenschaften heisst es dann «gescannt am» statt «Vortrag vom».
Nennt der Kopf nur ein Jahr («Familientherapie-Kurs I 1991», «Kurs II/89»),
kommt dieses Jahr mit Tag und Monat vom Scan in den Dateinamen
(`…_29.10.1991.pdf`); die Dokumenteigenschaften sagen das ausdrücklich.
Nennt er Monat und Jahr («Dezember 1991»), heisst die Datei `Titel_12.1991.pdf`;
eine Jahreszeit («Herbst 1988») gilt wie ein blosses Jahr.
`--datum` und `--titel` überschreiben die Erkennung.

**Textebene und Links:** Die erkannten Wörter liegen unsichtbar über den
Seitenbildern, jedes an seiner Stelle; das PDF lässt sich vollständig
markieren und durchsuchen. Erkannte Webadressen sind anklickbar. Die
Erkennung kann sich verlesen («www,ganglion.ch», «l Einleitung»), lässt aber
keine Zeile aus. Gelesen wird das doppelt vergrösserte Bild; erst so erfasst
Tesseract auch unterstrichenen Text und die kleinen Seitenzahlen.

**Fehlende Seiten:** Die Seitenzahl in der Fusszeile wird eigens vergrössert
gelesen. Springt sie («-3-», dann «-5-»), meldet das Programm «fehlt eine
Seite?». Weil auch diese Lesung irren kann, entscheidet sie nie über die
Trennung der Vorträge.

Bis Oktober 2026 lief hier Apples Vision. Aus dem Rust-Programm heraus liess
es auf vielen Seiten ganze Zeilen aus – beim «Trotzenden Kind» fehlte rund ein
Drittel des Textes in der Textebene. Alle Scans sind seither mit Tesseract neu
aufgebaut; ein Wortvergleich zeigte, dass dabei nichts verloren ging, was
Vision gelesen hatte.

**Mehrere Vorträge in einem Stapel** ergeben je ein PDF. Getrennt wird nach der
Seitenzahl unten (1 beginnt einen Vortrag, «Seite 3 von 7» setzt ihn fort, eine
Lücke in der Zählung wird gemeldet). Ohne Seitenzahl entscheidet der Kopf:
«Vortrag vom …» oder die Autorenzeile mit Adresse. Ein laufender Kopf, der auf
jeder Seite wiederkehrt, zählt dabei nicht.

**Kontaktdaten:** Mailadressen, Strassenadressen mit Hausnummer und
Telefonnummern werden vor dem Ablegen entfernt – im Bild weiss übermalt (ein
Kasten im PDF genügte nicht, das Bild darunter liesse sich herausziehen) und
aus der Textebene gelöscht.

**Auflösung:** 150 dpi in Graustufen, einseitig, A4. Für kleine Schrift oder
Bleistiftnotizen `--dpi 300`, für Farbe `--farbe`.

**Werkzeuge ohne Scanner:** `--lesen BILD…` zeigt nur, was erkannt wird;
`--aus BILD…` baut die PDFs aus schon gescannten JPEGs – etwa um nach einem
Papierstau den Rest mit den ersten Seiten zusammenzusetzen. Die JPEGs eines
fertigen PDFs holt `pdfimages -j` wieder heraus. `--ein` hält den Stapel
zusammen, etwa wenn ein Kurztext als erste Seite vor den Vortrag gehört und
sonst als eigener Vortrag abgetrennt würde. `--adresse-behalten` lässt
Strassenangaben stehen, etwa den Vortragsort im Kopf; Mail und Telefon werden
trotzdem entfernt. `--sprache eng` (oder `deu+eng`) liest englische
Vorlagen; Vorgabe ist `deu`.
`--duplex` zieht beidseitig bedruckte Blätter mit Vorder- und Rückseite ein.
Schief eingezogene Seiten (ab 0,2°, gemessen mit `magick -deskew 80%`)
richtet `scannen` vor der Texterkennung selbst gerade; Geraderichten und
Texterkennung laufen je Seite parallel.

**Hinweis auf adhs.expert:** Jede Seite bekommt unten eine kleine graue
Fusszeile «https://adhs.expert», klickbar – auch Vorlagen, die selbst auf
ganglion.ch verweisen. Bestehende
PDFs rüstet `--fuss PDF…` nach, ohne Bild oder Textebene anzutasten.

**Warum SANE und nicht Epson Scan 2:** Dessen Argument `-SCAN` ist nicht
dokumentiert, und der Image-Capture-Treiber «EPSON Scanner.app» ist nur für
Intel gebaut und startet auf Apple Silicon ohne Rosetta nicht. SANE führt den
GT-2500 im Backend `epson2` als «ES-H300».

### Übersicht der Scans

[`Vorträge_Dr.med._Ursula_Davatz_von_1980_bis_2018.pdf`](Vortr%C3%A4ge_Dr.med._Ursula_Davatz_von_1980_bis_2018.pdf)
führt alle Scans nach
Vortragsdatum auf, je Datei mit Seitenzahl, einer Zusammenfassung in drei bis
fünf Sätzen und einem Link auf das PDF hier auf GitHub. Die Zusammenfassungen
stehen in `src/zusammenfassungen.rs`. Nach jedem neuen Scan trägt `eintragen`
die Zusammenfassung alphabetisch ein, setzt die Zeile im Bestand unten nach
Datum und zählt «N PDFs, M Seiten» aus `scan/` neu; danach die Übersicht
neu erzeugen:

```bash
cargo run --release --bin eintragen -- --datei Titel_TT.MM.JJJJ.pdf \
    --text "Drei bis fünf Sätze." --bestand "2 (Vortrag …)" [--titel "Voller Titel"]
cargo run --release --bin scanliste
```

Fehlt zu einem PDF in `scan/` die Zusammenfassung, bricht das Programm ab.

### Scans per Mail verschicken

`src/mail.rs` schickt eine Mail mit Anhängen über die Gmail-API:

```bash
cargo run --release --bin mail -- --an adresse@example.com \
    --betreff "Betreff" --nachricht "Text" --anhang scan/x.pdf \
    --token token.json        # oder: ADHS_MAIL_TOKEN=token.json
```

Der Token ist eine Google-OAuth-Datei («authorized user», Bereich
`gmail.send`) mit Refresh-Token; das Programm holt damit bei jedem Aufruf ein
frisches Zugriffstoken. `--text datei.txt` liest den Text aus einer Datei,
`--trocken` prüft Zugang und Nachricht, ohne zu senden. Empfänger und
Token-Pfad gehören nicht ins Repository.

### Ordnerrücken drucken

`src/etiketten.rs` setzt Rückenschilder «Vorträge Dr. med. Ursula Davatz
JJJJ – JJJJ» nebeneinander auf ein A4-Blatt im Querformat, mit gestrichelter
Schnittlinie. Vorgabe 50 × 145 mm (Einstecktasche der grauen Ordner);
`@MM` hinter den Jahren setzt die Breite eines einzelnen Schilds. Schilder
unter 40 mm Breite bekommen den Text längs, von unten nach oben lesbar.

```bash
cargo run --release --bin etiketten -- --jahre 1983-1988 --jahre 1987-1994 \
    --jahre 1991-1999 --jahre 2013-2018@25 --out etiketten.pdf
cargo run --release --bin pcldruck -- etiketten.pdf --an DRUCKER:9100
```

`src/pcldruck.rs` druckt die **erste Seite** eines PDFs als PCL-Rasterbild
(300 dpi, schwarzweiss, `pdftoppm -mono`) auf einen Drucker, der kein PDF
versteht – etwa einen HP LaserJet 2100 am Port 9100. Querformat wird gedreht,
der unbedruckbare linke Rand (1/4 Zoll) links abgeschnitten, damit das Bild
an derselben Stelle steht wie im PDF. `--datei seite.pcl` schreibt nur.
Die Jahre im Dateinamen sind das erste und das letzte Vortragsjahr; ändern
sie sich, ersetzt die neue Datei die alte.

### Bestand

365 PDFs, 1406 Seiten, nach Vortragsdatum (Scantag, wo der Text keines nennt; bei blosser Jahresangabe das Jahr mit Tag und Monat vom Scan).

| Datei | Seiten |
|---|---|
| `Vortrag_über_Drogenprobleme_06.06.1980.pdf` | 2 (Text datiert 6.6.1980 am Schluss) |
| `Einige_Richtlinien_für_die_familiensystemorientierte_07.1980.pdf` | 5 (Sozialpsychiatrischer Dienst Königsfelden; datiert «Juli 1980»; Bild abgedunkelt, unterstrichener Titel für die Texterkennung ohne Unterstreichung gelesen) |
| `Krisensituationen_in_der_Familie_03.08.1980.pdf` | 3 (Vortrag an der Vermittlerinnentagung des Haus- und Krankenpflegevereins; Stichworte) |
| `Krisensituationen_in_der_Familie_03.08.1980_2.pdf` | 3 (zweite Fassung, anders gesetzt, Kopf von Hand, mit Abschnitt «Selbst verursachte Krisen») |
| `Vortrag_betreffend_Drogen_in_Magden_23.10.1980.pdf` | 3 (Stichworte mit vielen Handnotizen; Text datiert Okt. 80) |
| `Familientherapie_Familiensystemtheorie_13.11.1980.pdf` | 7 (Vortrag an der 57. Supervisionssitzung im SPD Königsfelden, nach Tonband aufgezeichnet von Benedikt Stalder; erster Teil; auf Wunsch nicht anonymisiert) |
| `Gedanken_zur_Präventivpsychiatrie_15.12.1980.pdf` | 7 (Bericht über ein Gespräch mit U. Davatz, Sozialpsychiatrischer Dienst Königsfelden; Datum am Schluss) |
| `Gedanken_zur_Präventivpsychiatrie_15.12.1980_2.pdf` | 7 (zweite Fassung, anders gesetzt: Titel links und unterstrichen, anderer Umbruch; Seite 5 kam zuletzt aus dem Einzug, Überlauf auf Seite 4 übermalt) |
| `Drogengefährdung_der_Jugendlichen_Fragen_der_CVP-Fraktion_17.12.1980.pdf` | 5 (Sozialpsychiatrischer Dienst Königsfelden, Antworten auf 13 Fragen zur Veranstaltung der CVP-Fraktion Baden/Obersiggenthal; Handnotiz auf Seite 5) |
| `Angst_und_ihre_Auswirkung_auf_Krisensituationen_20.02.1981.pdf` | 6 (Vortrag an der Generalversammlung der Familienhilfe Koblenz; blasse Schreibmaschinenschrift, Bild abgedunkelt; Text datiert 19.2.1981) |
| `Ablösungskonflikt_zwischen_erwachsenen_Kindern_und_ihren_16.03.1981.pdf` | 4 (Vortrag vor dem Hauspflegeverein Villnachern; Abschrift datiert 9.3.1981; Bild abgedunkelt, unterstrichener Titel für die Texterkennung ohne Unterstreichung gelesen) |
| `Das_Drogenproblem_aus_der_Sicht_der_Familiensystemtheorie_27.04.1981.pdf` | 4 (Vortrag für den Haus- und Krankenpflegeverein Hausen; blasse Schreibmaschinenschrift, Bild abgedunkelt) |
| `Prophylaxe_in_der_Psychiatrie_08.05.1981.pdf` | 5 (Vortrag in der Reihe «Gesundheit und Medizin» der Christlich-sozialen Krankenkasse, Sektion Wettingen; Abschrift datiert 6.9.1982) |
| `Fortbildung_für_Heimleiter_und_Erzieher_22.10.1981.pdf` | 3 (Sozialpsychiatrischer Dienst Königsfelden, Fortbildung über verhaltensgestörte Jugendliche; Datum am Schluss) |
| `Einführung_in_das_Denken_von_Jean_Piaget_11.1981.pdf` | 6 (Skript der theoretischen Fachausbildung in Psychiatrie und Psychotherapie, Psychiatrischer Dienst für Erwachsene Kanton Aargau; Monat und Jahr am Schluss; Überlauf eines Doppeleinzugs auf Seite 2 übermalt) |
| `Jugend_Unser_Problem_oder_unsere_Zukunft_20.06.1983.pdf` | 4 (Beitrag zu einem Podiumsgespräch über Schwierigkeiten, Ansätze und Zuständigkeiten einer Jugendpolitik) |
| `Die_psychische_Komponente_des_Asthmas_10.11.1983.pdf` | 3 (Diskussionsbeitrag an der Weiterbildungstagung der Aargauischen Frauenliga auf der Barmelweid; gedruckt auf einem gefalteten A4-Blatt, in A5-Einzelseiten zerlegt, leere Rückseite weggelassen) |
| `Vortrag_über_Drogen_für_den_Frauenbund_Leuggern_02.04.1984.pdf` | 1 (Gliederung in fünf Punkten, Vortrag in Leuggern) |
| `Der_Patient_in_seinem_Umfeld_07.05.1984.pdf` | 2 (Thesen für den Schweizerischen Fachverband Sozialdienst in Spitälern, Aarau; zwei Blätter mit eigenem Titel) |
| `Drogenvortrag_06.06.1984.pdf` | 1 (Stichworte zum Aufbau einer Suchtpräventionsberatungsstelle) |
| `Alkoholismus_Eine_Krankheit_03.08.1984.pdf` | 3 (Text, datiert 3.8.1984 am Schluss) |
| `Hypothesen_zur_Suchtkrankheit_09.08.1984.pdf` | 1 (sechs Thesen, datiert Königsfelden, 9.8.84) |
| `Mein_Verständnis_der_Schizophrenie_heute_15.08.1984.pdf` | 1 (Vortrag vor freiwilligen Helfern; Stichworte) |
| `Krankheitsbild_der_Schizophrenie_28.09.1984.pdf` | 3 (Text mit Abschnitten zu Symptomen, Ätiologie, Familientherapie und Helferproblemen; datiert 28.9.84 am Schluss) |
| `Depression_28.09.1984.pdf` | 2 (Text, gleiche Reihe wie «Krankheitsbild der Schizophrenie», datiert 28.9.84 am Schluss) |
| `Psychische_Gesundheit_und_Krankheit_im_Alltag_08.11.1984.pdf` | 2 (Vortrag an einer Zusammenkunft der SVP-Frauen; Stichworte; Namen in drei Beispielen durch erfundene ersetzt) |
| `Erziehungsfragen_Suchtberatung_13.11.1984.pdf` | 2 (Elternabend Murgenthal; Stichworte, Datum von Hand) |
| `Gibt_es_Schulstress_Wie_steht_es_um_das_Drogenproblem_06.02.1985.pdf` | 1 (Vortrag, CVP; Stichworte) |
| `Drogen_Suchtverhalten_15.03.1985.pdf` | 1 (Vortrag, Lehramtschule Gränichen; Stichworte) |
| `Familiendynamik_Theorie_und_Praxis_15.03.1985.pdf` | 1 (Stichworte für Haushaltanleiterinnen, drei Punkte) |
| `Einladung_zur_Sitzung_des_Kantonalvorstandes_der_19.03.1985.pdf` | 1 (Einladung vom 5.3.1985 zur Sitzung im Aarhof Wildegg mit Aussprache mit U. Davatz; auf Wunsch nicht anonymisiert, Privatadresse und Telefon im Kopf entfernt) |
| `Jugend_und_Suchtprobleme_21.03.1985.pdf` | 1 (Vortrag, Aargauischer Frauenbund; Stichworte) |
| `Sozialmedizinische_Gesichtspunkte_23.05.1985.pdf` | 2 (Beitrag zur Pressekonferenz der AFGA; Text datiert Königsfelden, 14.5.1985) |
| `Beratung_und_Betreuung_von_Langzeitpatienten_und_ihren_20.06.1985.pdf` | 11 (Referat an der Generalversammlung der Aargauischen Frauenliga gegen Tuberkulose und langdauernde Krankheiten, Lenzburg) |
| `Einladung_zur_Generalversammlung_der_Aargauischen_Frauenliga_20.06.1985.pdf` | 1 (gedruckte Einladung zur Generalversammlung mit der Aargauischen Krebsliga, Lenzburg, mit ihrem Referat; Kontrast angehoben, graues Papier) |
| `Brief_der_Gesundheits-Beratungsstelle_Baden_zum_Referat_in_27.06.1985.pdf` | 1 (Begleitbrief von Gerda Haber zur Abschrift des Referats vom 20.6.1985, auf Wunsch nicht anonymisiert; Strasse und Telefon im Briefkopf entfernt; Handnotizen) |
| `Gassenarbeit_Stubenarbeit_24.10.1985.pdf` | 2 (Vortrag in Gebenstorf; Stichworte, Seite 2 mit Gegenüberstellung Stube/Gasse) |
| `Psychohygiene_des_Pflegepersonals_im_Umgang_mit_26.10.1985.pdf` | 2 (Stichworte für Pflegepersonal, Seite 2 nur der Schluss und «Fallvorstellung:») |
| `Psychohygiene_des_Pflegepersonals_im_Umgang_mit_26.10.1985_2.pdf` | 19 (Abschrift des Vortrags zu den Stichworten, SBK, für Gemeindeschwestern, mit Fragen aus dem Publikum; Überläufe aus Doppeleinzügen auf fünf Seiten übermalt) |
| `Was_sind_die_Besonderheiten_im_Alter_Was_sind_unsere_07.11.1985.pdf` | 1 (Weiterbildung für Sportleiterinnen der Pro Senectute; Datum von Hand) |
| `Interview_am_Radio_DRS_anlässlich_der_Pro_Mente_Sana-Tagung_10.11.1985.pdf` | 5 (Abschrift einer Sendung von Radio DRS über die Jahrestagung der Stiftung Pro Mente Sana 1985; nur das Jahr, Tag und Monat vom Scan; Überlauf eines Doppeleinzugs auf Seite 1 übermalt) |
| `Neue_psychosoziale_Erkrankungen_bei_Kindern_und_Jugendlichen_10.11.1985.pdf` | 27 (Abschrift eines Vortrags in Heidelberg 1985 mit Fragen aus dem Publikum; nur das Jahr, Tag und Monat vom Scan; Seitenzahlen oben) |
| `Einflussnahme_des_Schulsystems_auf_neue_Formen_der_10.11.1985.pdf` | 4 (Gliederung und Stichworte zum Heidelberger Vortrag 1985; von Hand «1995 (?)», nach dem Heidelberger Vortrag 1985 datiert, Tag und Monat vom Scan; Namen in den Beispielen auf Wunsch stehen gelassen) |
| `Drogenprophylaxe_und_Drogentherapie_25.11.1985.pdf` | 3 («Drogen im Aargau», Informationsveranstaltung des Gemeindeverbandes Sozialdienste Bezirk Rheinfelden, Arbeitsgruppe Suchtprobleme; Datum wie im Kopf, die Vortragsliste 1986–1988 nennt den 25.11.1986) |
| `Psychiatrie_im_Alltag_25.11.1985.pdf` | 2 (Vortrag, Frauenverein Kaiseraugst; Stichworte) |
| `Jugend_und_Drogen_16.01.1986.pdf` | 1 (Vortrag am Elterngesprächsabend, Aula des Oberstufenschulhauses Ebnet, Frick; Stichworte) |
| `Selbsthilfe_Eigenverantwortung_oder_Laientherapie_04.03.1986.pdf` | 5 (Streitgespräch mit Jakob Bösch im Tages-Anzeiger, abgetippt; Erscheinungsdatum, die Vortragsliste 1986–1988 nennt das Gespräch am 15.1.1986; Kontaktadressen und Telefonnummern entfernt, zum Teil von Hand; ein Abschnitt auf Seite 3 im Original doppelt getippt) |
| `Emotionelle_Hintergründe_zum_Lernprozess_05.03.1986.pdf` | 3 (Vortrag vor der Krankenpflegeschule Chur, «… Bezugnahme auf die Situation der Krankenpflege»; Titelblatt und zwei Seiten Stichworte) |
| `Drogenprobleme_in_der_Familie_wie_gehe_ich_damit_um_17.03.1986.pdf` | 1 (Vortrag für die Elternvereinigung DAJ Aargau in der Psychiatrischen Klinik; Stichworte) |
| `Spitex_in_der_Psychiatrie_18.03.1986.pdf` | 2 (Vortrag an der Generalversammlung des Haus- und Krankenpflegevereins Würenlingen/Station Siggenthal, Bären Würenlingen; Stichworte) |
| `Viel_Wissen_um_Krankheit_macht_noch_nicht_gesund_20.03.1986.pdf` | 2 (Diskussionsrunde, staatsbürgerliche Woche der Kantonsschule Zug; Stichworte) |
| `Das_Kind_als_schwächstes_Glied_im_System,_wie_gehen_wir_28.05.1986.pdf` | 4 (Vortrag zur Tagung «Schule und Prävention», Lehrertagung HTL Windisch; Text datiert 22.5.1986) |
| `Welche_Patienten_können_im_Wohnheim_aufgenommen_werden_28.05.1986.pdf` | 1 (Vortrag an der Generalversammlung der EPK, Restaurant Cardinal, Windisch; Stichworte) |
| `Umgang_mit_Psychiatriepatienten_ausserhalb_der_Klinik_03.06.1986.pdf` | 2 (Vortrag für die Sozialtätigen im Bezirk Bremgarten, St. Josefsheim; Patientenname im Beispiel durch erfundenen ersetzt) |
| `Ziele_und_Organisation_des_sozialpsychiatrischen_Dienstes_10.06.1986.pdf` | 2 (Vortrag beim Schweizerischen Verband der Akademikerinnen, Sektion Aargau, Hotel Du Parc Baden; zwei Personennamen im Beispiel durch erfundene ersetzt) |
| `Bürokratisierung_und_Entbürokratisierung_der_Sozialhilfe_25.08.1986.pdf` | 12 («… demokratische Alternativen zum bürokratischen Wohlfahrtsstaat», Vortrag an der GDI-Konferenz 25.–29.8.1986; spätere Reinschrift mit www.ganglion.ch; Datum erster Konferenztag) |
| `Bureaucratization_and_De-Bureaucratization_of_Social_Welfare_25.08.1986.pdf` | 15 (englisches Original des GDI-Vortrags, «… Democratic Alternatives to the Bureaucratized Welfare State», mit Summary und Literatur; mit `--sprache eng` gelesen) |
| `Psychiatrie_im_täglichen_Leben_15.09.1986.pdf` | 16 (Abschrift der Tonbandaufnahme eines Vortrags in der EPK-Reihe, Schwerpunkt Rolle der Frau; Datum auf Seite 1 markiert) |
| `Psychiatrie_im_täglichen_Leben_15.09.1986_2.pdf` | 19 (überarbeitete Reinschrift desselben Vortrags, Seitenzahlen unten) |
| `Genuss_ohne_Sucht_Motivation_zum_Leben_als_Prävention_13.11.1986.pdf` | 6 (Vortrag an der GDI-Tagung «Suchtverhalten in der Wohlstandsgesellschaft», mit der SGGP; Titelblatt und Inhaltsverzeichnis des Tagungsbands vorn, auf Wunsch nicht anonymisiert) |
| `Bedeutung_von_Randgruppen_in_unserer_Gesellschaft_20.11.1986.pdf` | 3 (Vortrag, Clubabend der Badener Berufs- und Geschäftsfrauen, Hotel Du Parc Baden; Stichworte) |
| `Sozialpsychiatrie_11.12.1986.pdf` | 2 (Fortbildung PKK, Stichworte) |
| `Vernissage_der_Malgruppe_im_Kurzentrum_in_Rheinfelden_06.01.1987.pdf` | 2 (eigenes Stichwortblatt zur selben Vernissage, Belsola-Galerie; «Vortrag vom 6.1.87») |
| `Vernissage_Malgruppe_Rheinfelden_Ansprache_06.01.1987.pdf` | 2 (Ansprache zur Vernissage der Malgruppe der Gesundheits-Beratungsstelle Rheinfelden; ohne Datum, Datum aus der Vortragsliste 1986–1988 («Was verbindet Malen u. Leben») und dem Stichwortblatt zur selben Vernissage; Adresse und Telefon im Briefkopf entfernt; auf Seite 2 aufgeklebter Zeitungsartikel von Willy Oser, «Die Kunst als heilende Kraft») |
| `Emotioneller_Prozess_in_der_Kernfamilie_Möglichkeiten_zur_02.02.1987.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs I; «… Möglichkeiten zur Stressabsorption oder Adaptionsmechanismen einer Familie auf Überlastungssituationen») |
| `Machtkampf_in_der_pflegerischen_Beziehung_05.02.1987.pdf` | 3 (Fortbildung SBK, mit Rollenspielen und Plenumsdiskussion) |
| `Angst_ist_ein_schlechter_Lehrmeister_fürs_Leben_26.03.1987.pdf` | 3 (Vortrag, Mütterberatung des Bezirks Laufenburg in Frick; Stichworte) |
| `Angst_Schuld_Sucht_07.05.1987.pdf` | 3 (Vortrag, Frauenverein Kaisten; Stichworte) |
| `Umgang_mit_schwierigen_Mitarbeitern_08.05.1987.pdf` | 2 (Vortrag bei Coop Schweiz, Muttenz; Stichworte) |
| `Wie_gehe_ich_mit_psychisch_Kranken_um_04.06.1987.pdf` | 2 (Vortrag, Frauenverein Rheinfelden) |
| `Wo_liegen_mögliche_Ansatzpunkte_für_die_Sozial-_und_11.06.1987.pdf` | 4 («… Präventivmedizin», SPD-Tagung über Prävention; Text datiert 4.6.1987; Überlauf eines Doppeleinzugs auf Seite 2 übermalt) |
| `List_of_Participants_Visions_of_Higher_Education,_Dalian_03.08.1987.pdf` | 2 (Teilnehmerliste der Third International Conference «Visions of Higher Education – Trans-National Dialogues», 3.–7.8.1987, Dalian, Gottlieb Duttweiler Institut; zwei Blätter beidseitig; auf Wunsch des Benutzers ungekürzt mit Adressen, öffentliche Liste) |
| `How_can_global_perspectives_be_taught_and_learned_Global_04.08.1987.pdf` | 7 (englisch, «… Global versus territorial thinking», Weiterbildungstagung Dalian, China; Text datiert 6.1.1988; mit `--sprache eng` gelesen) |
| `How_can_global_perspectives_be_taught_and_learned_Global_04.08.1987_2.pdf` | 7 (zweites Exemplar, Kopf anders gesetzt, Datum von Hand und Randnotiz «schon geschrieben + E-mail») |
| `Angst_Schuldgefühle_Sinnfindung_29.08.1987.pdf` | 3 (Tagung für Eltern von drogenabhängigen und drogengefährdeten Jugendlichen, 29./30.8.1987, Tagungszentrum Boldern, VEV-DAJ; Text datiert 7.9.1987) |
| `Warum_sucht_die_heutige_Jugend_so_oft_die_Flucht_in_die_20.09.1987.pdf` | 11 (Vortrag in Widen, Abschrift; die Vortragsliste 1986–1988 nennt den 24.9., Freisinnig-Demokratische Partei) |
| `Sogenannt_verhaltensgestörte_resp._verhaltensauffällige_25.09.1987.pdf` | 3 («… Schüler», Vortrag an der Lehramtschule Liebegg; Seite 2 nur wenige Zeilen) |
| `Ausländerfamilien_28.09.1987.pdf` | 2 (Familientherapiekurs, Schreibmaschine; ein Blatt beidseitig, Rückseite gedreht) |
| `Schichtung_Klasse_26.10.1987.pdf` | 4 (Kursunterlage von B. Baumann, Kurs in Familientherapie, über soziale Ungleichheit und Schichtmodelle) |
| `Das_Konzept_des_cut-off-Verhaltens_in_einer_Beziehung_oder_29.10.1987.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1987, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Frauen_in_der_Lebensmitte_02.11.1987.pdf` | 2 (Vortrag in Gipf-Oberfrick, Frauenverein) |
| `Differenzierungsprozess_von_der_Ursprungsfamilie_09.11.1987.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1987, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Differenzierungsprozess_von_der_Ursprungsfamilie_09.11.1987_2.pdf` | 2 (zweite Fassung, enger gesetzt, anderer Seitenumbruch; Text gleich) |
| `Die_Dreieckbeziehung_oder_Triangle_nach_Murray_Bowen_09.11.1987.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1987, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Beziehungen_Bezugsprobleme_25.11.1987.pdf` | 2 («Schwierigkeiten im Umgang mit Teenager», Vortrag ELPOS Aargau; ein Blatt beidseitig) |
| `Vorbereitung_der_Eltern_auf_die_Ablösungsphase_13.01.1988.pdf` | 3 (Vortrag bei ELPOS; Datum im Kopf, Text datiert 11.1.1988; die Verzeichnisse 1988 nennen den 13.3. bzw. 9.3.) |
| `Süchte_Suchtprobleme_13.01.1988.pdf` | 1 (Vortrag bei ELPOS, Stichworte; Text datiert 25.1.1988; Randnotiz von Hand) |
| `Wie_bewahre_ich_meine_Frische,_Zuversicht,_Phantasie_und_14.01.1988.pdf` | 2 («… Kraft in der Betreuung von Langzeitpatienten?», Vortrag Spitex-Stützpunkt Leuggern; ein Blatt beidseitig; am Schluss «11.01.1987», wohl verschrieben) |
| `Aufteilung_der_verschiedenen_Themen_in_Familientheorie_und_07.03.1988.pdf` | 1 (Fortbildung in Familientheorie und Familienberatung, II. Teil; Themenübersicht, Schreibmaschine, geschrieben am 9.2.1988) |
| `Gerechtigkeit_07.03.1988.pdf` | 2 (Kurstext von Ch. Hofmann, Kurs in Familientheorie; frühe, rein theoretische Fassung von «Gerechtigkeit in Beziehungen») |
| `Psychiatrie_im_täglichen_Leben_10.03.1988.pdf` | 3 (Vortrag EPK in Windisch) |
| `Eltern_sind_gleichzeitig_Führer_und_Partner_ihrer_Kinder_wie_16.03.1988.pdf` | 4 (Vortrag, Mütterberatung und Pro Juventute des Bezirks Brugg) |
| `Gesundheitsschwester_wo_ist_ihr_Platz_im_heutigen_18.03.1988.pdf` | 2 (Vortrag am Weiterbildungs- und Klassentreffentag der Gesundheitsschwestern, Kurs 7; ein Blatt beidseitig; Text datiert 17.3.1988) |
| `Psychiatrie_im_täglichen_Leben_24.03.1988.pdf` | 2 (Vortrag EPK in Baldingen; ein Blatt beidseitig) |
| `Soziale_Vererbung_oder_Weitergabe_von_menschlichen_26.03.1988.pdf` | 3 (Workshop Familientherapie, 6. Orientierungstagung «Bedeutende Psychotherapieformen der Gegenwart»; Datum am Schluss, im Verzeichnis 1988 der 25.3.; Seite 3 leere Genogramm-Vorlage im Querformat) |
| `Einige_Gedanken_zur_Zusammenarbeit_zwischen_15.04.1988.pdf` | 2 («… zwischen Gemeindesozialwesen und SPD», Arbeitstagung des Gemeindeschreiber-Vereins des Bezirks Brugg in der PKK; Text datiert 12.4.1988) |
| `Psychiatrie_im_täglichen_Leben_oder_Psychiatrie_in_27.04.1988.pdf` | 2 (Vortrag EPK, kath. Landfrauen Wettingen; Datum handschriftlich) |
| `Depressionen_05.05.1988.pdf` | 2 (Vortrag, 15. Generalversammlung des Haus- und Krankenpflegevereins Rein; Stichworte) |
| `Das_Tellereisen_06.06.1988.pdf` | 2 (Kursarbeit von K. Bachmann, Kurs in Familientheorie, zu Walther Kauers Roman «Tellereisen») |
| `Wo_steht_der_Patient_zwischen_Krankenkasse,_Arzt_und_Staat_17.06.1988.pdf` | 5 (Vortrag an der Jahresdelegiertenversammlung der Schweizerischen Krankenkasse Zurzach; schräg kopierte Vorlage, alle Seiten gerade gerichtet) |
| `Wo_steht_der_Patient_zwischen_Krankenkasse,_Arzt_und_Staat_17.06.1988_2.pdf` | 7 (zweite Fassung in Schreibmaschinensatz, Randnotiz von Hand; Seiten 6 und 7 Schemata «Teufelsspirale» und «autoregulatives System» im Querformat) |
| `Der_Psychiatriepatient_aus_der_Sicht_des_Arztes_23.06.1988.pdf` | 4 (Entwurf für ein Informationsblatt für Eltern, handschriftlich «für neues Infoblatt»; zwei Blätter beidseitig) |
| `Das_Fehlen_eines_aktiven_Erziehungspartners_19.07.1988.pdf` | 1 (Kurs in Familientheorie, Schreibmaschine) |
| `Gedanken_zum_Thema_Ablösung_von_Alleinerziehern_15.08.1988.pdf` | 1 (Kurstext von L. Müller, Kurs in Familientherapie) |
| `Psychosomatische_Erkrankungen_am_Beispiel_von_15.08.1988.pdf` | 1 (Kurstext von M. Schlaepfer: «… Herz-Kreislauferkrankungen, Störungen des Verdauungsapparates, funktionelle Störungen des Bewegungsapparates») |
| `Wie_kläre_ich_meine_Kinder_auf_über_Sexualverbrechen,_ohne_01.09.1988.pdf` | 2 (Vortrag vom 1.9.1988, «… ohne ihnen Angst zu machen?»; Text datiert 30.8.1988) |
| `Wohnheime_Übergangslösung_oder_Dauerghetto_08.09.1988.pdf` | 2 (Arbeitsgruppe am Jubiläumssymposium «10 Jahre Sozialpsychiatrische Universitätsklinik Bern», 8.–10.9.1988; ein Blatt beidseitig) |
| `Psychiatrische_Probleme_beim_Zusammenleben_von_Jung_und_Alt_29.09.1988.pdf` | 2 (Vortrag EPK in Frick; ein Blatt beidseitig) |
| `Asthma_in_der_Familie_Welches_therapeutische_Vorgehen_03.10.1988.pdf` | 2 (Kurs II in Familientheorie, Schreibmaschine; «… schlagen wir vor?») |
| `Aussterbende_Familien_Welches_therapeutische_Vorgehen_wäre_03.10.1988.pdf` | 1 (Kurs II in Familientheorie, Schreibmaschine; «… um Familien vor dem Aussterben zu bewahren») |
| `Aussterbende_Familien_03.10.1988.pdf` | 1 (Kurstext von Lotti Höner zum gleichen Kursthema, Kurs in Familientheorie 3.10.1988) |
| `Adoptiv-_und_Stieffamilien_28.10.1988.pdf` | 6 (Kursarbeit von M. Friedli, Herbst 1988; nur das Jahr, Tag und Monat vom Scan) |
| `Differenzierung_in_Bilderbuchfamilien_11.1988.pdf` | 3 (Kursarbeit von Ruth Buser, November 1988; Monat und Jahr im Text) |
| `Gerechtigkeit_in_Beziehungen_11.1988.pdf` | 5 (Kursarbeit von Ch. Hofmann, November 1988; Monat und Jahr im Text; Patientenname in Text und handgezeichnetem Genogramm durch erfundenen ersetzt) |
| `Vorträge_und_Veröffentlichungen_09.11.1988.pdf` | 2 (Verzeichnis «Vorträge und Veröffentlichungen U. Davatz 1988», 20 Einträge Januar bis Juni; nur das Jahr, Tag und Monat vom Scan) |
| `Vortragsliste_1986_bis_1988_09.11.1988.pdf` | 3 (Tabelle Datum/Gesellschaft/Thema, Januar 1986 bis März 1988; ohne Titel und Datum, die 1988er Einträge von Hand gestrichen; Jahr des letzten Eintrags, Tag und Monat vom Scan) |
| `Familientherapeutische_Interventionen_in_der_22.11.1988.pdf` | 4 (Kurs in Familientheorie II, Schreibmaschine; Datum vom Schluss des Skripts) |
| `Der_Ablösungs-_und_Trennungsprozess_bei_Affenkindern_05.12.1988.pdf` | 2 (Kurstext von D. Faust) |
| `Emigration_Geschichte_der_Emigration,_speziell_der_06.03.1989.pdf` | 8 (Kursarbeit von G. Petrazzo, Familientherapie Kurs II; mit Beiblatt zu den Redeverboten des Bundesrats; Seite 6 lag doppelt) |
| `Emigration_Geschichte_der_Emigration,_speziell_der_06.03.1989_2.pdf` | 7 (überarbeitete Fassung, dichter gesetzt; Seite 7 Literaturliste im Querformat) |
| `Emanzipation_eines_Partners_und_die_Folgen_auf_die_18.04.1989.pdf` | 2 (Referat von F. Haller, Kurs II Familientherapie; vier anonyme Paarfälle; Adresse entfernt) |
| `Cut-off_29.05.1989.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1989: Cut-off als Problemlösungsstrategie) |
| `Prävention_im_Kanton_Aargau_21.06.1989.pdf` | 9 (Schreibmaschine, Datum am Schluss; Adressen und Telefonnummern der Beratungsstellen auf Seite 9 entfernt) |
| `Prävention_sinnvoller_Ansatz_bei_gefährdeten_Familien_mit_03.07.1989.pdf` | 3 (Kursarbeit von E. Maarse Bäbler, Familientheoriekurs II; ein Blatt beidseitig) |
| `Wie_schütze_ich_mein_Kind_vor_Sexualverbrechen_22.08.1989.pdf` | 6 (Vortrag, Gruppe junger Mütter Untersiggenthal; Datum und Veranstalter handschriftlich auf Seite 1, Seitenzahlen oben) |
| `Loyalität_und_Familie_04.09.1989.pdf` | 6 (Kursarbeit von Canzek, Kurs II in Familientherapie) |
| `Krankheit_als_Anpassung_28.10.1989.pdf` | 7 (Referat von Jakob Wehrli, Kurs II/89 Familientherapie; zwei Blätter beidseitig, Seitenzahlen oben; nur das Jahr im Text, Tag und Monat vom Scan) |
| `Führungsposition_im_Zusammenhang_mit_Geschwisterposition_05.11.1989.pdf` | 3 (Familienberatung Kurs II; Arbeit der Kursteilnehmerin Heidi Deplazes-Kunz nach Walter Toman, Schreibmaschine) |
| `Adoleszentenpsychiatrie_Prävention_im_Adoleszentenalter_03.09.1990.pdf` | 2 (Familientherapiekurs II, Schreibmaschine) |
| `Sexualverhalten_Sexualstörungen_16.10.1990.pdf` | 4 (Kursskript «FT II», Schreibmaschine; Datum vom Schluss) |
| `Meine_Familie_aus_systemischer_Sicht_von_der_Herkunft_bis_12.1990.pdf` | 9 (Fragebogen «… bis heute», Dezember 1990, nach Unterlagen von Sepp Duss-von Werdt; Seiten 8-9 Beilagen im Querformat: Symbole zur Familienstruktur, Beziehungsrad) |
| `Literaturliste_für_Kurs_in_Familientheorie_25.02.1991.pdf` | 2 (Literaturliste zum Kurs in Familientheorie) |
| `Ablösung_von_der_Autorität_28.10.1991.pdf` | 2 (Familientheorie Kurs II/91, FTH291, Schreibmaschine; nur das Jahr im Text, Tag und Monat vom Scan) |
| `Anpassungsmechanismen_innerhalb_der_Kernfamilie_in_29.10.1991.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1991, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Soziale_Vererbung_in_menschlichen_Systemen_oder_Weitergabe_29.10.1991.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs I 1991, nur das Jahr im Text, Tag und Monat vom Scan; Seite 4 Muster-Genogramm im Querformat) |
| `Soziale_Vererbung_in_menschlichen_Systemen_oder_Weitergabe_29.10.1991_2.pdf` | 3 (zweite Fassung, anders gesetzt, ohne Genogramm-Vorlage; Text gleich) |
| `Scheidungsfamilien_Zusammengewürfelte_Familien_29.10.1991.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1991, nur das Jahr im Text, Tag und Monat vom Scan; Seite 2 deckt sich mit Seite 2 von «Das gesunde Cut-off») |
| `Überbrückung_vom_Cut-off_in_meiner_Herkunftsfamilie_12.1991.pdf` | 5 (Kursarbeit von Pasquale Carucci, Familientheorie-Kurs II, Dezember 1991; Monat und Jahr im Text; Adresse entfernt) |
| `Schuldgefühle_Funktion_und_Dysfunktion_02.12.1991.pdf` | 1 (Kurstext von V. M., Schreibmaschine; Datum am Textende) |
| `Vom_Umgang_mit_Cut-off_(Beziehungsabbruch)_und_dessen_29.10.1992.pdf` | 3 (anonyme Kursarbeit, Familientheorie II 1992, nur das Jahr im Text, Tag und Monat vom Scan; mit Handnotizen) |
| `Anorexia_09.11.1992.pdf` | 1 (Skript von U. Davatz, Familientherapie-Kurs I 1992, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Die_Zweierbeziehung_in_der_Ehe_23.11.1992.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1992) |
| `Soziale_Vererbung_Wie_kann_ich_diesen_Prozess_unterbrechen_25.10.1993.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1993; Datum am Textende) |
| `Weshalb_habe_ich_einen_18_Jahre_älteren_Mann_geheiratet_28.10.1993.pdf` | 2 (persönliche Kursarbeit, Familientherapie 1993; Name der Verfasserin geändert; nur das Jahr im Text, Tag und Monat vom Scan) |
| `Arbeits-_und_Freizeitplanung_innerhalb_der_Familie_28.10.1993.pdf` | 3 (leerer Fragebogen von Eva Zimmerli, Familientheorie-Kurs «FT II/93», nur das Jahr im Text, Tag und Monat vom Scan; gehört zur Arbeit vom 30.1.1994) |
| `Das_gesunde_Cut-off_29.10.1993.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1993, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Abschlussarbeit_Kurs_II_Familientheorie_und_-beratung_30.12.1993.pdf` | 8 (Abschlussarbeit eines Kursteilnehmers über die eigene Familie; Namen, Wohnort und Jahreszahlen durch erfundene ersetzt, Adresse entfernt; Seite 3 Genogramm im Querformat) |
| `Der_Einfluss_des_Berufes_des_Mannes_auf_die_Frau_und_auf_die_30.01.1994.pdf` | 3 (Kursarbeit von Eva Zimmerli, FT II/93, Auswertung ihres Fragebogens zur Arbeits- und Freizeitplanung) |
| `Einleitung_zur_Dissertation_von_Dieter_Hasler_04.05.1994.pdf` | 15 (zu einer Studie über Schizophreniefamilien und Erziehungsstil) |
| `Soziale_Vererbung_09.11.1994.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1994, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Kommunikation_unter_Spannungssituationen_09.11.1994.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1994, nur das Jahr im Text, Tag und Monat vom Scan; Teil III fehlt in der Nummerierung) |
| `Stichworte_zum_Vortrag_für_Lehrertagung_07.1995.pdf` | 3 (drei Stichwortblätter «Stichworte zu Vortrag», «Vortrag für Lehrertagung», «Vortragsthemen», je datiert Juli 1995; ein beiliegender Abzug der «Fragen für Seminar an Lehrer» als Dublette verworfen) |
| `Seminar_Einflussnahme_des_Schulsystems_auf_neue_Formen_der_10.11.1995.pdf` | 2 (Seminarausschreibung und Fragen an Lehrer; nur das Jahr, Tag und Monat vom Scan) |
| `Braucht_eine_neue_Generation_eine_neue_Pädagogik_10.11.1995.pdf` | 3 (Tagung ELPOS, im Kopf «1995 (?)»; Untertitel «Einflussnahme des Schulsystems auf psychosoziale Erkrankungen der Jugend»; nur das Jahr, Tag und Monat vom Scan) |
| `Lehrerfortbildung_Der_glückliche_Lehrer_10.11.1995.pdf` | 4 (Übersicht zur Lehrerfortbildung im Ausland und Stichworte zum Tagungsbeitrag; im Kopf «1995 (?)», Tag und Monat vom Scan) |
| `Emotionelle_Hintergründe_zum_Lernprozess_10.11.1995.pdf` | 3 (Fassung für Lehrer, Text datiert 1995 am Schluss, Tag und Monat vom Scan) |
| `Welche_Hilfe_braucht_der_Lehrer_10.11.1995.pdf` | 3 (Stichwortblatt «Welche Hilfe braucht der Lehrer?», 1995, dazu Notizen zur «Lehrertagung in Montreux (?)» und zu einer Enquete von Dr. Manor; nur das Jahr, Tag und Monat vom Scan; ein beiliegender Abzug der «Stichworte zu Vortrag» als Dublette verworfen) |
| `Migrationsprobleme_in_der_Psychiatrie_15.05.1996.pdf` | 4 (Vortrag von U. Davatz, Supervision Psychiatrischer Stützpunkt Biel; Seiten in vier Durchgängen eingezogen) |
| `Elternberatung_von_Schizophrenen_25.11.1996.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I; Datum am Textende) |
| `Behandlungsansätze_bei_Erwachsenen_mit_ADS_15.11.1997.pdf` | 9 (Vortrag von Dr. med. Johanna Krause für die Elterninitiative zur Förderung hyperaktiver Kinder, Forchheim; Postfach-Adressen und Faxnummer entfernt; Seite 7 einzeln nachgescannt, weil der Einzug sie mit Seite 6 zog) |
| `Die_soziale_Vererbung_19.01.1998.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs I 1998; Datum am Textende) |
| `Suchtmechanismen_und_Essen_27.07.1998.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs 1998; Datum am Textende) |
| `Therapie_von_Essstörungen_31.08.1998.pdf` | 3 (Skript von U. Davatz, ohne Kursangabe; Datum am Textende) |
| `Differenzierung_von_der_Ursprungsfamilie_25.10.1998.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1998; Datum am Textende) |
| `Differenzierung_von_der_Ursprungsfamilie_25.10.1998_2.pdf` | 3 (zweite Fassung, neu gesetzt: Kopf zentriert, Seitenzahlen; Text gleich) |
| `Fusionsbeziehung_oder_Symbiose_09.11.1998.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1998, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Streit_unter_Geschwistern_19.11.1998.pdf` | 3 (Vortrag von U. Davatz, Elternbildung Reinach; Mailadresse im Briefkopf entfernt) |
| `Familien_mit_Kind_mit_schwerer_Behinderung_oder_Krankheit_30.11.1998.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 1998) |
| `Ablösungskonflikt_von_betagten_Eltern_mit_Starrköpfigkeit_06.09.1999.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 1999, 2. Hälfte; «… oder Loyalitätskonflikt zwischen den Generationen»; Seite 3 einzeln nachgescannt, weil der Einzug sie mit Seite 2 zog) |
| `Depression_als_Verliererkrankheit_oder_Totstellreflex_27.09.1999.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I) |
| `Inzest_25.10.1999.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I/1999; Datum am Textende) |
| `Konstruktiver_Umgang_mit_Aggression_und_Gewalt_im_01.02.2000.pdf` | 4 (Weiterbildung am KSB; «… im Pflegealltag») |
| `Was_bewirkt_das_Auftreten_einer_lebensbedrohlichen_Krankheit_27.02.2000.pdf` | 9 (Jahresarbeit Kurs II / 1999 einer Ärztin, Exemplar in Originalgrösse, mit Beilage von zehn Fallbeispielen; Namen, Initialen und Geburtsdaten der Patienten und Angehörigen durch erfundene ersetzt) |
| `Meine_Themen_im_Familienkurs_II_25.04.2000.pdf` | 4 (persönlicher Bericht eines Kursteilnehmers, Name auf Wunsch geändert in «Bernhard Berger»; Datum handschriftlich) |
| `Adoptivkinder,_Adoptiveltern_31.07.2000.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I/2000) |
| `Ethnisch_gemischte_Familie_28.08.2000.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 2000) |
| `Geschwisterposition_29.10.2000.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs I 2000; nur das Jahr im Text, Tag und Monat vom Scan) |
| `Patchworkfamilie_09.11.2000.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I/2000; nur das Jahr im Text, Tag und Monat vom Scan; Teil II fehlt schon in der Vorlage) |
| `Chronische_Krankheit_in_der_Familie,_Gewalt_und_Alkohol_in_18.12.2000.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 2000; «… in der Familie»; Datum am Textende) |
| `Psychiatrische_Krankheit_und_sozialer_Abstieg_07.03.2001.pdf` | 5 (BDK; Blätter 2 bis 4 einzeln nachgescannt) |
| `Freizeit_und_Alkohol_08.03.2001.pdf` | 3 (Präventionsgruppe Muri; Überlauf von Blatt 3 unten auf Seite 2 übermalt, Blatt 3 einzeln nachgescannt) |
| `Krisenintervention_bei_Schizophrenieerkrankungen_unter_15.03.2001.pdf` | 5 (Fachtagung Psychiatrische Klinik Wil; Blätter 4 und 5 nachgescannt) |
| `Psychologie_der_16jährigen_27.03.2001.pdf` | 4 (Berufswahlunterricht BFF Bern; am Schluss Vermerk «Fallbeispiele», die nicht beiliegen) |
| `Ablösung_von_POS-Kindern_in_der_Pubertät_29.03.2001.pdf` | 4 (ELPOS Suhr; Blatt 3 einzeln nachgescannt) |
| `Gedanken_zum_monumentalen_Denkmal_und_Städtebau_anhand_von_05.2001.pdf` | 1 (nur Monat und Jahr) |
| `Früherkennung_der_chronischen_Krankheit_Sucht,_wie_nimmt_der_03.05.2001.pdf` | 3 (Mitgliederversammlung AVS Brugg) |
| `Warum_so_viele_Ess-Störungen_in_der_heutigen_21.05.2001.pdf` | 5 (Kath. Frauenverein und Gemeinnütziger Frauenverein; frühere Fassung desselben Texts wie am 8.3.2003; Überlauf von Blatt 4 unten auf Seite 3 übermalt, Blatt 4 einzeln nachgescannt) |
| `Wie_erkennen_wir_die_Vorzeichen_einer_beginnenden_21.06.2001.pdf` | 5 (Hausärzte-Seminar «Früherfassung und Frühbehandlung der Schizophrenie», Stadtcasino Baden; mit Programmblatt; Blatt 5 einzeln nachgescannt) |
| `Strukturerkennung_innerhalb_der_Familie_Opferrolle_25.06.2001.pdf` | 2 (Skript von U. Davatz, Familientherapiekurs I/2001) |
| `Das_POS-ADD_Kind,_eine_Herausforderung_für_Eltern_und_Lehrer_20.08.2001.pdf` | 4 (Elternvereinigung Mutschellen; Blatt 3 einzeln nachgescannt) |
| `Empowerment_von_Schmerzpatienten_27.08.2001.pdf` | 3 (Vortrag von U. Davatz, Familienkurs I) |
| `Magersucht_und_andere_Essstörungen_18.09.2001.pdf` | 5 (La vida, Wohlen; Blätter 4 und 5 einzeln nachgescannt) |
| `Suchtverhalten_im_Essbereich_20.09.2001.pdf` | 4 (Schule Laufenburg, Aula des Schulhauses Blauen) |
| `Erwachsene_mit_POS_ADS_ADHD_27.09.2001.pdf` | 4 (Workshop CP-Zentrum Solothurn; Blätter 3 und 4 einzeln nachgescannt) |
| `Was_können_Eltern_zur_Suchtprävention_beitragen_18.10.2001.pdf` | 3 (Elternforum Schöftland) |
| `Literaturliste_Suchtprävention_18.10.2001.pdf` | 1 (fremdes Material: Literaturliste der Mediothek von Sucht-Info Aargau, Beilage zum Elternforum Schöftland; Datum vom Vortrag übernommen; Strasse, Telefon, Fax und Mail übermalt) |
| `Depressionen,_der_medizinische_Aspekt_22.10.2001.pdf` | 4 (Weiterbildung für Pharmaassistentinnen, Gruppe Aargau, Eli Lilly AG; Blatt 4 einzeln nachgescannt) |
| `POS_und_Pubertät_24.10.2001.pdf` | 3 (Elpos Nordwestschweiz; Überlauf von Blatt 3 unten auf Seite 2 übermalt, Blatt 3 einzeln nachgescannt) |
| `Frauenrollen,_Frauen_sollen,_Frauen_wollen_25.10.2001.pdf` | 4 (Frauenverein Egliswil, Frauenzmorge; Blatt 4 einzeln nachgescannt) |
| `Religion_oder_die_Zugehörigkeit_zu_einem_menschlichen_System_30.10.2001.pdf` | 4 (Fortbildung Psychiatriestützpunkt Biel; Überlauf von Seite 3 auf Seite 2 übermalt) |
| `Umgang_mit_aggressiven_Patienten_02.11.2001.pdf` | 2 (Seminar für Patientenadministration, KSB; im Original «Umfang mit aggressiven Patienten») |
| `Pubertät,_loslassen_heisst_nicht_fallenlassen,_und_wer_15.11.2001.pdf` | 4 (Elternbildung Reinach; eine kleine Handkorrektur auf Seite 1) |
| `Schizophrenie_die_Familiendynamik_der_inneren_Scheidung_26.11.2001.pdf` | 3 (Skript von U. Davatz, Familienkurs I/2001) |
| `Was_ist_uns_die_Pflege_des_kranken_Menschen_noch_wert_27.11.2001.pdf` | 2 (einleitende Gedanken zum Podiumsgespräch der SGGP; endet mit der Frage nach Lösungen) |
| `Was_ist_uns_die_Pflege_des_kranken_Menschen_noch_wert_27.11.2001_2.pdf` | 4 (zweites Exemplar mit einer Seite Handnotizen zu den Voten des Panels und der Einladung der SGGP-Regionalgruppe Zürich; Adresse, Telefon, Mail und Postkonto im Briefkopf entfernt) |
| `Mütter-_und_Väterberatung_Consulting_in_menschlicher_30.11.2001.pdf` | 3 (Rede zur Diplomfeier der Mütter- und Väterberatung am JSB) |
| `Jahreszeiten_Fotografie_und_Haiku_16.12.2001.pdf` | 2 (Rede zur Vernissage der Ausstellung von Bjarne Geiges im Hans-Thoma-Museum, Bernau im Schwarzwald) |
| `Familientherapie,_wo_kann_sie_beitragen_zur_Verhinderung_18.12.2001.pdf` | 4 (Seminar Klinik Münsterlingen) |
| `Warum_wir_eine_Angehörigengruppe_machen_18.12.2001.pdf` | 2 (Material der Angehörigengruppe der Psychiatrischen Klinik Münsterlingen, Kürzel «WB», nicht von U. Davatz; gehört zum Seminar vom selben Tag; Folie im Querformat und Handzettel) |
| `Vorwort_zu_dieser_Arbeit_von_Stephan_Meyer_08.02.2002.pdf` | 1 (Vorwort zur Arbeit eines Betroffenen über POS/ADD) |
| `Ausstiegshilfen_unter_Berücksichtigung_der_Kultwellen_und_11.03.2002.pdf` | 3 (Suchtpräventionsveranstaltung im Gfellergut, 11./12.3.2002; «… geschlechtsspezifischen Aspekte»; Seite 1 mit Handnotizen) |
| `Begleitung_von_Menschen_mit_depressiven_Symptomen_im_14.03.2002.pdf` | 4 (Fortbildungsseminar am KSB; frühere Fassung des Seminars vom 13.2.2003) |
| `Begleitung_von_Menschen_mit_depressiven_Symptomen_im_14.03.2002_2.pdf` | 4 (weiteres Exemplar, auf Seite 3 zwei Kommas von Hand eingefügt) |
| `Mobbing_am_Arbeitsplatz_11.04.2002.pdf` | 4 (Reusspark) |
| `Verstecktes_Leiden_Suchtkranke_und_Pflegende_in_der_Spitex_18.04.2002.pdf` | 3 (Weiterbildung Spitex Gelterkinden) |
| `Umgang_mit_schwierigen_Familiensituationen_23.04.2002.pdf` | 4 (Fortbildung Spitex, Kirchgemeindehaus Neumünster Zürich; Strassenadresse entfernt) |
| `Zusammenarbeit_unter_Sozialtherapeutischen_Systemen_bei_30.04.2002.pdf` | 3 (SV Psychiatriestützpunkt Biel; «… gefährdeter Mutter-Kind Beziehung») |
| `Ist_die_menschliche_Brutpflege_in_unserer_globalisierten_15.05.2002.pdf` | 6 (Mütter- und Väterberatungsstelle des Bezirks Baden; «… Gesellschaft ernsthaft gefährdet?»; Seite 6 Illustration «Die Folgekosten sind enorm!») |
| `POS_bei_Erwachsenen,_Wie_gehe_ich_mit_dem_POS-ADD_am_22.05.2002.pdf` | 4 (Eingliederung WWB Basel; «… Arbeitsplatz um?»; Kontaktangaben im Kopf entfernt) |
| `POS_bei_Erwachsenen,_Wie_gehe_ich_mit_dem_POS-ADD_am_22.05.2002_2.pdf` | 4 (zweite Fassung mit Ganglion-Briefkopf, Mailadresse entfernt; Vortragsort «Flughafenstrasse 235» bleibt stehen) |
| `Mutterschaft_und_Krise_und_Eltern_werden_Eltern_sein_24.05.2002.pdf` | 5 (Nachdiplomkurs der Hebammen in Geburtsvorbereitung, Atelierbewegung Zürich; Strassenadresse entfernt) |
| `Loslassen,_bedingungslos_loslassen_25.05.2002.pdf` | 3 (Seminar für Eltern von Pubertierenden, Reinach) |
| `Was_bedeutet_mir_Lebensqualität_Was_ist_sie_mir_wert_04.06.2002.pdf` | 4 (Kath. Frauenbund Untersiggenthal) |
| `Umgang_mit_Ängsten_06.06.2002.pdf` | 3 (Weiterbildung Spitex Höfie in Pfäffikon) |
| `Sinn_und_Zweck_der_Mütter-_und_Väterberatung_12.06.2002.pdf` | 4 (Mübe Brugg, Delegiertenversammlung) |
| `Depression_eine_Zeiterscheinung_19.09.2002.pdf` | 4 (Kath., ref. und gemeinnütziger Frauenbund) |
| `Depression_eine_Zeiterscheinung_19.09.2002_2.pdf` | 4 (zweites Exemplar mit Handkorrekturen auf Seite 1-2, Randnotiz «Verlustreaktion») |
| `Der_Emanzipierte_Mann_24.09.2002.pdf` | 3 (La Vida) |
| `Designerdrogen,_die_modernen_chemischen_Problemlöser_der_26.09.2002.pdf` | 3 (Samariterverein Wölflinswil; «… der Jugend») |
| `Was_machen_wir_mit_den_ewig_Unverstandenen,_Abgewiesenen_16.10.2002.pdf` | 4 (Tagung Therapeutische Jurisprudenz, Workshop 3; «… Störefrieden und Querulanten, die später auch gefährlich werden können?») |
| `Vom_Paar_zur_Familie_Fruchtbarkeit_und_Unfruchtbarkeit_22.10.2002.pdf` | 3 (Praxis Claudia Lutz) |
| `Krisen_in_der_Familie_als_therapeutische_Chance,_betrachtet_30.10.2002.pdf` | 4 (Weiterbildung ZASP/SGSP «Krisen und Umgang mit Krisen»; «… durch verschiedene interkulturelle Aspekte») |
| `Wie_erkenne_ich_meine_Achillesferse_Was_sind_meine_typischen_31.10.2002.pdf` | 3 («Engagiert pflegen ohne Burnout», Beratung/Weiterbildung) |
| `Hilfe,_mein_Kind_wird_gemobbt_07.11.2002.pdf` | 4 (Elternzirkel Reinach) |
| `Pflegende_Angehörige_Dureschnufe_dank_Spitex_09.11.2002.pdf` | 3 (St. Galler Gesundheitstag) |
| `Psychisch_Kranke_in_der_Spitex_13.11.2002.pdf` | 3 (Spitex St. Gallen) |
| `Psychisch_Kranke_in_der_Spitex_13.11.2002_2.pdf` | 3 (zweites Exemplar, Seite 3 mit Handnotiz «Kontrolle, Medi-abgabe …») |
| `Unser_Teenager_wirkt_komisch,_Pubertät_oder_eine_beginnende_21.11.2002.pdf` | 5 (VASK Aargau; «… beginnende Psychose?») |
| `Einige_Worte_zur_Aufarbeitung_von_Frau_Ursula_Müller-Biondi_25.11.2002.pdf` | 1 (Ansprache zur Buchvernissage in Zürich) |
| `Therapeutische_Jurisprudenz_27.11.2002.pdf` | 3 (Fortbildung am Psychiatrie-Zentrum Biel) |
| `Erkennen_von_Triangulierung_und_wie_kommt_man_wieder_heraus_16.12.2002.pdf` | 2 (Familienkurs I, U. Davatz) |
| `Begleitung_von_Menschen_mit_depressiven_Symtomen_im_13.02.2003.pdf` | 3 (Fortbildungsseminar am KSB; «… im Akutspital», «Symtomen» wie auf der Vorlage) |
| `Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_06.03.2003.pdf` | 5 (Elternverein Muri, Mütter- und Väterberatung Bezirk Muri; derselbe Vortrag wie am 20.10.2003 in Frick) |
| `Warum_so_viele_Ess-Störungen_in_der_heutigen_08.03.2003.pdf` | 4 («… Überflussgesellschaft?») |
| `Depression_Gibt_es_das_auch_bei_geistig_behinderten_10.03.2003.pdf` | 4 (Insieme Region Baden; «… Jugendlichen und Erwachsenen?») |
| `Das_POS-Kind_in_der_Pubertät_24.03.2003.pdf` | 4 (ELPOS Zürich, Glarus, Schaffhausen) |
| `Die_Kommunikation_im_therapeutischen_Prozess_27.05.2003.pdf` | 3 (Weiterbildung Psychiatriestützpunkt Biel; Seite 2 mit handschriftlichen Randnotizen) |
| `Die_Kommunikation_im_therapeutischen_Prozess_27.05.2003_2.pdf` | 3 (überarbeitete Fassung, die Randnotizen abgetippt) |
| `Betreuung_psychisch_Kranker_in_der_Spitex_05.06.2003.pdf` | 3 (Spitex Windisch) |
| `Familiendynamik_mit_einem_POS-ADS-Kind_06.08.2003.pdf` | 7 |
| `Familiendynamik_mit_einem_POS-ADS-Kind_06.08.2003_2.pdf` | 7 (überarbeitete Fassung: Seiten 1-4 gleich, ab Seite 5 umformuliert, ohne den Absatz zur Angehörigengruppe) |
| `Das_POS-ADS-Kind_13.10.2003.pdf` | 8 (Kath. Pfarreizentrum Rampart Frick) |
| `Das_POS-ADS-Kind_13.10.2003_2.pdf` | 9 (zweite Fassung desselben Vortrags, anderer Kopf) |
| `Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_20.10.2003.pdf` | 5 (Kreis junger Mütter, Mütter- und Väterberatung Bezirk Frick) |
| `Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_20.10.2003_2.pdf` | 5 (zweite Fassung, Titel direkt unter dem Briefkopf) |
| `Depression,_eine_Frauenkrankheit_und_wie_sieht_es_bei_den_21.10.2003.pdf` | 4 (Kath. Pfarreizentrum Nussbaumen, mit Briefkopf) |
| `Welche_Chancen_bietet_die_familienergänzende_Betreuung_von_06.11.2003.pdf` | 4 |
| `Das_trotzende_Kind_Bin_ich_zu_lieb_oder_bin_ich_zu_streng_11.11.2003.pdf` | 5 |
| `Gedanken_zum_Tod_in_der_Familie_13.11.2003.pdf` | 4 |
| `Alkohol_und_Familie_20.11.2003.pdf` | 6 |
| `Alkohol_und_Familie_20.11.2003_2.pdf` | 5 (zweite Fassung mit Briefkopf) |
| `Jugendliche_und_Sucht_Freiheit_oder_Sackgasse_auf_dem_Weg_02.12.2003.pdf` | 7 |
| `Einbezug_von_Familienangehörigen_bei_Depressionen_03.12.2003.pdf` | 2 (Weiterbildung Stützpunkt Biel) |
| `Paarbeziehungen_24.02.2004.pdf` | 7 (Frauenverein Egliswil) |
| `POS_im_Kindes-_und_Erwachsenenalter_03.03.2004.pdf` | 2 (Besuchsdienst PKK; fast derselbe Text wie die erste Gebenstorfer Fassung vom 28.10.2004) |
| `Pubertät_18.03.2004.pdf` | 5 (Familienforum Münchenstein) |
| `POS_im_Erwachsenenalter_26.08.2004.pdf` | 5 (Seite 1 Kurztext «Sinnvoller Umgang mit POS/ADS im Kindes- und Erwachsenenalter» zum Abend in Gebenstorf; Seiten 2-5 derselbe Text wie die zweite Gebenstorfer Fassung vom 28.10.2004) |
| `ADS_und_Borderlinepersönlichkeitsstörung,_ihre_Auswirkung_im_30.08.2004.pdf` | 3 (Skript von U. Davatz, Familienkurs I 2004; «… im Familiensystem») |
| `Symbiose_zwischen_Mutter_und_Kind_sowie_symbiotische_22.09.2004.pdf` | 2 (Fortbildung Psychiatrisches Zentrum Biel; «… Beziehung in der Paarbeziehung»; Adresse, Telefon und Fax im Briefkopf entfernt) |
| `Depression_als_Verliererkrankheit_oder_Totstellreflex_27.09.2004.pdf` | 3 (Fassung für den Familientherapie-Kurs I 2004, Text weitgehend gleich wie 1999; Datum in der Fusszeile) |
| `POS-ADS_Sinnvoller_Umgang_damit_im_Kindes-_Und_28.10.2004.pdf` | 4 (Frauengemeinschaft Gebenstorf; «… Kindes- und Erwachsenenalter») |
| `POS-ADS_Sinnvoller_Umgang_damit_im_Kindes-_Und_28.10.2004_2.pdf` | 3 (gleicher Anlass und Titel, aber ein anderer Text: hohe Sensibilität, Impulskontrolle, Erziehung) |
| `Kompetenter_Umgang_von_Eltern_und_Lehrern_mit_POS-ADHS-Kind_24.02.2005.pdf` | 5 (Spitex Wolfwil, Pfarrschür; «… als Prävention einer Sekundärerkrankung») |
| `Jugend_und_Alkohol_05.04.2005.pdf` | 4 (Birr) |
| `Pubertät_Rollenverlust_der_Eltern_24.04.2005.pdf` | 4 (ohne Vortragsdatum; Datum aus der Fusszeile, vermutlich Druckdatum) |
| `Das_Leben_der_Frau_im_steten_Wandel_01.06.2005.pdf` | 3 (Morgentreff Gränichen) |
| `Psychisch_krank_scheininvalid_18.06.2005.pdf` | 4 (20-jähriges Jubiläum Stiftung Guyerweg) |
| `Das_Leiden_der_Familienangehörigen_und_ihr_Potential_als_25.08.2005.pdf` | 4 (VASK Aargau; «… als Co-Therapeuten») |
| `Patientenautonomie_und_Verwahrlosung_Recht_auf_Freiheit_25.10.2005.pdf` | 3 (Kantonaler Spitexverband Aargau; «… versus Pflicht zur Hygiene und Ordnung») |
| `Familien-Systemtheorie_nach_Murray_Bowen_15.11.2005.pdf` | 21 (Folien-Handout mit Inhaltsverzeichnis; Firmenlogo, Vermerk im Kopf und Firmenname in der Fusszeile entfernt) |
| `Das_POS-ADS_in_der_Pubertät_Wie_Eltern_helfen_können_17.11.2005.pdf` | 3 (ELPOS Zürich, Glarus, Schaffhausen, Zentrum Karl der Grosse Zürich) |
| `Das_POS-ADS_in_der_Pubertät_Wie_Eltern_helfen_können_17.11.2005_2.pdf` | 3 (andere Fassung: ohne das Meerschweinchen-Beispiel, Abschnitte umformuliert) |
| `Gedanken_im_Anschluss_an_die_Tagung_Therapeutische_23.03.2006.pdf` | 3 (zur Tagung Therapeutische Jurisprudenz «Homo invalidus – oder der wertlose Mensch») |
| `Das_Alter_mit_seinen_Würden_und_Bürden_06.04.2006.pdf` | 3 (Spitex Oftringen) |
| `Verwahrlosung_17.08.2006.pdf` | 2 (Spitex Fricktal; mit Handnotizen) |
| `Familien_mit_POS-Kindern_24.08.2006.pdf` | 2 (Abschlusskurs Neuromotorik) |
| `Woher_die_vielen_Ess-Störung_in_der_heutigen_Zeit_und_wie_02.09.2006.pdf` | 5 («Essverhalten und Psyche … und wie gehen wir damit um?»; überarbeitete Fassung des Vortrags vom 8.3.2003) |
| `Woher_die_vielen_Ess-Störung_in_der_heutigen_Zeit_und_wie_02.09.2006_2.pdf` | 5 (Entwurf mit Handkorrekturen, Titel noch «Warum so viele …», Seiten 3-5 stark überarbeitet) |
| `Familientherapie_nach_Bowen_mit_einer_Person_im_02.03.2007.pdf` | 2 (Weiterbildung; «… im sozial-psychiatrischen Setting») |
| `Das_volle_Nest_das_leere_Nest_Was_ist_schlimmer_Was_ist_15.03.2007.pdf` | 3 (Frauengemeinschaft Gebenstorf; «… Was ist schöner?») |
| `Schizophrenie_Autoaggression_und_Autismus_27.04.2007.pdf` | 2 (Vortrag AWZ) |
| `Referat_über_ADS_29.08.2007.pdf` | 2 (Schule Neuenhof; Seite 2 mit handschriftlicher Ergänzung) |
| `Das_Burn-out-Syndrom_27.09.2007.pdf` | 4 (Frauenzmorge Auenstein) |
| `Psychische_Krankheiten_und_ihre_Auswirkungen_auf_die_08.11.2007.pdf` | 3 (Vereinigung Schweizer Amtsvormünder; «… betroffene Person») |
| `Kinder-_Jugend-_und_Familientherapie_04.03.2008.pdf` | 4 (Vortrag BDK) |
| `Zusammenfassung_aller_6_Konzepte_von_M._Bowen_26.07.2010.pdf` | 2 (Skript von U. Davatz, Familienkurs I) |
| `Eifersucht_23.08.2010.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2010) |
| `Die_Kunst_des_Fragenstellens_25.10.2010.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2010) |
| `Das_Konzept_des_Cut-off-Verhaltens_in_einer_Beziehung_oder_09.11.2010.pdf` | 3 (Fassung des Skripts von 1987 für den Familientherapie-Kurs I 2010, nur das Jahr im Text, Tag und Monat vom Scan; Text fast gleich wie die Fassung von 2018) |
| `Innere_Beheimatung_12.2010.pdf` | 1 (persönlicher Erfahrungsbericht eines Kursteilnehmers, Monat und Jahr im Text; Name durch Pseudonym ersetzt) |
| `Flexibilisierung_der_Vaterrolle_10.2011.pdf` | 2 (persönlicher Bericht eines Kursteilnehmers über die Beziehung zu seinem Vater, Monat und Jahr im Text; Namen des Verfassers und der Tochter durch Pseudonyme ersetzt) |
| `Schuld-_und_Schamgefühle_Sühne_09.11.2011.pdf` | 3 (Skript von U. Davatz, Familienkurs I 2011, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Verhalten_in_der_Partnerbeziehung,_welche_Rückschlüsse_09.11.2011.pdf` | 3 (Skript von U. Davatz, Familienkurs I 2011; «… können auf die Ursprungsfamilie gemacht werden»; nur das Jahr im Text, Tag und Monat vom Scan) |
| `Sucht_aus_systemischer_Sicht_24.09.2012.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2012; frühere, kürzere Fassung des Skripts vom 16.12.2013; Strassenangabe der Praxis entfernt) |
| `Migration,_Migrantenfamilien_und_Clanstrukturen_22.10.2012.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2012; Teil IV in der Vorlage nur als Überschrift; Strassenangabe der Praxis entfernt) |
| `Migration,_Migrantenfamilien_und_Clanstrukturen_22.10.2012_2.pdf` | 3 (vollständige Fassung: Teil IV ausgeführt, dazu Teil V «Umgang mit Clanstrukturen»; Strassenangabe der Praxis entfernt) |
| `Geschwisterliebe_Geschwisterstreit_09.11.2012.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2012, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Repetition_Genogramme_und_ihre_Symbole_09.11.2012.pdf` | 1 (Seite 30 aus einem fremden Lernjournal «Systemische Sozialpädagogik», Aufbaukurs 2011/12, nicht von U. Davatz; Logo und Fusszeile des Anbieters entfernt, Seitenzahl belassen, schräg eingezogene Seite gerade gerichtet; Jahr 2012 mit Tag und Monat vom Scan) |
| `Psychisch_kranke_Eltern_26.11.2012.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2012; Strassenangabe der Praxis entfernt) |
| `Ablösung_erwachsener_Kinder_von_ihren_betagten_Eltern_17.12.2012.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2012; Strassenangabe der Praxis entfernt) |
| `Gewalt_im_Familiensystem_22.07.2013.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2013) |
| `Herkunftsfamilie_und_Identitätsfindung_19.08.2013.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2013) |
| `Geschwisterliebe_Geschwisterstreit_21.10.2013.pdf` | 3 (Fassung für den Familientherapie-Kurs I 2013, Text fast gleich wie 2012) |
| `Angst_Panik_Zwang_Sucht_Lüge_18.11.2013.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2013; Seite 3 einzeln nachgescannt, der Überlauf unten auf Seite 2 weiss übermalt) |
| `Sucht_aus_systemischer_Sicht_16.12.2013.pdf` | 3 (Skript von U. Davatz, Familienkurs I 2013; Seiten 2-3 mit eigenem Kopf «Suchtverhalten in Familien – Sucht und Schizophrenie») |
| `Kindheitstraumatas_heilen_ablösen_21.07.2014.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2014) |
| `Umgang_mit_Religion_18.08.2014.pdf` | 5 (Skript von U. Davatz, Familientherapie-Kurs I 2014; in fünf Durchgängen eingezogen) |
| `Patchwork_Familie_und_die_Rolle_von_Tieren_im_Familiensystem_22.09.2014.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2014) |
| `Normierungswahn_und_Perfektionismus_in_unserer_Gesellschaft_20.10.2014.pdf` | 3 (Skript von U. Davatz, Familientherapie-Kurs I 2014; «… – Auswirkungen auf das Familien-System»; der Überlauf unten auf Seite 1 weiss übermalt) |
| `Geschwisterpositionen_ihre_Vor-_und_Nachteile_21.08.2016.pdf` | 4 (Skript von U. Davatz, Familientherapie-Kurs I 2016; Seite 4 einzeln nachgescannt, der Überlauf unten auf Seite 3 weiss übermalt) |
| `Sexueller_Missbrauch_Wie_heilt_man_ihn_19.09.2016.pdf` | 4 (Skript von U. Davatz, Familientheorie-Kurs I; in vier Durchgängen eingezogen) |
| `Prozess-Management_versus_Case-Management_in_der_Medizin_und_17.10.2016.pdf` | 5 (Skript von U. Davatz, Familientheorie-Kurs I; «… insbesondere bei der Behandlung von Schizophreniepatienten»; in vier Durchgängen eingezogen) |
| `Trauma_und_Migration_21.11.2016.pdf` | 2 (Skript von U. Davatz, Familientheorie-Kurs I) |
| `Burnout_und_Gesellschaft_09.01.2017.pdf` | 2 (Skript von U. Davatz, Familientherapie-Kurs I 2016; Datum am Textende) |
| `Die_soziale_Vererbung_15.01.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1998 für den Familientherapie-Kurs I 2018, mit Links auf ganglion.ch und schizo.li) |
| `Fusionsbeziehung_oder_Symbiose_02.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1998 für den Familientherapie-Kurs I 2018; Monat und Jahr am Textende) |
| `Das_Konzept_des_Cut-off-Verhaltens_in_einer_Beziehung_oder_15.03.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1987 für den Familientherapie-Kurs I 2018; «… oder die gewaltsame Abtrennung einer Fusions-Beziehung») |
| `Die_Dreieckbeziehung_oder_Triangle_nach_Murray_Bowen_23.04.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1987 für den Familientherapie-Kurs I 2018) |
| `Angst,_Panikattacken_und_Zwangsverhalten_03.09.2018.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2018; Praxisadresse, Telefon und Mail entfernt) |
| `Ambivalenz_und_Pattsituation_15.10.2018.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2018; Praxisadresse, Telefon und Mail entfernt) |
| `Anpassungsmechanismen_innerhalb_der_Kernfamilie_in_09.11.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1991 für den Familientherapie-Kurs I 2018, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Differenzierung_von_der_Ursprungsfamilie_09.11.2018.pdf` | 3 (überarbeitete Fassung des Skripts von 1998 für den Familientherapie-Kurs I 2018, nur das Jahr im Text, Tag und Monat vom Scan) |
| `Arbeiten_mit_ganzen_Familien_als_Mehrpersonensystem_12.11.2018.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2018; Praxisadresse, Telefon und Mail entfernt) |
| `Behandlung_des_Familiensystems_als_Ganzes_10.12.2018.pdf` | 2 (Skript von U. Davatz, Familienkurs I 2018; Praxisadresse, Telefon und Mail entfernt) |
| `Prevention_in_the_field_of_law_and_mental_health_28.10.2026.pdf` | 4 (englisch, «Therapeutic Jurisprudence», 28. Congress of Law and Mental Health in den Niederlanden; ohne Datum im Text, Scantag) |
| `Sind_Mütter_die_besseren_Beraterinnen_28.10.2026.pdf` | 7 (Artikel für «Clic», ohne Datum im Text, Scantag) |
| `Text_für_Werbung_Familientherapie_28.10.2026.pdf` | 2 (ohne Datum im Text, Scantag) |
| `Was_und_wieviel_kann_die_Mütter-_und_Väterberatung_zur_28.10.2026.pdf` | 8 (Artikel für «Clic», ohne Datum im Text, Scantag) |
| `Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026.pdf` | 2 (Artikel für die Spitex Zeitung mit Literaturhinweisen, ohne Datum im Text, Scantag; zur Fortbildung vom 23.4.2002) |
| `Automatismus_Motivation_28.10.2026.pdf` | 22 (Kursarbeit «FT II», ohne Datum, Scantag; handschriftliche Adresse entfernt) |
| `Differenzierung_im_Ehekonflikt_28.10.2026.pdf` | 7 (Seminararbeit Familientherapie Seminar II, um 1990, ohne Datum, Scantag; Adresse entfernt, Schlussseite lag doppelt) |
| `Krankheit_als_Konfliktlösung_28.10.2026.pdf` | 1 (Kursunterlage ohne Verfasser und Datum, Scantag) |
| `Das_Leiden_der_Familienangehörigen_und_ihr_Potential_als_28.10.2026.pdf` | 3 (Zeitschriftenartikel mit Lead und Autorenhinweis, ohne Datum im Text, Scantag; neuer Text zum Thema des VASK-Vortrags vom 25.8.2005) |
| `Gibt_es_das_auch_Depressionen_bei_geistig_behinderten_28.10.2026.pdf` | 3 («… Jugendlichen und Erwachsenen», ohne Datum im Text, Scantag; neuer Text zum Thema des Insieme-Vortrags vom 10.3.2003) |
| `Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026_2.pdf` | 3 (andere Fassung: grösser gesetzt, mit Autorenzeile statt Unterschrift, ohne Literaturhinweise) |
| `Familienschema_od._Familiendiagramm_od._Genogramm_29.10.2026.pdf` | 1 (leeres Muster-Genogramm mit Legende, Querformat; ohne Datum, Scantag) |
| `Psychoseerleben_in_der_Literatur_29.10.2026.pdf` | 1 (Literaturhinweise nach Asmus Finzen 1993, ohne Datum, Scantag) |
| `Übersicht_der_Kursarbeiten_Familientherapie_29.10.2026.pdf` | 2 (Liste der Kursarbeiten nach Thema mit Jahr, 1987–2001; ohne Überschrift und Datum, Scantag) |
| `ADHS_ist_keine_Krankheit_09.11.2026.pdf` | 10 (Aufsatz von U. Davatz ohne Datum, Scantag; zitiert die NZZ am Sonntag vom 15.7.2018, ist also danach entstanden; Seiten 8 und 10 einzeln nachgescannt; Praxisadresse, Telefon und Mail entfernt) |
| `ADHS_ist_keine_Krankheit_09.11.2026_2.pdf` | 10 (zweite Fassung, anders umbrochen, Text gleich; Praxisadresse, Telefon und Mail entfernt) |
| `Familie_Bowen_09.11.2026.pdf` | 1 (Genogramm der Herkunftsfamilie von Murray Bowen, Querformat, ohne Datum, Scantag) |
| `Epigenetisches_Potential_in_der_Psychiatrie_09.11.2026.pdf` | 14 (Aufsatz von U. Davatz ohne Datum, Scantag; zitiert Literatur bis 2018; «Gravierende Mängel im Denkmodell und Versorgungskonzept der Psychiatrie»; Seiten 3-5 und 12-14 in vier weiteren Durchgängen nachgescannt, der Überlauf unten auf Seite 11 weiss übermalt; Praxisadresse, Telefon und Mail entfernt) |
| `Epigenetisches_Potential_in_der_Psychiatrie_09.11.2026_2.pdf` | 13 (andere, gestraffte Fassung mit dem Untertitel «Herleitung psychischer Krankheiten aus ADS und ADHS», Textähnlichkeit etwa 0,6; ohne Datum, Scantag; in sechs Durchgängen eingezogen; Praxisadresse, Telefon und Mail entfernt) |
| `Familientherapie_bei_Schizophrenie-Kranken_09.11.2026.pdf` | 3 (Skript von U. Davatz ohne Datum, Scantag; nennt «30-jährige Erfahrung» seit 1980, also um 2010) |
| `Früherfassung_von_Psychosen-Schizophrenie_in_der_09.11.2026.pdf` | 6 (Skript von U. Davatz für Hausärzte, ohne Datum, Scantag; «… in der hausärztlichen Praxis») |
| `Emotionelle_Hintergründe_zum_Lernprozess_10.11.2026.pdf` | 8 (ausgeschriebener Text mit Fussnoten, ohne Datum, darum Scantag; vermutlich um 1986, vgl. die Stichworte vom 5.3.1986; Strasse in einer Quellenangabe entfernt) |
| `Seminar_über_Einflussnahme_der_Schule_auf_neue_psychosoziale_10.11.2026.pdf` | 2 (Seminarunterlage ohne Datum, darum Scantag; nach Satz und Thema aus der Seminarreihe um 1995) |
| `Stufenspezifische_Aspekte_der_Gesundheitserziehung_10.11.2026.pdf` | 9 (Aufsatz ohne Datum, darum Scantag; als leitende Ärztin des SPD Königsfelden gezeichnet, also nach 1981) |
| `Gesundheitserziehung_Lektionen_I_bis_IV_10.11.2026.pdf` | 9 (vier Lektionen ohne Datum und Namen, darum Scantag; gehört nach Thema zu «Stufenspezifische Aspekte der Gesundheitserziehung»; Überlauf eines Doppeleinzugs auf Seite 7 übermalt) |
| `Alkoholprobleme_innerhalb_der_Familie_10.11.2026.pdf` | 2 (Stichworte ohne Datum, darum Scantag; Schreibmaschine) |
| `Prevention_is_better_than_cure_10.11.2026.pdf` | 1 (englische Stichworte für einen Vortrag im English-Club, ohne Datum, darum Scantag; mit «--sprache eng» gelesen) |
| `Drogenvortrag_Sucht_und_Abhängigkeit_10.11.2026.pdf` | 3 (Stichworte zu einem gemeinsamen Vortrag mit Herrn Jans von der Beratungsstelle Kontakt; ohne Datum, darum Scantag) |
| `Aufzeichnung_einer_neuen_psychopathologischen_10.11.2026.pdf` | 7 (Entwurf, von Hand so bezeichnet; ohne Datum, darum Scantag; Seitenzahlen oben rechts) |
| `Krisensituation_10.11.2026.pdf` | 4 (ohne Datum, Scantag; Schreibmaschine, Bild abgedunkelt; unterstrichene Überschriften für die Texterkennung ohne Unterstreichung gelesen; Tippfehler «Bower» im Bild zu «Bowen» korrigiert) |
| `Gesundheitserziehung_Lektionen_I_bis_IV_10.11.2026_2.pdf` | 9 (zweites Exemplar, gleicher Satz; auf Seite 4 ein Absatz von Hand gestrichen und mit Leuchtstift markiert) |

### Wenn etwas fehlt oder doppelt ist

- **Weniger Blätter als angekündigt:** Die Seitenübergänge prüfen (Ende der
  einen, Anfang der nächsten Seite) und die Seitenzahlen in den Fusszeilen.
  Das fehlende Blatt einzeln nachscannen und mit `--aus` an der richtigen
  Stelle einfügen.
- **Ein Blatt doppelt:** Seiten mit fast gleichem Text (Wortvergleich über
  99 %) sind eine Kopie; sie kommt nicht ins PDF. So lag beim «POS/ADS-Kind»
  Seite 7 zweimal im Stapel.
- **Leere Seite:** Das Blatt lag verkehrt herum; das Programm warnt.
- **Derselbe Vortrag nochmals:** Eine zweite Fassung (anderer Kopf, anderer
  Umbruch) wird mit `_2` abgelegt; dieselben Blätter nochmals gescannt
  werden verworfen.

## Aufbau

| Datei | Zweck |
|---|---|
| `src/uebersicht.rs` | Satz des PDFs und Link-Overlay |
| `src/eintragen.rs` | Neuen Scan nachtragen: Zusammenfassung, Bestandszeile nach Datum, Zählzeile (`--zaehlen`) |
| `src/dateiname.rs` | Zerlegt `Titel_TT.MM.JJJJ[_N].pdf` (gemeinsam für `scanliste` und `eintragen`) |
| `src/mail.rs` | Mail mit Anhängen über die Gmail-API (OAuth-Refresh, MIME, `--trocken`) |
| `src/etiketten.rs` | Ordnerrücken «Vorträge … JJJJ – JJJJ» auf ein A4-Blatt, Breite je Schild |
| `src/pcldruck.rs` | Erste PDF-Seite als PCL-Raster an einen Drucker ohne PDF (Port 9100) |
| `src/scanliste.rs` | Übersicht der Scans mit Zusammenfassung und GitHub-Link (`Vorträge_Dr.med._Ursula_Davatz_von_…_bis_….pdf`) |
| `src/zusammenfassungen.rs` | je Scan drei bis fünf Sätze Zusammenfassung |
| `src/pdflinks.rs` | Link-Overlay, gemeinsam für beide Übersichten |
| `src/scannen.rs` | Vorträge über den Einzug des GT-2500 scannen, Datum und Titel erkennen |
| `scan/` | gescannte Vorträge als durchsuchbare PDFs |
| `src/eintraege.rs` | generierte Daten: 150 Beiträge mit Aufrufzahlen, Feld `ocr` markiert die OCR-Fälle |
| `fonts/` | DejaVu Sans, wird ins PDF eingebettet |

## Hyperlinks

`genpdf` 0.2 kennt keine Hyperlinks. Deshalb werden die URL-Zeilen als einzige
Zeilen des Dokuments in 8 pt gesetzt; nach dem Rendern legt `add_links()`
(`src/pdflinks.rs`) mit
`lopdf` über jede solche Zeile eine Link-Annotation. Sobald irgendwo sonst im
Satz diese Schriftgrösse auftaucht, verschiebt sich die Zuordnung — Kopfzeile
läuft deshalb auf 7 pt, die Metazeilen auf 9 pt. Eine Assertion bricht ab,
wenn Zeilen- und URL-Zahl auseinanderlaufen.

Dieselbe Pipeline wie in [fundaziun-davaz](https://github.com/zdavatz/fundaziun-davaz).

## Lizenz

GPL-3.0
