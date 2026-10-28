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
trotzdem entfernt.
`--duplex` zieht beidseitig bedruckte Blätter mit Vorder- und Rückseite ein.

**Warum SANE und nicht Epson Scan 2:** Dessen Argument `-SCAN` ist nicht
dokumentiert, und der Image-Capture-Treiber «EPSON Scanner.app» ist nur für
Intel gebaut und startet auf Apple Silicon ohne Rosetta nicht. SANE führt den
GT-2500 im Backend `epson2` als «ES-H300».

### Bestand

120 PDFs, 504 Seiten, nach Vortragsdatum (Scantag, wo der Text keines nennt).

| Datei | Seiten |
|---|---|
| `Ausländerfamilien_28.09.1987.pdf` | 2 (Familientherapiekurs, Schreibmaschine; ein Blatt beidseitig, Rückseite gedreht) |
| `Schichtung_Klasse_26.10.1987.pdf` | 4 (Kursunterlage von B. Baumann, Kurs in Familientherapie, über soziale Ungleichheit und Schichtmodelle) |
| `Aufteilung_der_verschiedenen_Themen_in_Familientheorie_und_07.03.1988.pdf` | 1 (Fortbildung in Familientheorie und Familienberatung, II. Teil; Themenübersicht, Schreibmaschine, geschrieben am 9.2.1988) |
| `Das_Tellereisen_06.06.1988.pdf` | 2 (Kursarbeit von K. Bachmann, Kurs in Familientheorie, zu Walther Kauers Roman «Tellereisen») |
| `Das_Fehlen_eines_aktiven_Erziehungspartners_19.07.1988.pdf` | 1 (Kurs in Familientheorie, Schreibmaschine) |
| `Gedanken_zum_Thema_Ablösung_von_Alleinerziehern_15.08.1988.pdf` | 1 (Kurstext von L. Müller, Kurs in Familientherapie) |
| `Asthma_in_der_Familie_Welches_therapeutische_Vorgehen_03.10.1988.pdf` | 2 (Kurs II in Familientheorie, Schreibmaschine; «… schlagen wir vor?») |
| `Aussterbende_Familien_Welches_therapeutische_Vorgehen_wäre_03.10.1988.pdf` | 1 (Kurs II in Familientheorie, Schreibmaschine; «… um Familien vor dem Aussterben zu bewahren») |
| `Familientherapeutische_Interventionen_in_der_22.11.1988.pdf` | 4 (Kurs in Familientheorie II, Schreibmaschine; Datum vom Schluss des Skripts) |
| `Emanzipation_eines_Partners_und_die_Folgen_auf_die_18.04.1989.pdf` | 2 (Referat von F. Haller, Kurs II Familientherapie; vier anonyme Paarfälle; Adresse entfernt) |
| `Prävention_sinnvoller_Ansatz_bei_gefährdeten_Familien_mit_03.07.1989.pdf` | 3 (Kursarbeit von E. Maarse Bäbler, Familientheoriekurs II; ein Blatt beidseitig) |
| `Führungsposition_im_Zusammenhang_mit_Geschwisterposition_05.11.1989.pdf` | 3 (Familienberatung Kurs II; Arbeit der Kursteilnehmerin Heidi Deplazes-Kunz nach Walter Toman, Schreibmaschine) |
| `Adoleszentenpsychiatrie_Prävention_im_Adoleszentenalter_03.09.1990.pdf` | 2 (Familientherapiekurs II, Schreibmaschine) |
| `Sexualverhalten_Sexualstörungen_16.10.1990.pdf` | 4 (Kursskript «FT II», Schreibmaschine; Datum vom Schluss) |
| `Meine_Familie_aus_systemischer_Sicht_von_der_Herkunft_bis_12.1990.pdf` | 9 (Fragebogen «… bis heute», Dezember 1990, nach Unterlagen von Sepp Duss-von Werdt; Seiten 8-9 Beilagen im Querformat: Symbole zur Familienstruktur, Beziehungsrad) |
| `Abschlussarbeit_Kurs_II_Familientheorie_und_-beratung_30.12.1993.pdf` | 8 (Abschlussarbeit eines Kursteilnehmers über die eigene Familie; Namen, Wohnort und Jahreszahlen durch erfundene ersetzt, Adresse entfernt; Seite 3 Genogramm im Querformat) |
| `Der_Einfluss_des_Berufes_des_Mannes_auf_die_Frau_und_auf_die_30.01.1994.pdf` | 3 (Kursarbeit von Eva Zimmerli, FT II/93, Auswertung ihres Fragebogens zur Arbeits- und Freizeitplanung) |
| `Einleitung_zur_Dissertation_von_Dieter_Hasler_04.05.1994.pdf` | 15 (zu einer Studie über Schizophreniefamilien und Erziehungsstil) |
| `Konstruktiver_Umgang_mit_Aggression_und_Gewalt_im_01.02.2000.pdf` | 4 (Weiterbildung am KSB; «… im Pflegealltag») |
| `Was_bewirkt_das_Auftreten_einer_lebensbedrohlichen_Krankheit_27.02.2000.pdf` | 9 (Jahresarbeit Kurs II / 1999 einer Ärztin, mit Beilage von zehn Fallbeispielen; Namen, Initialen und Geburtsdaten der Patienten und Angehörigen durch erfundene ersetzt) |
| `Meine_Themen_im_Familienkurs_II_25.04.2000.pdf` | 4 (persönlicher Bericht eines Kursteilnehmers, Name auf Wunsch geändert in «Bernhard Berger»; Datum handschriftlich) |
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
| `Symbiose_zwischen_Mutter_und_Kind_sowie_symbiotische_22.09.2004.pdf` | 2 (Fortbildung Psychiatrisches Zentrum Biel; «… Beziehung in der Paarbeziehung»; Adresse, Telefon und Fax im Briefkopf entfernt) |
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
| `Prevention_in_the_field_of_law_and_mental_health_28.10.2026.pdf` | 4 (englisch, «Therapeutic Jurisprudence», 28. Congress of Law and Mental Health in den Niederlanden; ohne Datum im Text, Scantag) |
| `Sind_Mütter_die_besseren_Beraterinnen_28.10.2026.pdf` | 7 (Artikel für «Clic», ohne Datum im Text, Scantag) |
| `Text_für_Werbung_Familientherapie_28.10.2026.pdf` | 2 (ohne Datum im Text, Scantag) |
| `Was_und_wieviel_kann_die_Mütter-_und_Väterberatung_zur_28.10.2026.pdf` | 8 (Artikel für «Clic», ohne Datum im Text, Scantag) |
| `Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026.pdf` | 2 (Artikel für die Spitex Zeitung mit Literaturhinweisen, ohne Datum im Text, Scantag; zur Fortbildung vom 23.4.2002) |
| `Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026_2.pdf` | 3 (andere Fassung: grösser gesetzt, mit Autorenzeile statt Unterschrift, ohne Literaturhinweise) |
| `Automatismus_Motivation_28.10.2026.pdf` | 22 (Kursarbeit «FT II», ohne Datum, Scantag; handschriftliche Adresse entfernt) |
| `Weshalb_habe_ich_einen_18_Jahre_älteren_Mann_geheiratet_28.10.2026.pdf` | 2 (persönliche Kursarbeit, Familientherapie 1993; Name der Verfasserin geändert; ohne Tagesdatum, Scantag) |
| `Differenzierung_im_Ehekonflikt_28.10.2026.pdf` | 7 (Seminararbeit Familientherapie Seminar II, um 1990, ohne Datum, Scantag; Adresse entfernt, Schlussseite lag doppelt) |
| `Krankheit_als_Anpassung_28.10.2026.pdf` | 7 (Referat von Jakob Wehrli, Kurs II/89 Familientherapie; zwei Blätter beidseitig, Seitenzahlen oben; ohne Tagesdatum, Scantag) |
| `Krankheit_als_Konfliktlösung_28.10.2026.pdf` | 1 (Kursunterlage ohne Verfasser und Datum, Scantag) |
| `Adoptiv-_und_Stieffamilien_28.10.2026.pdf` | 6 (Kursarbeit von M. Friedli, Herbst 1988; ohne Tagesdatum, Scantag) |
| `Arbeits-_und_Freizeitplanung_innerhalb_der_Familie_28.10.2026.pdf` | 3 (leerer Fragebogen von Eva Zimmerli, Familientheorie-Kurs «FT II/93», ohne Tagesdatum, Scantag; gehört zur Arbeit vom 30.1.1994) |
| `Ablösung_von_der_Autorität_28.10.2026.pdf` | 2 (Familientheorie Kurs II/91, FTH291, Schreibmaschine; ohne Tagesdatum, Scantag – der Kurs deutet auf 1991) |
| `Das_Leiden_der_Familienangehörigen_und_ihr_Potential_als_28.10.2026.pdf` | 3 (Zeitschriftenartikel mit Lead und Autorenhinweis, ohne Datum im Text, Scantag; neuer Text zum Thema des VASK-Vortrags vom 25.8.2005) |
| `Gibt_es_das_auch_Depressionen_bei_geistig_behinderten_28.10.2026.pdf` | 3 («… Jugendlichen und Erwachsenen», ohne Datum im Text, Scantag; neuer Text zum Thema des Insieme-Vortrags vom 10.3.2003) |

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
| `src/scannen.rs` | Vorträge über den Einzug des GT-2500 scannen, Datum und Titel erkennen |
| `scan/` | gescannte Vorträge als durchsuchbare PDFs |
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
