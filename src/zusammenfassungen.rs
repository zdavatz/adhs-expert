// Zusammenfassungen der gescannten Vorträge in scan/, je PDF drei bis fünf
// Sätze. Gelesen von `scanliste`; nach jedem neuen Scan hier nachtragen.
//
// Lizenz: GPL-3.0

pub struct Zusammenfassung {
    pub datei: &'static str,
    /// Voller Titel, wo der Dateiname ihn kürzt oder Satzzeichen fehlen; leer: aus dem Dateinamen
    pub titel: &'static str,
    pub text: &'static str,
}

pub const ZUSAMMENFASSUNGEN: &[Zusammenfassung] = &[
    Zusammenfassung {
        datei: "Ablösung_von_der_Autorität_28.10.1991.pdf",
        titel: "",
        text: "Die Funktion der männlichen Autorität in der patriarchalen Familie steht im Zentrum dieses Kursskripts von U. Davatz (Familientheorie Kurs II/91): Sie ordnet und kontrolliert nach innen und schützt die Familie nach aussen. Setzt sich ein adoleszentes Kind mit dieser Autorität auseinander, bleiben ihm vier Möglichkeiten: sich fügen, sich durch Rückzug oder Weglaufen entziehen (Cut-off), den Konflikt austragen und dabei in Kontakt bleiben (Differenzierung) oder die Autorität zerstören. Die fruchtbare Auseinandersetzung führt nach Bowen zur Entwicklung einer eigenen Autorität mit eigenen Wertvorstellungen. Bei zu starker oder fehlender Autoritätsfigur, etwa im antiautoritären Erziehungsmuster, fehlt dem Kind ein Gegenüber für diese Auseinandersetzung.",
    },
    Zusammenfassung {
        datei: "Abschlussarbeit_Kurs_II_Familientheorie_und_-beratung_30.12.1993.pdf",
        titel: "",
        text: "Abschlussarbeit eines Kursteilnehmers im Kurs II «Familientheorie und -beratung» (1993, Leitung U. Davatz) über die eigene Familie, aufbauend auf der Bowen-Theorie. Sie enthält ein Familienschema über drei Generationen, Porträts von Eltern, Geschwistern und der eigenen Kernfamilie samt den vermittelten Werten und Haltungen sowie Cut-offs zu Geschwistern. Auffällige Todesursachen und Krankheitsbilder, etwa Herz- und Krebserkrankungen, deutet der Verfasser mit Dethlefsen und Dahlke symbolisch als «Sprache der Seele» und leitet daraus Fragen und Lösungsansätze ab. Namen, Wohnort und Jahreszahlen sind im vorliegenden Exemplar durch erfundene ersetzt.",
    },
    Zusammenfassung {
        datei: "Adoleszentenpsychiatrie_Prävention_im_Adoleszentenalter_03.09.1990.pdf",
        titel: "Adoleszentenpsychiatrie – Prävention im Adoleszentenalter",
        text: "Davatz beschreibt typische Verhaltensmerkmale des Adoleszenten (Rebellion, Autonomiesuche, Infragestellen von Werten, Schwärmen für Idolfiguren) und die Reaktionen der Erzieher: Väter neigen zum Machtkampf, Mütter und Jugendtherapeuten eher zur Verbrüderung, die bei Misserfolg in Machtkampf umschlagen kann. Mit dem Bild des «Rattenfängers von Hameln» warnt sie vor Jugendarbeitern, die Jugendliche von den Eltern wegführen, etwa in autonome Jugendzentren. Eine sinnvolle Adoleszentenpsychiatrie unterstützt und begleitet die Eltern im Ablösungskonflikt, vermischt Jugendarbeit nicht mit politischen Anliegen, hört offen zu und nutzt das kreative Potenzial der Jugend auf Erwachsenenebene. Familientherapiekurs II, 3.9.1990.",
    },
    Zusammenfassung {
        datei: "Adoptiv-_und_Stieffamilien_28.10.1988.pdf",
        titel: "",
        text: "Kursarbeit von M. Friedli über Adoptiv- und Stieffamilien als erweiterte, zusammengesetzte Familiensysteme. Bei Adoptionen geht es um kinderlose Paare mit hohen Erwartungen und Schuldgefühlen, um die Herkunft des Kindes und die Vererbungsfrage (Intelligenz, Grösse und Gewicht, Temperament, Alkoholismus), um die frühe Aufklärung über die leibliche Herkunft, die Identitätsprobleme und die Wahl des Adoptionszeitpunkts. Bei Stieffamilien werden Phasen von Abschied, Teilfamilie und neuer Partnerschaft, der Druck, sofort Kernfamilie zu sein, das Überengagement der Stiefeltern, Tabuisierung, Koalitionen und der Kontakt zum ausserhalb lebenden Elternteil behandelt. Als Therapieziele stehen: sich als Stieffamilie erkennen, Elternverantwortung für die leiblichen Kinder betonen, die Paarebene stärken, Paar- und Elternebene trennen. Mit Literaturangaben, Herbst 1988.",
    },
    Zusammenfassung {
        datei: "Adoptivkinder,_Adoptiveltern_31.07.2000.pdf",
        titel: "",
        text: "Adoptivkinder und Adoptiveltern sind das Thema dieses Skripts von U. Davatz (Familientherapie-Kurs I, 2000). Adoptiveltern erleben sich als Ersatzeltern und stehen unter Leistungsdruck, mit Ambivalenz, Schuldgefühlen und Rivalität zu den leiblichen Eltern. Adoptivkinder suchen spätestens in der Pubertät nach den leiblichen Eltern, mit Sehnsucht und Wut, und lehnen die Adoptiveltern dann besonders ab, um loyal zu bleiben. Therapeutisch sollen die Eltern in ihrem Druck ernst genommen werden und den leiblichen Eltern Respekt entgegenbringen. Die Begegnung mit ihnen, auch rein gedanklich und rituell, entlastet den Loyalitätskonflikt.",
    },
    Zusammenfassung {
        datei: "Alkohol_und_Familie_20.11.2003.pdf",
        titel: "",
        text: "Vortrag bei der Elternbildung Reinach über Alkohol und Familie. Davatz führt Alkohol als kulturspezifische Droge im Übergangsritual ein, die säkularisiert wurde und im Einzelfall zum «Sorgenkiller» und damit zum Psychopharmakon wird. Besprochen werden die Rolle des Alkohols für Jugendliche (Männlichkeitsbeweis, Alkopops), für Ehemänner (Rückzug vor Kritik, Teufelskreis mit der Partnerin) und für Frauen (heimliches Trinken, Bevormundung). Hilfreich sei die Arbeit mit der co-süchtigen Ehefrau, die Verantwortung abgibt und Fachhilfe für sich holt, das Ernstnehmen von Alkoholikerinnen und die Haltung, dass Alkohol ein Erwachsenengetränk ist, das Jugendlichen schadet.",
    },
    Zusammenfassung {
        datei: "Alkohol_und_Familie_20.11.2003_2.pdf",
        titel: "",
        text: "Zweite, redaktionell überarbeitete Fassung des Vortrags für die Elternbildung Reinach (BL), mit Briefkopf. Alkohol wird als kulturspezifische Droge für Übergangsrituale dargestellt, die sich von der Religion löste und individuell als «Sorgenkiller» dienen kann. Behandelt werden die Bedeutung für Jugendliche, Ehemänner, Frauen und die Wirkung von Alkopops. Für den Umgang empfiehlt Davatz, bei der Partnerin eines alkoholkranken Mannes anzusetzen, Alkoholikerinnen nicht zu bevormunden und Jugendlichen gegenüber auf die Vorbildfunktion der Erwachsenen im massvollen Umgang mit Alkohol zu setzen.",
    },
    Zusammenfassung {
        datei: "Anpassungsmechanismen_innerhalb_der_Kernfamilie_in_29.10.1991.pdf",
        titel: "Anpassungsmechanismen innerhalb der Kernfamilie in Überforderungssituationen",
        text: "Das Skript von U. Davatz für den Familientherapie-Kurs I (1991) beschreibt drei Anpassungsmechanismen der Kernfamilie in Überforderungssituationen, in Anlehnung an Bowen. Der Ehekonflikt wird als Dominanzkampf und zugleich als Ventil verstanden. Reicht dies nicht aus, wird einer der Partner zum Symptomträger (psychisch, physisch oder sozial), oder die elterlichen Spannungen werden auf das schwächste Glied, das Kind, projiziert. Jeder dieser Mechanismen entlastet kurzfristig und führt bei ausschliesslicher, langer Anwendung in einen Teufelskreis mit Krankheitscharakter.",
    },
    Zusammenfassung {
        datei: "Arbeits-_und_Freizeitplanung_innerhalb_der_Familie_28.10.1993.pdf",
        titel: "",
        text: "Leerer Fragebogen zur Arbeits- und Freizeitplanung innerhalb der Familie, entstanden im Familientheorie-Kurs «FT II/93». Er fragt nach dem Zeitaufwand von Partnern für Haushalt, Kindererziehung, Information und Weiterbildung und nach Familienpolitik (was Eltern weitergeben wollen), Krisenmanagement und Versöhnung, Budget und Entscheidungsfindung. Weitere Fragen gelten Freizeit, Anerkennung sowie persönlichen Pflichten, Stärken, Schwächen und dem Unterstützungsbedarf. Er gehört zur Kursarbeit vom 30.1.1994, in der die ausgefüllten Bogen ausgewertet werden.",
    },
    Zusammenfassung {
        datei: "Asthma_in_der_Familie_Welches_therapeutische_Vorgehen_03.10.1988.pdf",
        titel: "Asthma in der Familie – Welches therapeutische Vorgehen schlagen wir vor?",
        text: "Davatz formuliert therapeutische Ratschläge bei Asthma in der Familie, getrennt nach dem Ort des Symptoms. Ist ein Ehepartner betroffen, soll er lernen, sich Luft zu machen, sich besser abzugrenzen, Dysharmonie auszuhalten und das Harmoniebedürfnis aus der Ursprungsfamilie zu bearbeiten, der andere Partner soll Dysharmonie ansprechen und wahrnehmen lernen. Hat ein Kind das Symptom, sollen die Eltern ihre Spannungen und Konflikte wahrnehmen, die Unzufriedenheit klar definieren und die Kinder aus dem Positionenbeziehen heraushalten; Krach und Schreien sind erlaubt. Blatt aus Kurs II in Familientheorie, Oktober 1988.",
    },
    Zusammenfassung {
        datei: "Aufteilung_der_verschiedenen_Themen_in_Familientheorie_und_07.03.1988.pdf",
        titel: "Aufteilung der verschiedenen Themen in Familientheorie und Familienberatung",
        text: "Themenübersicht von U. Davatz für die Fortbildung in Familientheorie und Familienberatung (II. Teil). Die Themen werden in vier Gruppen mit je eigenem Vorgehen geordnet: statische, symptombezogene Themen wie Asthma (Symptom im Kontext analysieren, erst danach Analogien suchen), dynamische Prozessthemen wie Fusion und Differenzierung oder Ablösung bei Alleinerziehern (Prozessanalyse anhand fester Merkpunkte), Konstellationsthemen wie das Fehlen eines aktiven Erziehungspartners und Beziehungsthemen wie Kommunikation, Gerechtigkeit oder Loyalitätskonflikt des Helfers (Analyse der Wechselwirkungen zwischen zwei Personen). Das Blatt dient als Ordnungsraster für die Kursarbeiten.",
    },
    Zusammenfassung {
        datei: "Ausländerfamilien_28.09.1987.pdf",
        titel: "",
        text: "Stichwortartiges Kursblatt von U. Davatz zu Auswanderer- und Einwandererfamilien. Es stellt die Perspektiven des Heimatlandes, des Auswanderers und der Aufnahmegesellschaft gegenüber (Heimweh, Idealisierung, Misstrauen, Angst vor Überfremdung) und beschreibt die Folgen für die Familie: einzelne Grossleistungen, aber auch mehr Stress, Krankheitsanfälligkeit und Delinquenz der Kinder durch Anpassungsdruck. Die Ratschläge richten sich an die Einwandererfamilie (Wurzeln pflegen, nicht alle Traditionen halten, rasch die Sprache lernen) und an die Umwelt (Integration unterstützen, Andersartigkeit akzeptieren, Krankheit nicht als Faulheit abstempeln). Der Therapeut soll die Einwanderersituation im Fall stets mitbedenken.",
    },
    Zusammenfassung {
        datei: "Aussterbende_Familien_03.10.1988.pdf",
        titel: "",
        text: "Kurstext von Lotti Höner zum selben Kursthema wie das Blatt von Davatz. Als mögliche Merkmale aussterbender Familien nennt sie Loyalität (das Überleben der Familie wiegt mehr als die Entfaltung des Einzelnen), hohe gegenseitige Verantwortlichkeit, Nebeneinander von Fusionsverhalten und cut-off sowie ein relativ geschlossenes System mit Misstrauen gegenüber Eindringlingen wie möglichen Ehepartnern. Dazu kommt wenig Flexibilität, weil in Notlagen bewährtes Verhalten beibehalten wird. Ihr Fazit: Die Angst ums Überleben der Familie verhindert ihr Überleben. Kurs in Familientheorie, Oktober 1988.",
    },
    Zusammenfassung {
        datei: "Aussterbende_Familien_Welches_therapeutische_Vorgehen_wäre_03.10.1988.pdf",
        titel: "Aussterbende Familien – Welches therapeutische Vorgehen wäre notwendig, um Familien vor dem Aussterben zu bewahren?",
        text: "Davatz beschreibt die Situation «aussterbender» Familien: Die Kinder stehen unter der Verpflichtung, ihre Energie ganz in die Ursprungsfamilie zu stecken, wehren sich aber zugleich innerlich gegen diese Abhängigkeit, sodass der Konflikt zwischen Loyalität und Abwehr die Kräfte verbraucht. Therapeutisch soll der in der Schwebe gebliebene Entscheidungsprozess zwischen Loslösung und engerer Rückkehr vorangetrieben werden; oft gelingt die Ablösung besser, wenn man die Abhängigkeit zunächst anerkennt. Gelingt das nicht, bleibt nur die geografische und kulturelle Entfernung, die aber nur eine Pseudo-Ablösung ist. Blatt aus Kurs II in Familientheorie, Oktober 1988.",
    },
    Zusammenfassung {
        datei: "Ausstiegshilfen_unter_Berücksichtigung_der_Kultwellen_und_11.03.2002.pdf",
        titel: "Ausstiegshilfen unter Berücksichtigung der Kultwellen und geschlechtsspezifischen Aspekte",
        text: "Unterlage von U. Davatz für die Suchtpräventionsveranstaltung im Gfellergut vom 11./12. März 2002 zu Ausstiegshilfen unter Berücksichtigung von Kultwellen und geschlechtsspezifischen Aspekten. Prävention wird als richtiges Handeln im kritischen Augenblick und als interaktiver Entwicklungsprozess verstanden; die Auseinandersetzung mit Jugendlichen gleicht dem Ablösungskampf der Pubertät und dauert etwa zwei Jahre. Die Bedeutung des Suchtmittelkonsums wird für Jungen, Mädchen, ausländische Jugendliche und Schweizer unterschieden. Die Betreuer sollen diese Bedürfnisse erkennen und auf andere Weise befriedigen helfen, statt nur das Symptom durch Verbote zu bekämpfen. Die erste Seite trägt Handnotizen.",
    },
    Zusammenfassung {
        datei: "Automatismus_Motivation_28.10.2026.pdf",
        titel: "",
        text: "Kursarbeit zum Thema «Automatismus – Motivation» im Familientherapiekurs II, verfasst von einer Kursteilnehmerin. Automatismen werden als gespeicherte, einst sinnvolle Problemlösungen verstanden, die das Gehirn entlasten, aber erstarren, wenn die Wahrnehmung gestört ist; sie seien von Gewohnheiten zu unterscheiden und liessen sich nur durch Erleben, nicht durch Training abbauen. Die Arbeit überträgt das auf geschlossene Systeme und Machtstrukturen, auf Rollenverhalten in Familien (Ablösung, Vater-Mutter-Kind-Dreiecke), auf Angst und Fluchtverhalten und auf Beziehung als Grenzerfahrung. Mit Beispielen aus Tierwelt und eigener Erfahrung wird gefragt, warum Rollen später nicht mehr passen, und zur Elternarbeit betont, dass es dort meist um Vertrauen statt um Schuld gehe.",
    },
    Zusammenfassung {
        datei: "Begleitung_von_Menschen_mit_depressiven_Symptomen_im_14.03.2002.pdf",
        titel: "Begleitung von Menschen mit depressiven Symptomen im Akutspital",
        text: "Depressive Menschen im Akutspital zu begleiten, ist das Thema dieser Fortbildung am KSB vom 14.3.2002 von U. Davatz. Depression wird als Inflation der Gefühlswelt und Motivation beschrieben, die Helfer zu hilflosen und dann leicht aggressiven Helfern macht; entstehen kann sie aus überhöhten Zielen, einer ständigen Verliererposition in Beziehungen oder dem Kampf gegen einen Schicksalsschlag. Empfohlen wird, sich nicht emotional zu überinvestieren, den Patienten über seine Lebensgeschichte zu verstehen, den Schmerz und die Ohnmacht auszuhalten und keine schnellen Ratschläge zu geben. Die Haltung des Lernenden, der vom Leben lernen will, fördert eine partnerschaftliche Beziehung.",
    },
    Zusammenfassung {
        datei: "Begleitung_von_Menschen_mit_depressiven_Symptomen_im_14.03.2002_2.pdf",
        titel: "Begleitung von Menschen mit depressiven Symptomen im Akutspital",
        text: "Fortbildungsseminar am KSB vom 14.3.2002 zur Begleitung depressiver Patienten im Akutspital, von U. Davatz (frühere Fassung des Seminars von 2003; weiteres Exemplar mit zwei von Hand eingefügten Kommas). Die Depression gilt als chronisch und im Akutspital nicht behandelbar, doch Helfer sollen kompetent damit umgehen können. Beschrieben werden die inflationäre Gefühlswelt der Betroffenen, die Hilflosigkeit und Aggression der Helfer sowie Ursachen wie unerreichbare Ideale, Verliererposition und nicht akzeptierte Schicksalsschläge. Hilfreich sind ein Zugang über die Lebensgeschichte, Empathie ohne eigene Erschöpfung und eine lernende Haltung.",
    },
    Zusammenfassung {
        datei: "Begleitung_von_Menschen_mit_depressiven_Symtomen_im_13.02.2003.pdf",
        titel: "Begleitung von Menschen mit depressiven Symptomen im Akutspital",
        text: "Depression ist im Fortbildungsseminar am KSB Thema, obwohl sie als chronische Krankheit eigentlich nicht im Akutspital behandelt werden kann. Der Vortrag beschreibt sie «ökonomisch» als Inflation der Gefühlswelt: Selbstwert und Motivation brechen zusammen und zerstören auch die Motivation der Helfer, die hilflos werden und passiv aggressiv reagieren können. Zur depressiven Konstitution tragen laut Vortrag zwanghafter Ehrgeiz, das ständige Unterliegen in wichtigen Beziehungen und das Ankämpfen gegen Schicksalsschläge bei. Empfohlen wird, nicht emotional zu überinvestieren, den Patienten auf lebensgeschichtlicher Ebene zu verstehen, Ohnmacht auszuhalten, keine schnellen Ratschläge zu geben und die Haltung des Lernenden einzunehmen.",
    },
    Zusammenfassung {
        datei: "Betreuung_psychisch_Kranker_in_der_Spitex_05.06.2003.pdf",
        titel: "",
        text: "Vortrag für die Spitex Windisch über die Betreuung psychisch Kranker. Davatz versteht psychische Krankheiten als Beziehungskrankheiten und zeichnet den Weg von der Sozialpsychiatrie der 68er-Jahre (Basaglia, Maxwell Jones) zur gemeindenahen Versorgung durch die Spitex nach. Da die Hospitalisation meist Trauma und Stigmatisierung bedeute, sei es besser, Betroffene im natürlichen Umfeld zu belassen. Spitex-Mitarbeitende sollen keine Diagnosen behandeln, sondern eine normale, authentische und nicht auf Helfen ausgerichtete Beziehung anbieten, bei Überforderung eine Fachperson beiziehen und ihre «psychiatrische Unwissenheit» behalten.",
    },
    Zusammenfassung {
        datei: "Cut-off_29.05.1989.pdf",
        titel: "",
        text: "Davatz betrachtet das cut-off-Verhalten als Problemlösungs- bzw. Coping-Strategie. Sie unterscheidet Aggressions- und Kampfverhalten, Flucht- und Rückzugsverhalten sowie Ausweich-, Tarn- und Ablenkungsverhalten, jeweils mit typischen Folgen und diagnostischen Zuordnungen (etwa Depression, Sucht, psychosomatische Leiden). Cut-off sei eine Mischung aus Flucht- und Ausweichverhalten, geografisch (Umzug, Auswanderung) oder emotional (Tabuisierung, Kommunikationsabbruch). Als Folgen nennt sie Entwicklungsstillstand, ungelöste und nur «auf Eis gelegte» Konflikte, vermeintliche Selbstständigkeit, Energieverlust im erweiterten Familiensystem und eine Neigung zu neuer Fusion in der Kernfamilie. Skript für den Familientherapie-Kurs I, 1989.",
    },
    Zusammenfassung {
        datei: "Das_Alter_mit_seinen_Würden_und_Bürden_06.04.2006.pdf",
        titel: "",
        text: "Vortrag bei der Spitex Oftringen über Würden und Bürden des Alters. Zu den Würden zählen Lebens- und Berufserfahrung, schnelle Entscheidungen dank eingeschliffener «Autobahnen» im Gehirn, Altersweisheit (Verweis auf Piaget: Problemlösung brauche Junge, Mittelalte und Weise), Gelassenheit, die Weitergabe von Geschichten und Traditionen sowie die Freiheit, Dinge loszulassen. Als Bürden werden Gebrechen und Schmerzen, chronische Krankheiten, der näher rückende Tod und Abschiede, Abhängigkeit von anderen und das Gefühl, selbst zur Last zu werden, genannt. Am schwersten wiege das Gefühl, das Leben nicht richtig gelebt zu haben; die Schlussfolgerung lautet, man solle das Leben jetzt leben, um das Alter mit Würde zu erleben.",
    },
    Zusammenfassung {
        datei: "Das_Burn-out-Syndrom_27.09.2007.pdf",
        titel: "",
        text: "Vortrag für den Frauenzmorge Auenstein (27.9.2007) über das Burn-out-Syndrom von Ursula Davatz. Als Ursachen nennt sie Leistungsdruck und Ehrgeiz in der Wachstumsgesellschaft, Abhängigkeit von Anerkennung, Angst vor Stellenverlust oder Kritik sowie zu hohe eigene Ziele; in sozialen Berufen komme das ständige Geben hinzu. Zur Vorbeugung empfiehlt sie Abgrenzung und den Mut, Grenzen gegenüber Vorgesetzten zu vertreten, die Konzentration auf die Qualität des eigenen Handwerks und regelmässige Erholungsphasen. Burn-out versteht sie als chronische Überforderung und Vorstufe einer Depression oder anderer Krankheiten.",
    },
    Zusammenfassung {
        datei: "Das_Fehlen_eines_aktiven_Erziehungspartners_19.07.1988.pdf",
        titel: "",
        text: "Stichwortblatt zu den Folgen, wenn in einer Familie ein aktiver Erziehungspartner fehlt. Genannt werden stärkere Umsorgung der Kinder, ein ältestes Kind in der Rolle des Ersatzelternteils, das eigene Bedürfnisse zurückstellt, stärker an den Elternteil gebunden ist und nur schwach eine eigene Identität entwickelt, sowie ein «Pseudo-Erwachsensein». Weitere Punkte sind fehlende bewusste Erziehung und Förderung in Schule und Beruf, das Empfinden des neuen Partners als Eindringling und die Suche nach Identifikationsfiguren ausserhalb der Familie in der Pubertät (etwa Kirche, Jugendarbeit). Notizen aus dem Kurs in Familientheorie, 1988.",
    },
    Zusammenfassung {
        datei: "Das_Konzept_des_cut-off-Verhaltens_in_einer_Beziehung_oder_29.10.1987.pdf",
        titel: "Das Konzept des «cut-off»-Verhaltens in einer Beziehung oder die gewaltsame Abtrennung einer Fusions-Beziehung",
        text: "Das von Bowen stammende Konzept des «cut-off» wird als gewaltsames Sichabwenden aus einer zu intensiv gewordenen Fusionsbeziehung erklärt, das zum Fluchtverhalten gehört. Davatz zeigt es an Beispielen: dem abrupten Auszug oder Rauswurf Jugendlicher, dem Umschlagen von Liebe in Hass und der Scheidung, dem Ende von Geschäftspartnerschaften sowie dem Bruch zwischen Geschwistern bei Erbstreitigkeiten, der sich über Generationen fortsetzen kann. Cut-off kann geografisch oder rein emotional sein und folgt oft einer Familientradition; auf eine Trennung folgt häufig eine neue Fusionsbeziehung. Entscheidend ist ihre Schlussaussage, dass cut-off nie zu echter Freiheit von den Problemen führt, sondern nur vorübergehend entlastet. Skript für den Familientherapie-Kurs I 1987.",
    },
    Zusammenfassung {
        datei: "Das_Leben_der_Frau_im_steten_Wandel_01.06.2005.pdf",
        titel: "",
        text: "Vortrag am Morgentreff Gränichen über das Leben der Frau als steten Wandel. Ausgehend vom zyklischen Geschehen in der Natur und im weiblichen Körper beschreibt der Text die Lebensphasen der Frau (Mädchen, junge Frau, Mutter, Frau im mittleren Alter mit «empty nest», reife und alte, weise Frau) und das Verhältnis zur Sexualität. Eine berufstätige Mutter habe vier Rollen zu erfüllen: Partnerin, Mutter, Berufsfrau und Individuum. Die Referentin plädiert dafür, die Fähigkeit zum Wandel zu bewahren und im öffentlichen Leben stärker zu vertreten, da die Welt nicht allein von Fortschritt und Superlativen regiert werden könne.",
    },
    Zusammenfassung {
        datei: "Das_Leiden_der_Familienangehörigen_und_ihr_Potential_als_25.08.2005.pdf",
        titel: "Das Leiden der Familienangehörigen und ihr Potential als Co-Therapeuten",
        text: "Vortrag bei der VASK Aargau, der Vereinigung der Angehörigen von Schizophrenie-/Psychisch-Kranken, über Eltern als Co-Therapeuten. Die Schizophrenie wird als funktionelle Störung und Entwicklungsstillstand gesehen, als missglückter Ablösungskonflikt mit Endlosphase («maligne Pubertät»): Der Autonomieinstinkt des Patienten und das Sorgeverhalten der Eltern laufen sich entgegen. Therapeutisch sollen Eltern zuerst den Zustand des Patienten verstehen, emotionalen Stress senken (Verweis auf Leff und Vaughn zu «high EE»), das Sorgeverhalten zurücknehmen, Verantwortung übergeben und rasch einen Berater erreichen können. Langfristiges Ziel ist die Entwicklung des jungen Menschen (Beruf, soziale Kontakte, Beziehung) statt der Krankheitsbehandlung; Früherfassung und Frühberatung der Eltern könnten oft eine Hospitalisation vermeiden.",
    },
    Zusammenfassung {
        datei: "Das_Leiden_der_Familienangehörigen_und_ihr_Potential_als_28.10.2026.pdf",
        titel: "Das Leiden der Familienangehörigen und ihr Potential als Co-Therapeuten",
        text: "Zeitschriftenartikel von Ursula Davatz über das Leiden von Angehörigen psychisch Kranker, vor allem von Schizophreniekranken, und ihr Potenzial als Co-Therapeuten (Thema des VASK-Vortrags vom 25.8.2005). Angehörige passen sich dem Kranken über Jahre an, ziehen sich aus Scham zurück, brechen schliesslich erschöpft zusammen und sehen oft nur noch den fürsorgerischen Freiheitsentzug als Ausweg. Fachleute sollten ihnen zuerst Verständnis und Empathie entgegenbringen, bevor sie sie als Mitarbeitende gewinnen. Davatz berichtet aus ihrer über zwanzigjährigen Angehörigengruppe, dass der frühe Einbezug der Familie Hospitalisationen verhindern, Sitzungen sparen und die Prognose verbessern kann.",
    },
    Zusammenfassung {
        datei: "Das_POS-ADS-Kind_13.10.2003.pdf",
        titel: "",
        text: "Vortrag von Dr. med. U. Davatz im Katholischen Pfarreizentrum Frick vor Eltern. Ausgangspunkt ist die These, dass POS/ADS-Kinder Normierungstendenzen widerstehen und Erziehung deshalb kreativ und im Sinne einer «rollenden Planung» immer neu angepasst werden muss. Nach einer Liste typischer Merkmale (Aufmerksamkeits-, Wahrnehmungs- und Lernstörungen, Impulsivität, Hypersensibilität) werden vier Problemfelder mit Umgangsvorschlägen besprochen: Ungehorsam, Machtkampf, Leistungsschwankungen sowie das Zusammentreffen von Impulsivität und Sensibilität. Bei Normpressung drohe Pathologie (Delinquenz, Sucht, Depression, Schizophrenie), bei individueller Behandlung könnten kreative, durchsetzungsstarke Erwachsene heranwachsen.",
    },
    Zusammenfassung {
        datei: "Das_POS-ADS-Kind_13.10.2003_2.pdf",
        titel: "",
        text: "Zweite Fassung des Vortrags von Dr. med. U. Davatz im Pfarreizentrum Frick über das POS/ADS-Kind, mit anderem Kopf und leicht umgestellten Schlussbemerkungen. Erziehung müsse bei diesen von der Norm abweichenden Kindern kreativ und unter genauer Beobachtung laufend angepasst werden. Besprochen werden Merkmale der Störung sowie Ungehorsam, Machtkampf, Lernstörungen mit Leistungsschwankungen und die Verbindung von Impulsivität und hoher Sensibilität, jeweils mit Hinweisen: Aufmerksamkeit gewinnen, nur eine positive Anweisung geben, Stärken fördern, die Beziehung vor die Therapie stellen, eigene Emotionen zuerst beruhigen. Schlussgedanke: Richtig begleitet können aus POS/ADS-Kindern kreative, unternehmerische Erwachsene werden.",
    },
    Zusammenfassung {
        datei: "Das_POS-ADS_in_der_Pubertät_Wie_Eltern_helfen_können_17.11.2005.pdf",
        titel: "Das POS/ADS in der Pubertät – Wie Eltern helfen können",
        text: "Vortrag bei ELPOS Zürich, Glarus und Schaffhausen über das POS/ADS in der Pubertät. Eltern kämpfen um ihre Wertvorstellungen, der Jugendliche um Autonomie und Identität; ein Tierbeispiel mit Meerschweinchen zeigt, wie stark die Pubertät das spätere Sozialverhalten prägt. Beschrieben werden die Besonderheiten von POS/ADS-Kindern: hohe emotionale Aufmerksamkeit, Verletzlichkeit, Kritikempfindlichkeit, Dickkopf, Ängstlichkeit und plötzliches Reifen. Eltern sollen Regeln ohne emotionalen Druck, ohne Strafe und Schuldzuweisung vertreten, nachgeben können und eigene Konflikte nicht über das Kind austragen. Warnungen betreffen Selbstmedikation mit Alkohol und Drogen, Cannabis und Schizophrenie, Persönlichkeitsstörung bei rigider und Depression bei schuldzuweisender Handhabung; Eltern seien «Steigbügelhalter» ihrer Jugendlichen.",
    },
    Zusammenfassung {
        datei: "Das_POS-ADS_in_der_Pubertät_Wie_Eltern_helfen_können_17.11.2005_2.pdf",
        titel: "Das POS/ADS in der Pubertät – Wie Eltern helfen können",
        text: "Zweite, gekürzte Fassung des Vortrags für ELPOS über das POS/ADS in der Pubertät, ohne das Meerschweinchen-Beispiel und mit umformulierten Abschnitten. Sie behandelt die Pubertät als Zerreissprobe zwischen den Generationen, die Besonderheiten der Kinder mit POS/ADS (Sensibilität, Verletzlichkeit, Dickkopf, Ängstlichkeit, plötzliche Vernunft) und Hinweise für den Umgang: Regeln ohne emotionalen Druck und ohne Strafe durchsetzen, nicht in endlose Diskussionen geraten, auch verlieren können. Als Risiken einer fehlgelaufenen Pubertät werden Alkohol und Drogen, Cannabis mit dem Schizophrenierisiko, Persönlichkeitsstörung bei strenger und Depression bei schuldzuweisender Haltung genannt. Eltern sollen Fehler eingestehen, daraus lernen und im Machtkampf elastisch, aber standfest bleiben.",
    },
    Zusammenfassung {
        datei: "Das_POS-Kind_in_der_Pubertät_24.03.2003.pdf",
        titel: "",
        text: "Vortrag für Eltern bei ELPOS (Zürich, Glarus, Schaffhausen) über Kinder mit POS (Psychoorganisches Syndrom) in der Pubertät. Davatz beschreibt die Pubertät als nötigen Ablösungskonflikt und schildert, wie sich typische Verhaltensweisen (Intimwelt, Explorationsverhalten, Imponiergehabe, Angst vor Verantwortung) bei POS-Kindern äussern: eigenwillig, direkt, sensibel, oft aggressiv abwehrend, in der Berufswahl verzögert. Als häufige Elternfehler nennt sie zu lange Kontrolle, Übergriffe in die Intimwelt, Disziplinarmassnahmen statt Beruhigung und zu frühen Druck bei der Berufswahl. Ihre Tipps: von Erziehung zu Beziehungspflege wechseln, eine Art offizielle Machtübergabe vollziehen, sich abgrenzen, den Jugendlichen «Welpenschutz» gewähren und selbst innere Sicherheit finden.",
    },
    Zusammenfassung {
        datei: "Das_Tellereisen_06.06.1988.pdf",
        titel: "",
        text: "Kursarbeit von K. Bachmann zu Walther Kauers Roman «Tellereisen» mit der Leitfrage, inwieweit Familie und gesellschaftliche Beziehungen für die Persönlichkeitsentwicklung ein Tellereisen sind. Sie gibt die vier ineinandergeschachtelten Erzählebenen wieder: das Leben eines Aussenseiters im Bergell mit Fremdenhass, Schulden und Depression, die Geschichte seiner Ehe und Trennung, die historische Aufarbeitung des 17. Jahrhunderts und die Parabel vom Wolf, der sich nur durch Abbeissen der eigenen Pfote aus der Falle befreit. Das Bild wird so auf das Loslösen aus einengenden Beziehungen und die Prägung des Menschen durch Umstände und Sachzwänge bezogen. Kurs in Familientheorie, Juni 1988.",
    },
    Zusammenfassung {
        datei: "Das_gesunde_Cut-off_29.10.1993.pdf",
        titel: "",
        text: "Gesunde Ablösung im Sinne des Cut-off-Konzepts beschreibt dieses Skript von U. Davatz (Familientherapie-Kurs I, 1993) für beide Seiten: Die Kinder grenzen sich ab der Pubertät ab, die Eltern verweigern Dienstleistungen, Schonhaltung und unbeschränkten Zugang zum Geld und erwarten erwachsenes Verhalten. Pathologisch wird der Cut-off bei zu enger, fusionärer Eltern-Kind-Beziehung, Parentifizierung oder zu langer Dienstleistung der Eltern. Auch stark abweichende Begabungen oder ein ganz anderer Berufsweg des Kindes können ihn nötig machen. Ein Cut-off kann demnach überlebensnotwendig sein, weil der Konflikt sonst gefährlich eskalieren könnte.",
    },
    Zusammenfassung {
        datei: "Das_trotzende_Kind_Bin_ich_zu_lieb_oder_bin_ich_zu_streng_11.11.2003.pdf",
        titel: "Das trotzende Kind – Bin ich zu lieb oder bin ich zu streng?",
        text: "Vortrag für die Mütter- und Väterberatung und Pro Juventute zur Frage, ob man als Elternteil zu lieb oder zu streng ist. Trotz und pubertäre Rebellion versteht Davatz als Ausdruck von Ich-Stärke; der Erziehungsstil hänge vom Menschenbild ab, und es gebe keinen allgemein richtigen Stil. Sie unterscheidet das eigenwillige POS/ADS-Kind, das sensible Kind mit wenig Eigenwillen und das gleichzeitig trotzige und sensible Kind und gibt je Typ Erziehungsvorschläge: klare Prioritäten, sanfte Übergänge, Schutzräume, erst beruhigen und dann über Regeln sprechen. Die Antwort auf die Titelfrage hänge von Kind, Situation und Erzieher ab; Eltern sollen Kurs halten, vom Kind lernen und Fehler zulassen.",
    },
    Zusammenfassung {
        datei: "Das_volle_Nest_das_leere_Nest_Was_ist_schlimmer_Was_ist_15.03.2007.pdf",
        titel: "Das volle Nest – das leere Nest: Was ist schlimmer? Was ist schöner?",
        text: "Vortrag vor der Frauengemeinschaft Gebenstorf zum vollen und zum leeren Nest. Für das volle Nest gilt: Gute Brutpflege braucht eine gute Partnerschaft, die Frau soll ihren Mann nicht bemuttern oder sich unterordnen, die Kinder nicht als Tröster oder für Dankbarkeit einspannen und ihre Partnerschaft pflegen, ob Hausfrau oder berufstätig. Beim leeren Nest («empty nest syndrome») verliere die Mutter Macht und Lebenssinn und versuche möglicherweise, die Kinder zurückzuhalten, besonders das Nesthäkchen. Als Aufgabe für die Zeit danach nennt die Referentin das Pflegeverhalten auf allen Ebenen, Selbstbewusstsein, Mut zum Anderssein, Vertrauen in sich und «geistige Kinder», getragen von der weiblichen Regenerationsfähigkeit.",
    },
    Zusammenfassung {
        datei: "Depression,_eine_Frauenkrankheit_und_wie_sieht_es_bei_den_21.10.2003.pdf",
        titel: "Depression, eine Frauenkrankheit – und wie sieht es bei den Männern aus?",
        text: "Vortrag von Dr. med. U. Davatz im Pfarreizentrum Nussbaumen (AG) zur Frage, warum Depression häufiger bei Frauen diagnostiziert wird und wie es bei Männern aussieht. Depression ist für sie eine «Verliererkrankheit», die entsteht, wenn jemand sich in wichtigen Beziehungen wiederholt als Verlierer erlebt. Frauen würden zur Anpassung erzogen und dabei leicht ausgebeutet, Männer reagierten eher auf Situationen, in denen Kämpfen nichts nützt (Krankheit, Arbeitsplatzverlust). Therapeutisch gelte es, Beziehungssituationen und Auseinandersetzungsstrategien zu klären und tief sitzende Erziehungsmuster geduldig zu verändern, trotz auftretender Schuldgefühle; Frauen lernen dabei eher das Kämpfen, Männer das Anpassen.",
    },
    Zusammenfassung {
        datei: "Depression_Gibt_es_das_auch_bei_geistig_behinderten_10.03.2003.pdf",
        titel: "Depression – Gibt es das auch bei geistig behinderten Jugendlichen und Erwachsenen?",
        text: "Vortrag für Insieme Region Baden über Depression bei geistig behinderten Jugendlichen und Erwachsenen. Davatz beschreibt Depression als «soziale Verliererkrankheit» und zeigt, wie Eltern und Betreuende durch Überforderung, Überbehütung oder ein abwertendes Menschenbild Frustration erzeugen können. Sie betont die oft hohe emotionale Intelligenz geistig Behinderter, die ihre Wahrnehmungen nicht in Worte fassen können und für impulsive Reaktionen bestraft werden. Depression entstehe, wenn sie dauernd überfordert, überfahren oder für ihre sensiblen Reaktionen bestraft werden; die Zuhörenden sollen diese Hypothesen im Alltag prüfen.",
    },
    Zusammenfassung {
        datei: "Depression_eine_Zeiterscheinung_19.09.2002.pdf",
        titel: "",
        text: "Depression wird als Zeiterscheinung gedeutet: In einer Gesellschaft, die ständige Steigerung erwartet, erscheint jeder Rückgang als negativ, während biologische Prozesse zyklisch verlaufen. Der Vortrag vor drei Frauenbünden beschreibt Depression als «Krankheit der Verlierer»: Wer in wichtigen Beziehungen im Dominanzkampf (Hackordnung, Geschwisterposition) dauernd unterliegt oder mit zu hohen Zielen scheitert, verliere Energie. Auch Verlusterlebnisse (Partner, Stelle, Freiheit, Besitz) können auslösen. Antidepressiva seien bis zu einem gewissen Grad Aufputschmittel; positiv gesehen kann Depression eine Phase des Innehaltens und Neuüberlegens sein.",
    },
    Zusammenfassung {
        datei: "Depression_eine_Zeiterscheinung_19.09.2002_2.pdf",
        titel: "",
        text: "Zweites Exemplar des Vortrags «Depression – eine Zeiterscheinung» für den katholischen, den reformierten und den gemeinnützigen Frauenbund (mit Handkorrekturen und einer Randnotiz «Verlustreaktion»). Depression wird als Folge von Dominanzverlust in wichtigen Beziehungen, von zu hohen eigenen Zielen und von Verlusterlebnissen gedeutet. Die Leistungserwartung der Gesellschaft mache Menschen leicht zu vermeintlichen Verlierern; Antidepressiva wirkten bis zu einem gewissen Grad aufputschend. Positiv betrachtet könne Depression eine Ruhe- und Bedenkphase sein, in der man Ziele und Kampfstrategien überdenkt.",
    },
    Zusammenfassung {
        datei: "Der_Ablösungs-_und_Trennungsprozess_bei_Affenkindern_05.12.1988.pdf",
        titel: "",
        text: "Kurstext von D. Faust über die Ablösung und Trennung von Mutter und Kind bei Affen, mit Bezug auf Renggli. Menschenaffen reifen etwa doppelt so schnell wie Menschenkinder. Ab dem vierten Monat erkundet das Affenkind seine Umwelt, die Mutter schränkt dies zunächst ein, lässt später grössere Entfernungen zu und weist das Kind schliesslich zurück, auch mit Strafen, was meist mit der Geburt eines neuen Kindes abgeschlossen ist. Beim passiven Trennungsprozess der Schimpansen wendet sich die Mutter nur ab, und die Jungen reagieren mit Wutausbrüchen, Apathie oder Regression. Kurs in Familientherapie, 5.12.1988.",
    },
    Zusammenfassung {
        datei: "Der_Einfluss_des_Berufes_des_Mannes_auf_die_Frau_und_auf_die_30.01.1994.pdf",
        titel: "Der Einfluss des Berufes des Mannes auf die Frau und auf die Familie in ihrer Entwicklung",
        text: "Kursarbeit einer Teilnehmerin im Familientheorie-Kurs FT II/93: Ausgehend von der Frage, wie stark der Beruf des Mannes die Familie prägt, wertet sie einen Fragebogen zur Arbeits- und Freizeitplanung aus, den sie aus ihrer Arbeit in der Mütter- und Väterberatung heraus einsetzte. Frauen setzen demnach ein Mehrfaches an Zeit für Haushalt und Kinderbetreuung ein, Männer vergleichsweise viel für eigene Information; Kinderbetreuung wird von vielen als wichtig, aber auch als anstrengend genannt. Befragt wurden überwiegend Frauen, keine Alleinerziehenden. Als Schlussfolgerung schlägt sie ein Familienzentrum zur Entlastung der Eltern vor und will den Fragebogen in Kursen für werdende Eltern verwenden.",
    },
    Zusammenfassung {
        datei: "Der_Emanzipierte_Mann_24.09.2002.pdf",
        titel: "",
        text: "Der Vortrag für La Vida fragt, welche Rolle der Mann in einer globalisierten Welt hat, in der die Territorien als Kampfplatz weggefallen sind. Ausgehend von der Rollenverteilung «Jäger/Territoriumsverteidiger» und «Nestpflegerin» wird beschrieben, dass die Frau aus dem Nest in den Wettkampf um Beute (Geld) getreten ist und das Nest kaum noch besteht. Die neue Aufgabe des Mannes sei, gemeinsam mit der Frau ein neues Nest aufzubauen, Nein zu Sucht und Manie nach mehr Geld und Macht zu sagen und Lebensqualität zu fördern. Statt «emanzipierter» spricht die Vortragende lieber vom «involvierten» Mann; in einer globalen Welt brauche es mehr Kooperation als Konkurrenz.",
    },
    Zusammenfassung {
        datei: "Designerdrogen,_die_modernen_chemischen_Problemlöser_der_26.09.2002.pdf",
        titel: "Designerdrogen, die modernen chemischen Problemlöser der Jugend",
        text: "Designerdrogen werden im Vortrag für den Samariterverein Wölflinswil als Teil der Modekrankheit Drogensucht dargestellt: Weil Süchtige zur Gewohnheit wurden, brauche der Markt neue, attraktiv vermarktete Angebote. Der Begriff «Designer-» oder «Lifestyle-Droge» verharmlose die Substanzen und suggeriere einen Lebensstil. Für Jugendliche sind sie ein billiger privater Nachbau der schnellen chemischen Problemlöser der Pharmaindustrie. Am Beispiel von Ecstasy werden Wirkung und Risiken genannt; das Betäubungsmittelgesetz komme den ständig neuen Substanzen nicht nach. Schutz biete vor allem, wenn Eltern und Erzieher den Glauben an schnelle chemische Lösungen relativieren und Zeit für nachhaltige Problemlösungen vermitteln.",
    },
    Zusammenfassung {
        datei: "Die_Kommunikation_im_therapeutischen_Prozess_27.05.2003.pdf",
        titel: "",
        text: "Weiterbildung am Psychiatriestützpunkt Biel zur Kommunikation als wichtigstem Mittel von Psychiatern und Psychotherapeuten. Davatz stellt die soziobiologische Definition (Kommunikation ist egoistisch und zielgerichtet) der altruistischen Aufgabe der Therapie gegenüber und unterscheidet Inhalts-, Emotions- und Sozialanteil der Kommunikation. Die Diagnose als Fachsprache sieht sie im therapeutischen Prozess eher als Hindernis, da sie starr wirkt und vom Patienten oft als Stigmatisierung erlebt wird. Therapeutische Kommunikation soll Beziehung ohne Angst herstellen, den Patienten wertschätzen, das System aus der Homöostase bringen und begleiten – ohne Absicht, Macht oder Überzeugungsdruck, getreu «primum nihil nocere». Auf Seite 2 stehen handschriftliche Randnotizen.",
    },
    Zusammenfassung {
        datei: "Die_Kommunikation_im_therapeutischen_Prozess_27.05.2003_2.pdf",
        titel: "",
        text: "Überarbeitete Fassung der Weiterbildung am Psychiatriestützpunkt Biel, in der die handschriftlichen Randnotizen abgetippt sind. Davatz legt dar, dass Kommunikation nach soziobiologischer Definition egoistisch ist, Therapie dagegen altruistisch sein soll. Die Anteile der Kommunikation (Information, Mission, emotionale Färbung, Sozialverhalten) werden mit Begriffen wie «fitness through know how», «social fitness», «intellectual fitness», «emotional fitness» und «social regulation» ergänzt. Die Diagnose gilt als starres Element, das Veränderung eher verhindert; als goldene Regel nennt sie, Inhalte ohne Absicht, Macht und Ziel zur Verfügung zu stellen.",
    },
    Zusammenfassung {
        datei: "Die_Zweierbeziehung_in_der_Ehe_23.11.1992.pdf",
        titel: "",
        text: "Das Skript von U. Davatz (Familientherapie-Kurs I, 1992) beschreibt die Zweierbeziehung in der Ehe als labiles Gleichgewicht, auf dem die Kleinfamilie ihre gesamten Ressourcen aufbaut. Typische Fehlentwicklungen entstehen aus positiven und negativen Erwartungen, die aus der Beziehung zu den eigenen Eltern auf den Partner übertragen werden, sowie aus Erziehungskonflikten durch gegensätzliche Erziehungsstile. Ausserdem werden männliches und weibliches Stress- und Erziehungsverhalten gegenübergestellt. Als stabilisierende Faktoren nennt das Skript gute Beziehungen zur Ursprungsfamilie, eigene Interessen, klare Selbstdarstellung statt Anpassungsversuche am Partner und Zuhörenkönnen.",
    },
    Zusammenfassung {
        datei: "Differenzierung_im_Ehekonflikt_28.10.2026.pdf",
        titel: "",
        text: "Seminararbeit zum Familientherapie-Seminar II (um 1990, Dozentin U. Davatz) über Differenzierung im Ehekonflikt, verfasst von einer Teilnehmerin. Nach Bowen werden Ich-Differenzierung, Fusionsbeziehungen, Dreiecksbeziehungen und starre Systeme als Faktoren des Ehekonflikts dargestellt, dazu der Verlauf von Stress über Dominanzkampf bis zum «Cut-off». Ergänzt wird dies durch Willis Kollusionskonzept, Beziehungsphantasien nach Kast und Gedanken von Duss-von Werdt zu «Scheidungen in der Ehe». Behandelt werden schliesslich Wege zur Differenzierung (klare Kommunikation, faires Streiten) und die Haltung des Therapeuten mit «innerlich beteiligter Unparteilichkeit» nach Stierlin; am Ende steht ein Literaturverzeichnis.",
    },
    Zusammenfassung {
        datei: "Differenzierung_in_Bilderbuchfamilien_11.1988.pdf",
        titel: "",
        text: "Kursarbeit von Ruth Buser über «Bilderbuchfamilien», die nach innen und aussen Harmonie als Forderung vertreten und dafür Probleme, Aggression und Neid verleugnen. Sie führt dies auf idealisierende Familiengeschichten und einen verdeckten Ehekonflikt zurück, der durch die übermässige Zuwendung zu den Kindern verdeckt wird; die Kinder dienen als Puffer, Vermittler oder Sündenböcke und erhalten Doppelbotschaften. Differenzierung ist hier besonders schwer, weil Bequemlichkeit, Schuldgefühle und fusionierte Bindungen die Ablösung hemmen. Therapeutisch sei das System kaum zu erreichen, mit den Eltern wäre Arbeit rascher wirksam, doch gelingt eher die Arbeit mit Jugendlichen, die ihre Ablösungsschwierigkeiten wahrnehmen. November 1988.",
    },
    Zusammenfassung {
        datei: "Einbezug_von_Familienangehörigen_bei_Depressionen_03.12.2003.pdf",
        titel: "",
        text: "Weiterbildung am Stützpunkt Biel über den Einbezug von Angehörigen bei Depressionen. Davatz geht vom Konzept der «high expressed emotions» (Leff und Vaughn) aus und deutet die Depression familientherapeutisch und ethologisch als Verliererkrankheit im Sinne eines arretierten Kampfverhaltens, das in appellatives Bettelverhalten übergeht. Helfer geraten dabei in eine dominante Position, der Depressive wehrt sich dagegen und macht sie zu «hilflosen Helfern» (sekundärer Machtgewinn). Angehörige sollen deshalb die Helferrolle verlassen, Kampfmuster und Übergriffe erkennen, Störendes aus der Ich-Position ansprechen, nur frei gegebene Hilfe leisten und sich vom Patienten helfen lassen, um dessen Selbstwert zu stärken.",
    },
    Zusammenfassung {
        datei: "Einige_Worte_zur_Aufarbeitung_von_Frau_Ursula_Müller-Biondi_25.11.2002.pdf",
        titel: "Einige Worte zur «Aufarbeitung» von Frau Ursula Müller-Biondi anlässlich der Buchvernissage",
        text: "Kurze Ansprache zur Buchvernissage in Zürich, in der U. Davatz als Therapeutin über die Aufarbeitung einer Lebensgeschichte spricht. Sie unterscheidet verschiedene Wirkungen des Geschichtenerzählens (Unterhaltung, hypnotische Wirkung, Geschichtsschreibung) und betont die heilende Wirkung des Aufschreibens der eigenen, schmerzlichen Lebensgeschichte im Sinne eines «narrativen Rekonstruktivismus». Die Autorin sei ursprünglich wegen eines Ausbildungsnachweises in ganzheitlicher Psychologie gekommen; die Gespräche hätten sich zur Aufarbeitung traumatischer Erlebnisse aus Kindheit und Jugend entwickelt. Die Rednerin gratuliert zu Kraft und Mut und nennt diese Form eine «philosophische Therapieform» mit wachsender Bedeutung.",
    },
    Zusammenfassung {
        datei: "Einleitung_zur_Dissertation_von_Dieter_Hasler_04.05.1994.pdf",
        titel: "",
        text: "Einleitung von U. Davatz zur Dissertation von Dieter Hasler (4.5.1994), einer Pilotstudie zu Erziehungsstil und Stressbewältigung von Eltern und der Entwicklung einer Schizophrenie bei einem Kind. Sie zeichnet die Forschungsgeschichte nach (analytische Theorie, Zwillings- und Adoptivstudien, Familientherapie von Bowen, Bateson und anderen, Expressed Emotions, Vulnerabilitätsmodell) und beschreibt den Fragebogen. Verglichen wurden 75 Familien mit einem schizophrenen Kind und 100 Kontrollfamilien. Die Familien mit schizophrenem Kind gaben häufiger bedrohlich-bestrafende Erziehung, weniger Übereinstimmung im Erziehungs- und Problemlösungsstil, mehr ausweichendes Verhalten der Väter und überengagiertes Stressverhalten der Mütter an. Daraus wird ein präventiver, psychoedukativer Ansatz abgeleitet.",
    },
    Zusammenfassung {
        datei: "Emanzipation_eines_Partners_und_die_Folgen_auf_die_18.04.1989.pdf",
        titel: "Emanzipation eines Partners und die Folgen auf die Paar-Beziehung, Probleme des Ehe-Beraters/-Therapeuten",
        text: "Referat von F. Haller im Kurs II Familientherapie (Leitung U. Davatz) darüber, wie sich die Emanzipation eines Partners auf die Paarbeziehung und auf den Ehe-Berater auswirkt. Anhand von vier anonymisierten Fällen aus der Eheberatung zeigt er, dass Paare in der Krise Muster wiederholen, die aus ihren Herkunftsfamilien stammen, und dass der Emanzipation eine enge Fusionsbeziehung vorausging, die den anderen Partner stark verunsichert. Er hält fest, dass Emanzipation der Differenzierung von der Herkunftsfamilie entspricht und nicht mit cut-off verwechselt werden darf. Für die Neutralität des Therapeuten empfiehlt er, an der Beziehung zur Herkunftsfamilie zu arbeiten (wie bei Eltern schizophrener Kinder), notfalls mit zirkulären Fragen. Mai 1989.",
    },
    Zusammenfassung {
        datei: "Emigration_Geschichte_der_Emigration,_speziell_der_06.03.1989.pdf",
        titel: "Emigration – Geschichte der Emigration, speziell der italienischen Emigranten in der Schweiz",
        text: "Kursarbeit von G. Petrazzo für Familientherapie Kurs II über die Geschichte der Emigration, besonders der italienischen in der Schweiz. Sie unterscheidet die Eroberungs- und Auswanderungsepoche vom heutigen, wirtschaftlich und politisch bedingten Auswandern und schildert, wie italienische Arbeiter seit dem 19. Jahrhundert gebraucht, politisch überwacht und ausgewiesen wurden (Anarchisten, Streiks, Fremdenpolizei, Ausländergesetze, Marshallplan, Auswanderungsplan Italiens). Im persönlichen Schluss schildert der Autor aus der Sicht eines Angehörigen der zweiten Einwanderergeneration das Gefühl, zwischen den Gruppen zu stehen, und will zum Nachdenken statt zu «Patentrezepten» anregen. Ein Beiblatt listet Redeverbote des Bundesrats auf. März 1989.",
    },
    Zusammenfassung {
        datei: "Emigration_Geschichte_der_Emigration,_speziell_der_06.03.1989_2.pdf",
        titel: "Emigration – Geschichte der Emigration, speziell der italienischen Emigranten in der Schweiz",
        text: "Überarbeitete, dichter gesetzte Fassung der Kursarbeit von G. Petrazzo zur italienischen Emigration in die Schweiz, inhaltlich im Wesentlichen mit der ersten Fassung übereinstimmend. Sie behandelt die Abgrenzung der Eroberungsepoche vom modernen Auswandern, die Behandlung italienischer Arbeiter und Aktivisten durch die Schweizer Behörden, das Entstehen der Fremdenpolizei und die Nachkriegsmigration im Zeichen von Marshallplan und Überfremdungsangst. Der persönliche Abschluss schildert Erfahrungen als Kind der zweiten Emigrantengeneration. Neu sind die Seitenzahlen, ein Beiblatt zu den Redeverboten des Bundesrats und eine Literaturliste zu Italien und zur Schweiz. Familientherapie Kurs II, 1989.",
    },
    Zusammenfassung {
        datei: "Emotioneller_Prozess_in_der_Kernfamilie_Möglichkeiten_zur_02.02.1987.pdf",
        titel: "Emotioneller Prozess in der Kernfamilie – Möglichkeiten zur Stressabsorption oder Adaptionsmechanismen einer Familie auf Überlastungssituationen",
        text: "Eine Familie wird als lebendiger Organismus beschrieben, der auf Überlastung mit Anpassungsmechanismen reagiert; diese sind zunächst nichts Krankhaftes und werden erst pathogen, wenn sie lange andauern, ohne die Ursache zu beheben (Bild des «run away feedback loop», aufgehobene Monokausalität). Davatz unterscheidet drei Mechanismen einer Kernfamilie unter Stress: den Ehekonflikt als Dominanzkampf, das Krankheitssymptom bei einem Ehepartner (körperlich, psychisch oder sozial, meist beim Unterlegenen oder Überfunktionierenden) und die Weitergabe der elterlichen Stressreaktion an ein Kind, bei dem dann ein Symptom ausbricht. Welcher Mechanismus gewählt wird, hängt weniger von der Belastung als von der «Programmierung» durch die eigene Erziehung ab. Skript für den Familientherapie-Kurs 1987.",
    },
    Zusammenfassung {
        datei: "Ethnisch_gemischte_Familie_28.08.2000.pdf",
        titel: "",
        text: "Das Skript von U. Davatz (Familientherapie-Kurs I, 2000) behandelt ethnisch gemischte Familien. Die ethnische Zugehörigkeit prägt die Vorstellung von der Normfamilie. Bei Mischehen kommt es in kritischen Situationen zu Territorial- und Machtkämpfen der Kulturen, Religionen und Gebräuche. Zu beachten sind der Cut-off von der Ursprungsfamilie, der in einer Mischheirat steckt, die Geschlechterrollen der jeweiligen Ethnie im Ehekonflikt und die Kindererziehung, meist nach der Ethnie der Mutter. Kinder leben in zwei Kulturen und können in Loyalitätskonflikte geraten; der Therapeut sollte sich die Kulturen erklären lassen.",
    },
    Zusammenfassung {
        datei: "Familien-Systemtheorie_nach_Murray_Bowen_15.11.2005.pdf",
        titel: "",
        text: "Folienhandout im Umfang von 21 Seiten zur Familiensystemtheorie nach Murray Bowen, mit Inhaltsverzeichnis. Behandelt werden Grundregeln der Systemtheorie (Genogramm, Muster erkennen, «Fuzzy Logic», lernende Haltung, Joining), soziale Vererbung, Fusionsbeziehung und Symbiose, Triangulierung als Ausweitung eines Dominanzkampfes, die Anpassungsmechanismen Ehekonflikt, Symptom bei einem Partner und Projektion auf die Kinder, sowie Cut-off und Differenzierung von der Ursprungsfamilie. Weitere Kapitel gelten ADS und Borderline-Störung im Familiensystem, der Depression als «Verliererkrankheit» oder Totstellreflex und dem Erstgespräch mit Aufbau der therapeutischen Beziehung, einschliesslich des Umgangs mit Stillstand in der Therapie. Durchgehend werden Vergleiche mit dem Tierreich und therapeutische Faustregeln angeführt.",
    },
    Zusammenfassung {
        datei: "Familien_mit_Kind_mit_schwerer_Behinderung_oder_Krankheit_30.11.1998.pdf",
        titel: "",
        text: "Familien mit einem schwer behinderten oder kranken Kind sind das Thema dieses Skripts von U. Davatz (Familientherapie-Kurs I). Die Eltern neigen zur Überfokussierung auf das Kind. Diese ist bei akuter Krankheit hilfreich, behindert bei Dauer aber die Akzeptanz, die soziale Kompetenz des Kindes und die Entwicklung der Geschwister, die zu kurz kommen oder parentifiziert werden. Die Beratung soll den Geschwistern ein Recht auf eigene Existenz vermitteln und mit den Eltern an Verarbeitung und Akzeptanz arbeiten, weg von Mitleidsgedanken.",
    },
    Zusammenfassung {
        datei: "Familien_mit_POS-Kindern_24.08.2006.pdf",
        titel: "",
        text: "Stichwortartiges Skript für den Abschlusskurs Neuromotorik über Familien mit POS-Kindern. Früher seien Eltern zu Unrecht für die Symptomatik verantwortlich gemacht worden; heute bleibe der Umgang schwierig. Besprochen werden Lern- und Aufmerksamkeitsstörung, Hypersensibilität (die Kinder nehmen Spannungen auf und agieren sie aus, werden zu Störenfrieden und Sündenböcken), der Ehekonflikt, in den das POS-Kind leicht trianguliert wird, schlechte Impulskontrolle und Zeitverständnis. Zum Erziehungsstil: emotional geladen, bestrafend, drohend, zu rigide oder zu nachgiebig sei jeweils ungünstig; förderlich sei nur ein klarer, ruhiger, flexibler Stil mit erlernbaren Regeln.",
    },
    Zusammenfassung {
        datei: "Familiendynamik_mit_einem_POS-ADS-Kind_06.08.2003.pdf",
        titel: "",
        text: "Schriftlicher Text von Dr. med. U. Davatz zur Familiendynamik bei einem Kind mit POS/ADS. Das Kind sei Herausforderung und Lernchance für die Eltern: Reine «top down»-Erziehung führe zu Hilflosigkeit oder zu pathologischer Entwicklung (Delinquenz, Depression, Sucht, Schizophrenie). Behandelt werden die unterschiedlichen Erziehungsstile von Mutter und Vater, die sie als Teamarbeit statt Konkurrenz führen sollen, das POS-Kind als «Seismograph» und Indikator familiärer Störungen, die Gefahr für die Geschwister sowie die Bedeutung einer belastbaren Partnerbeziehung. Gefordert wird fachliche Begleitung der Eltern ohne Schuldzuweisung; erwähnt wird eine monatliche Angehörigengruppe in Baden, in der das POS-Kind zum «Katalysator elterlichen Lernens» wird.",
    },
    Zusammenfassung {
        datei: "Familiendynamik_mit_einem_POS-ADS-Kind_06.08.2003_2.pdf",
        titel: "",
        text: "Überarbeitete Fassung des Textes von Dr. med. U. Davatz zur Familiendynamik bei einem Kind mit POS/ADS; die ersten vier Seiten entsprechen der ersten Fassung, ab Seite 5 ist umformuliert. Im Zentrum stehen die unterschiedlichen Erziehungshaltungen der Eltern, das POS-Kind als «Seismograph» familiärer Spannungen, die Lage der Geschwister und die Pflege der Partnerbeziehung als Prävention. Ergänzt ist der Hinweis, dass dies nie als Schuld der Eltern an der Symptomentwicklung zu verstehen sei, was früher familientherapeutisch Tätige oft taten und als «Kunstfehler» bezeichnet wird. Therapeutisch sollen Eltern lernen, Kind und sich selbst zu beobachten und einen klaren Rahmen mit flexibler Konstanz statt rigider Regeln zu schaffen.",
    },
    Zusammenfassung {
        datei: "Familienschema_od._Familiendiagramm_od._Genogramm_29.10.2026.pdf",
        titel: "",
        text: "Leeres Musterblatt für ein Familienschema, Familiendiagramm oder Genogramm im Querformat. Vorgesehen sind Felder für Grosseltern mütter- und väterlicherseits, Onkel, Cousins, Eltern und Kinder, jeweils mit Geburts- und Todesdaten, Beruf und Wohnort. Eine Legende erklärt Symbole für Heirat, Trennung, Scheidung, Adoption, Abort, Zwillinge und den Indexpatienten. Das Blatt dient als Arbeitsinstrument für die Familientherapie.",
    },
    Zusammenfassung {
        datei: "Familientherapeutische_Interventionen_in_der_22.11.1988.pdf",
        titel: "",
        text: "Davatz stellt Grundregeln für familientherapeutische Interventionen nach Bowen zusammen. Symptome entstehen als evolutiver Prozess; der Therapeut soll deshalb Fragen nach dem Wie, Was, Wo und Wann statt direkter Warum-Fragen stellen, was schon selbst verlangsamend wirkt. Zentrales Problem ist die mangelnde Abgrenzung der Familienmitglieder, die Angst und Symptomatik aufschaukelt; die Intervention besteht darin, zu ruhiger Abgrenzung zu ermutigen, am besten durch das Nachlernen in der Ursprungsfamilie. Weitere Punkte sind das Vermeiden des Hineingezogenwerdens in den Familienprozess (Humor, klare eigene Position), vorsichtig formulierte Ratschläge, Umgang mit nicht befolgten Ratschlägen ohne Enttäuschung und der Verzicht auf Techniken, die nicht zur eigenen Person passen. Kurs in Familientheorie II, 22.11.1988.",
    },
    Zusammenfassung {
        datei: "Familientherapie_nach_Bowen_mit_einer_Person_im_02.03.2007.pdf",
        titel: "Familientherapie nach Bowen mit einer Person im sozial-psychiatrischen Setting",
        text: "Stichwortartige Unterlage für eine Weiterbildung zur Familientherapie nach Bowen mit einer Einzelperson im sozialpsychiatrischen Setting. Ausgangspunkt: psychiatrische Krankheiten sind funktionelle Hirnstörungen und Ausdruck gestörter Beziehungen, beeinflussbar durch Medikamente, durch Psychoanalyse oder durch Familientherapie. Der Einzelne wird im Familienkontext gesehen und mit dem Auftrag in die Familie geschickt, Muster zu beobachten und kleine Verhaltensänderungen zu erproben, was das ganze System verändere. Genannte Bowen-Konzepte sind Fusion, Cut-off, Triangle, Mehrgenerationenperspektive, Projektion und Differenzierung. Bei psychotischen, Borderline- und Suchtpatienten werden eher die Eltern beraten, bei Ehetherapie wird an der Entwicklung der Partner in Bezug auf ihre Herkunftsfamilie gearbeitet.",
    },
    Zusammenfassung {
        datei: "Führungsposition_im_Zusammenhang_mit_Geschwisterposition_05.11.1989.pdf",
        titel: "",
        text: "Kursarbeit der Teilnehmerin Heidi Deplazes-Kunz im Kurs «Familienberatung II» über Führungsposition und Geschwisterposition nach Walter Tomans «Familienkonstellationen». Position und Geschlecht in der Geschwisterreihe prägen lebenslang Verhaltenspräferenzen: Ältere haben eine Führungs- und Verantwortungsrolle, Jüngere sind eigenwilliger und weniger frustrationstolerant, mittlere flexibel, aber oft oppositionell; Altersabstände und Untergruppen beeinflussen das. Partnerbeziehungen sind laut Toman günstiger und dauerhafter, wenn die Rollen komplementär sind, bei gleichem Rang oder fehlendem Geschwister des anderen Geschlechts drohen Rang- und Geschlechtskonflikte. Zuletzt wird festgehalten, dass Tomans Befunde an unauffälligen Personen gewonnen wurden und bei besonderer Elternfokussierung abweichen.",
    },
    Zusammenfassung {
        datei: "Gedanken_im_Anschluss_an_die_Tagung_Therapeutische_23.03.2006.pdf",
        titel: "Gedanken im Anschluss an die Tagung «Therapeutische Jurisprudenz»",
        text: "Gedanken der Referentin nach der Tagung «Therapeutische Jurisprudenz: Homo invalidus – oder der wertlose Mensch» vom 23.3.2006. Sie unterstützt das Motto der 5. IV-Revision «Eingliederung vor der Rente», fragt aber, wer die Früherfassung leistet: Die IV komme zu spät, Krankenkassen und IV arbeiteten nicht zusammen, und Hausärzte, die die psychosozialen Belastungen kennen, hätten weder Zeit noch Casemanager. Vorgeschlagen wird, dass die IV die Früherfassung beim Hausarzt über Institutionen wie Lungenliga oder Pro Infirmis in Auftrag gibt. Chronische Leiden seien selten rein medizinisch, die Rente heile nicht; Rentenmissbrauch sei die Ausnahme, und Politik und Medien verzerrten das Bild. Juristen könnten als Kämpfer auch eine Casemanager-Rolle übernehmen; nötig sei frühere, unkonventionelle Zusammenarbeit der Disziplinen.",
    },
    Zusammenfassung {
        datei: "Gedanken_zum_Thema_Ablösung_von_Alleinerziehern_15.08.1988.pdf",
        titel: "",
        text: "Kursarbeit von L. Müller über die Ablösung von Alleinerziehern. Ausgangspunkt ist die übliche Aufteilung elterlicher Rollen (Vater: Autorität und Prinzipien; Mutter: Pflege und Vermittlung); fehlt ein Teil, steht das Kind einer ungeteilten Macht gegenüber und kann seine Grenzen nur an einer Person erproben. Der alleinerziehende Elternteil ist oft überfordert, sodass bewusste Erziehung und Förderung unterbleiben oder das Kind sich nicht behaupten kann; die Ablösung erfolgt dann häufig nur scheinbar oder verspätet, etwa durch cut-off oder Heirat. Sie verläuft schrittweise in Teilbereichen und braucht dazu andere Gruppen sowie die Sicherheit der Familie. Kurs in Familientherapie, August 1988.",
    },
    Zusammenfassung {
        datei: "Gedanken_zum_Tod_in_der_Familie_13.11.2003.pdf",
        titel: "",
        text: "Vortrag in einer Praxis über den Umgang mit dem Tod in der Familie. Davatz beschreibt den Tod als Tabu der individualisierten Gesellschaft und geht auf den Verlust eines Kindes (Gefahr des «Ersatzkindes», Schuldgefühle der Geschwister), eines Partners oder Elternteils und eines Geschwisters ein. Hilfreich seien das Brechen des Tabus, zeitlich begrenzte Rituale, das offene Aussprechen auch von Wut gegenüber dem Verstorbenen («zuerst Wut, dann Trauer, dann Loslassen») sowie die Suche nach einem Sinn. Wer sich so mit dem Tod auseinandersetzt, könne daraus Energie für das Leben gewinnen.",
    },
    Zusammenfassung {
        datei: "Gerechtigkeit_07.03.1988.pdf",
        titel: "",
        text: "Frühe, rein theoretische Kursarbeit von Ch. Hofmann zum Gerechtigkeitsbegriff in der Familientheorie. Ausgehend von Platon, Austin und Walster sowie Gouldner wird dargestellt, dass erlebte Ungerechtigkeit unangenehme Gefühle auslöst und zum Ausgleich motiviert. Mit Boszormenyi-Nagy werden Gerechtigkeit als Gegenseitigkeitsbilanz («Hauptbuch») der Familie, ihre Übertragbarkeit über Personen und Generationen sowie das Zusammenspiel von Gerechtigkeit, Macht und Schuld beschrieben. Beteiligte können ihr Unbehagen durch Verzerrung der Tatsachen mindern und neu definieren, was gerecht ist; Hess und Handel sowie Buchholz und Strauss werden dazu herangezogen.",
    },
    Zusammenfassung {
        datei: "Gerechtigkeit_in_Beziehungen_11.1988.pdf",
        titel: "",
        text: "Kursarbeit von Ch. Hofmann, ausgebaute Fassung der früheren Arbeit zur Gerechtigkeit. Sie fasst die Theorie (Platon, Austin und Walster, Gouldner, Boszormenyi-Nagy, Hess und Handel, Buchholz und Strauss) zusammen: Gerechtigkeit als Gegenseitigkeitsbilanz einer Familie, verknüpft mit Macht und Schuld und stets neu definierbar. Daraus folgt, dass Gerechtigkeit in Familie und Therapie nie absolut ist, sondern ein dynamisches Gleichgewicht, in das auch der Therapeut sein eigenes Gerechtigkeitserleben einbringt. Drei anonymisierte Beispiele illustrieren dies: Gerechtigkeit unter Geschwistern aus der eigenen Herkunftsfamilie, zwischen den Generationen bei einer Patientin mit Anorexie und in einem instabilen Familiensystem mit mehreren psychiatrischen Krisen, an dem die Grenzen des Verstehens sichtbar werden.",
    },
    Zusammenfassung {
        datei: "Geschwisterposition_29.10.2000.pdf",
        titel: "",
        text: "Die Position in der Geschwisterreihe prägt nach diesem Skript von U. Davatz (Familientherapie-Kurs I, 2000) die Persönlichkeit. Es stellt die acht Geschwisterpositionen des Psychologen Walter Toman vor und beschreibt Merkmale der Ältesten (Verantwortung, Hüter der Tradition), der Jüngsten (kompetitiv, kreativ, Regelbrecher) und der Mittleren («Sandwich»-Kinder mit flexibler Orientierung). Fällt ein Ältester aus seiner Rolle, kann ein mittleres Geschwister die Rolle des funktionellen Ältesten übernehmen. Therapeutisch sollen Älteste entlastet, Jüngste zur Verantwortung aufgefordert und Mittlere bei der Klärung ihrer Rolle unterstützt werden.",
    },
    Zusammenfassung {
        datei: "Gibt_es_das_auch_Depressionen_bei_geistig_behinderten_28.10.2026.pdf",
        titel: "Gibt es das auch? Depressionen bei geistig behinderten Jugendlichen und Erwachsenen",
        text: "Vortragstext von Ursula Davatz «Gibt es das auch? Depressionen bei geistig behinderten Jugendlichen und Erwachsenen» (Thema des Insieme-Vortrags vom 10.3.2003). Depression wird als «Verliererkrankheit» verstanden; geistig Behinderte erlebten sich nicht von sich aus als Verlierer, würden es aber durch Über- oder Unterforderung ihres Umfelds. Anhand des dreiteiligen Gehirns (Reptilien-, limbisches und kognitives Hirn) wird erklärt, dass ihr emotionales Gehirn fein wahrnimmt, sie nicht intellektualisieren können und ehrlich reagieren. Aufgezählt werden Umfeldfaktoren wie unverarbeitete Akzeptanz der Behinderung, Ehrgeiz, Bevormundung, Ungeduld, gereizte Stimmung und abruptes Unterbinden des Sexualverhaltens; geraten wird zu mehr Einfühlung und Aufmerksamkeit.",
    },
    Zusammenfassung {
        datei: "Hilfe,_mein_Kind_wird_gemobbt_07.11.2002.pdf",
        titel: "",
        text: "Mobbing unter Kindern wird im Vortrag für den Elternzirkel Reinach als Gruppenmechanismus beschrieben: Eine gestresste Gruppe wählt einen Sündenbock, um sich von allem Dysfunktionalen zu «säubern». Gefährdet ist, wer aus der Gruppe herausragt, positiv wie negativ (Aussehen, Kleidung, Verhalten, Glauben); POS-Kinder seien durch ihr Störverhalten besonders exponiert. Eltern sollen weder zur Anpassung drängen noch das Kind als Opfer verteidigen, sondern verschiedene Sichtweisen aufzeigen und dabei emotional voll hinter dem Kind stehen. Kommt der Gruppenprozess aus dem Ruder, sollen sie sich ohne Schuldzuweisung an die verantwortliche Person wenden, denn Mobbing sei Zeichen schwacher Führung.",
    },
    Zusammenfassung {
        datei: "Ist_die_menschliche_Brutpflege_in_unserer_globalisierten_15.05.2002.pdf",
        titel: "Ist die menschliche Brutpflege in unserer globalisierten Gesellschaft ernsthaft gefährdet?",
        text: "Ausgehend von zwanzig Jahren Supervision für die Mütter- und Väterberatung fragt der Vortrag, ob die menschliche Brutpflege durch die globalisierte Wirtschaft gefährdet ist. Er beschreibt die Umkehr von Jagd und Nest: Die Familie dient heute dem Beutezug nach Geld, statt dass der Beutezug die Familie versorgt. Folgen seien Scheidungen, schwindende Elternkompetenz, Jugendgewalt und steigende Sozial- und Gesundheitskosten. Die Mütter- und Väterberatung wird als langfristige, präventive Investition dargestellt (Hilfe zur Selbsthilfe, Entlastung der Mutter, Einbezug des Vaters); als Beleg dienen Early-Intervention-Studien aus Kanada. Gehalten für die Beratungsstelle des Bezirks Baden.",
    },
    Zusammenfassung {
        datei: "Jugend_und_Alkohol_05.04.2005.pdf",
        titel: "",
        text: "Vortrag in Birr zu Jugend und Alkohol: Alkohol sei das gesellschaftlich eingebundene Suchtmittel der westlichen Welt und bleibe ein Getränk für Erwachsene. Die Wirtschaft habe Jugendliche mit Alkopops als Konsumenten erschlossen. Jugendliche trinken, um sich erwachsen und stark zu fühlen; starke Fremdkontrolle steigere den Konsum eher, und Alkohol dient als Problemlöser, weshalb Jugendliche mit grossen Problemen besonders gefährdet sind (Tierexperiment mit Ratten). Die Referentin rät zu einer klaren, nicht als Verbot formulierten Haltung der Erwachsenen, ohne Bestrafung bei Übertretung, und sieht die Grenze persönlich bei 20 Jahren, den Staat bei 16.",
    },
    Zusammenfassung {
        datei: "Jugendliche_und_Sucht_Freiheit_oder_Sackgasse_auf_dem_Weg_02.12.2003.pdf",
        titel: "Jugendliche und Sucht – Freiheit oder Sackgasse auf dem Weg der Suchenden",
        text: "Vortrag bei La vida Wohlen über Jugendliche und Sucht. Davatz beschreibt die Pubertät als Übergang von der Erziehungs- zur Sozialisierungsphase und erklärt die Anziehung von Drogen aus Neugier, Wunsch nach schneller Gefühlsveränderung, Langeweile und Leistungsdruck; die vermeintliche Freiheit ende in der Abhängigkeit. Eltern sollten als «konservative Platzhalter» ihre Haltung klar vertreten, ohne Befehl oder Verbot, dem Jugendlichen die Verantwortung für sein Drogenverhalten überlassen und dennoch die Beziehung zur ganzen Person nicht abbrechen. Fehler von Eltern, Fachleuten und Politikern sieht sie in Anbiederung an den Zeitgeist, Konfliktscheu und Kontrollübernahme; Sucht sei eine Sackgasse, der nüchterne Zustand bedeute mehr Freiheit.",
    },
    Zusammenfassung {
        datei: "Kinder-_Jugend-_und_Familientherapie_04.03.2008.pdf",
        titel: "",
        text: "Vortrag vor dem BDK (4.3.2008) über Kinder-, Jugend- und Familientherapie. Die Familie wird als kleinste sozialpolitische Einheit definiert, deren Therapeut «Klempner» und Coach zugleich sei; das Ziel sei die gesunde Entwicklung aller Beteiligten, nicht der Erhalt eines dysfunktionalen Systems um jeden Preis. Beschrieben werden Rollen von Kindern im System (parentifiziertes ältestes Kind, Vermittler- und Trösterrolle, komplementäre Anpassung bei Geschwistern), die Ablösungsprobleme funktionalisierter Jugendlicher und die Arbeit mit einem Dreigenerationen-Genogramm. Kindertherapie sei aus dieser Sicht immer Familiensystemtherapie, bei der zuerst die Eltern behandelt werden.",
    },
    Zusammenfassung {
        datei: "Kompetenter_Umgang_von_Eltern_und_Lehrern_mit_POS-ADHS-Kind_24.02.2005.pdf",
        titel: "Kompetenter Umgang von Eltern und Lehrern mit POS/ADHS-Kind als Prävention einer Sekundärerkrankung",
        text: "POS/ADS/ADHS wird als Normvariante des Gehirns statt als Krankheit betrachtet; Eltern und Erzieher sollen kompetent damit umgehen, damit sich keine Sekundärerkrankung entwickelt (Primärprävention). Der Vortrag nennt Ursachen (genetisch, Knaben häufiger betroffen, oft überdurchschnittliche Intelligenz bei unausgeglichenem Leistungsprofil) und den Teufelskreis, in dem das Kind zum «dummen und bösen» gestempelt wird. Faustregeln: sich als Erzieher zuerst selbst beruhigen, Emotionen und Aggressionen beruhigen statt bestrafen, Lernstörungen differenziert erfassen und nicht als bösen Willen deuten, Begabungen fördern statt als Strafe entziehen. Zum Schluss stehen allgemeine Erziehungsregeln: elastisch, wenig Strafe, lange Leine, einig in den Regeln und wenige klare Worte. Gehalten bei der Spitex Wolfwil.",
    },
    Zusammenfassung {
        datei: "Konstruktiver_Umgang_mit_Aggression_und_Gewalt_im_01.02.2000.pdf",
        titel: "Konstruktiver Umgang mit Aggression und Gewalt im Pflegealltag",
        text: "Fortbildungsunterlage von U. Davatz für eine Weiterbildung am KSB vom 1.2.2000 zum Umgang mit Aggression und Gewalt im Pflegealltag. Aggression entsteht bei Patienten durch Ausgeliefertsein und Kontrollverlust, beim Pflegepersonal durch Überforderung, Verunsicherung oder Kritik; Gewalt setzt dagegen immer eine übergeordnete Position voraus. Rückzug oder Gegenaggression des Personals verstärken die Eskalation. Empfohlen werden, sich eigene Aggressionen einzugestehen und ihre Ursachen zu analysieren, das Verhalten des Patienten aus seiner Lebensgeschichte zu verstehen, Ziele herabzusetzen und Hilfe durch Dritte, Rollenspiel oder Fallsupervision zu nutzen. Fallbeispiele sollten besprochen werden.",
    },
    Zusammenfassung {
        datei: "Krankheit_als_Anpassung_28.10.1989.pdf",
        titel: "",
        text: "Referat von Jakob Wehrli im Kurs II/89 über Krankheit als Anpassung an Stress und Überforderung. Nach einer Rückschau auf Bowens Anpassungsmechanismen und die Macht des Kranken im Familiensystem stellt er das Buch von Bernd Frederich («Zuflucht in der Krankheit suchen») vor, das Krankheit als Lösung bei Beziehungsangst versteht und zwei Typen unterscheidet: «Angst vor Schwäche» und «Angst vor Fehlern». Danach referiert er die Theorie von David und Doris Jonas zu archaischen Relikten (Neotenie, Totstellreflex, Dominanz- und Fluchtreflexe), die psychosomatische Symptome wie Ohnmacht, Angina pectoris oder Herzrhythmusstörungen erklären sollen, und deren idiolektische Gesprächsführung. Mit Literaturverzeichnis.",
    },
    Zusammenfassung {
        datei: "Krankheit_als_Konfliktlösung_28.10.2026.pdf",
        titel: "",
        text: "Kursunterlage ohne Verfasserangabe über Krankheit als Konfliktlösung. Körperkrankheit wird zunächst als seelischer Selbstheilungsversuch beschrieben, etwa zur Entlastung zwischen Ich und Über-Ich, zur Wiederherstellung verletzter Selbst-Grenzen oder zur Verarbeitung von Verlust. Sodann wird sie als Teil eines Systems betrachtet, in dem sie Konflikte von der Beziehungs- auf die Körperebene verlagert und das System stabilisieren kann. Abschliessend ordnet der Text sie in ein bio-psycho-soziales Modell ein und bemerkt, dass vermutete Zusammenhänge etwa bei Krebs in grossen Untersuchungen nicht bestätigt wurden.",
    },
    Zusammenfassung {
        datei: "Krisen_in_der_Familie_als_therapeutische_Chance,_betrachtet_30.10.2002.pdf",
        titel: "Krisen in der Familie als therapeutische Chance, betrachtet durch verschiedene interkulturelle Aspekte",
        text: "Wie entstehen Krisen in Familien, und wie nutzt man sie therapeutisch? Der Vortrag für eine Weiterbildung zu Krisen beschreibt Familienregeln, die bei Abweichung (z.B. in der Pubertät) versagen, worauf professionelle Helfer erwartungsgemäss «mehr vom Gleichen» machen sollen. Familientherapeuten dürfen weder auf das System hereinfallen noch zum Advokaten des Patienten werden, müssen Krise und Notfall aus Angst unterscheiden, zwischen Subsystemen übersetzen und deeskalierend wirken. Bei Migrantenfamilien kommt die Kulturübersetzung hinzu: Der Therapeut darf weder die Hierarchievertreter übergehen noch die junge Generation bremsen und muss sich von der eigenen Kultur dezentrieren. Leitgedanke: ruhig bleiben, Geduld haben, «Primum nihil nocere».",
    },
    Zusammenfassung {
        datei: "Literaturliste_für_Kurs_in_Familientheorie_25.02.1991.pdf",
        titel: "",
        text: "Literaturverzeichnis zum Kurs in Familientheorie vom 25.2.1991 mit Büchern und Aufsätzen zu Familientherapie, Familiensystemen und biologischen Grundlagen des Sozialverhaltens. Genannt werden unter anderem Murray Bowen («Family Therapy in Clinical Practice»), Genogramm- und Stieffamilien-Literatur, Walter Toman zu Familienkonstellationen sowie Elisabeth Badinter zu Mutterliebe und Paarbeziehung. Daneben stehen Werke zur Soziobiologie (E. O. Wilson), zur Erkenntnistheorie (Maturana und Varela) und zur Verhaltensforschung an Primaten (Kummer, Gubernick und Klopfer). Eine eigene Arbeit von Ursula Davatz von 1978 über Fusion und Differenzierung bei Tier und Mensch ist ebenfalls aufgeführt.",
    },
    Zusammenfassung {
        datei: "Loslassen,_bedingungslos_loslassen_25.05.2002.pdf",
        titel: "",
        text: "Im Seminar für Eltern von Pubertierenden geht es um die Gratwanderung zwischen Halt geben und Loslassen. Als Gründe, warum Eltern nicht loslassen können, nennt der Vortrag Mutter- und Vaterinstinkt, ein Handicap oder Symptome des Kindes, Schuldgefühle, Angst vor Kritik von aussen sowie allgemeine Ängstlichkeit. Festhalten hemme die Entwicklung und könne Krankheit verstärken. Eltern sollen in der Pubertät von der Erziehung zur Beziehung wechseln: keine Befehle, klare Haltungen, Verhandlungsbereitschaft, Vertrauen in sich und das Kind, Hinwendung zu eigenen Tätigkeiten und zur Paarbeziehung. Bei Krankheitssymptomen wird familientherapeutische Beratung empfohlen.",
    },
    Zusammenfassung {
        datei: "Loyalität_und_Familie_04.09.1989.pdf",
        titel: "",
        text: "Kursarbeit von Canzek über das Loyalitätsprinzip in Familien, im Wesentlichen nach Boszormenyi-Nagy («Unsichtbare Bindungen»). Loyalität wird als moralisch-ethische Dimension verstanden, die in einer unsichtbaren Schuld- und Verdienstbuchführung festgehalten wird; auffälliges Verhalten kann mit der Familienloyalität in Einklang stehen. Behandelt werden die Weitergabe von Loyalitätsverpflichtungen über Generationen, acht notwendige Loyalitätsübertragungen im Lebenslauf, Loyalitätskonflikte in der Ehe zwischen den beiden Herkunftsfamilien und die Loyalitätsspaltung bei Kindern. Therapeutisch werden Symptome als nicht erkennbare Loyalität gegenüber der Herkunftsfamilie gedeutet, die in erkennbare und anerkennbare umzuwandeln ist. Kurs II in Familientherapie, Leitung U. Davatz, 1989.",
    },
    Zusammenfassung {
        datei: "Meine_Familie_aus_systemischer_Sicht_von_der_Herkunft_bis_12.1990.pdf",
        titel: "Meine Familie aus systemischer Sicht: von der Herkunft bis heute",
        text: "Fragebogen für Kursteilnehmende, die ihre eigene Herkunftsfamilie aus systemischer Sicht untersuchen, nach Unterlagen von Sepp Duss-von Werdt (Dezember 1990). Er führt durch das Zeichnen eines Genogramms und Familienstrukturen zu verschiedenen Lebensaltern, das Beziehungsrad und danach durch Fragen zu Subsystemen, Störungen, Rollen, kritischen Ereignissen, Tod und Trauer, Kommunikation, Regeln, Werten, Familiengeschichten und Geheimnissen. Weitere Themen sind Erziehung, Wohnung und Umgebung, Schule, Berufswelt, soziale Stellung, Aussenkontakte und Zeitgestaltung. Die Fragen werden anschliessend auf die jetzige Lebenssituation angewendet; zwei Beilagen mit Symbolen zum Genogramm und einem Beziehungsrad-Beispiel schliessen das Dokument ab.",
    },
    Zusammenfassung {
        datei: "Meine_Themen_im_Familienkurs_II_25.04.2000.pdf",
        titel: "",
        text: "Persönlicher Bericht eines Teilnehmers des Familienkurses II (2000, Name geändert) über seine Themen: die Rolle des jüngeren Bruders und die Vaterrolle. Er schildert seinen Lebenslauf, seine berufliche Entwicklung als Arzt und seine Entdeckung der Familientherapie und der Familienforschung. Ausführlich beschreibt er seine Herkunftsfamilie über mehrere Generationen, Gespräche mit Eltern und Geschwistern sowie die Veränderungen in der eigenen Familie. Als Ziel nennt er, seine Rolle als jüngerer Bruder anzunehmen und als Vater vom passiven zum aktiven, fordernden und stützenden zu werden.",
    },
    Zusammenfassung {
        datei: "Mobbing_am_Arbeitsplatz_11.04.2002.pdf",
        titel: "",
        text: "Mobbing wird als altes soziales Phänomen beschrieben: Ein Kollektiv unter Stress erklärt ein Mitglied zum Sündenbock und projiziert die eigene Dysfunktion auf es. Der Vortrag im Reusspark führt aus, wie Regeln, Rollen und Hierarchien in Gruppen wirken und warum die heutige Arbeitswelt mit ihrer Beschleunigung und «Quantitätskontrolle» Mobbing begünstigt. Besonders gefährdet sind Personen, die aus dem Kollektiv herausragen, Veränderungen anstossen oder sich nicht an Regeln halten. Eingreifen ist Sache der Führung: Ursachen des Stresses suchen, Konflikte klären, einen gemobbten «Märtyrer» mit seinen Ideen aufnehmen und einen Querschläger selbst disziplinieren.",
    },
    Zusammenfassung {
        datei: "Mutterschaft_und_Krise_und_Eltern_werden_Eltern_sein_24.05.2002.pdf",
        titel: "Mutterschaft und Krise und Eltern werden – Eltern sein",
        text: "Die Geburt eines Kindes ist für die Familie eine natürliche Krisensituation. Der Vortrag für einen Nachdiplomkurs von Hebammen nennt Krisenauslöser bei der Mutter: eine nicht tragfähige Partnerbeziehung, eine ungelöste Beziehung zur eigenen Mutter, Konflikte mit der Schwiegermutter und Probleme oder Krankheit beim Kind. Hebammen sollen sich über das Beziehungsnetz informieren, die Mutter ins «Hier und Jetzt» führen und ihr zeigen, dass sie Hilfe annehmen darf und den Partner anleiten muss. Im zweiten Teil geht es um den Wechsel von der Paar- zur Dreierbeziehung, die Gefahr, dass das Kind zum «Ehetherapeuten» wird, und Fragen für den Geburtsvorbereitungskurs.",
    },
    Zusammenfassung {
        datei: "POS-ADS_Sinnvoller_Umgang_damit_im_Kindes-_Und_28.10.2004.pdf",
        titel: "POS/ADS – Sinnvoller Umgang damit im Kindes- und Erwachsenenalter",
        text: "Vortrag vor der Frauengemeinschaft Gebenstorf über POS/ADS (heute ADS, ADD, ADHS) bei Kindern und Erwachsenen. Aufgeführt werden Symptome (Aufmerksamkeits-, Wahrnehmungs-, motorische, Lern- und Gedächtnisstörungen, emotionale Hypersensibilität und schlechte Impulskontrolle, daneben einzelne Begabungen), Ursachen (genetisch, leichte Hirnschädigung rund um die Geburt, Knaben häufiger betroffen) und der Umgang damit. Kinder sollen für ihre Defizite nicht bestraft, Leistungen an ihren eigenen Fortschritten gemessen, Motorik und Lernstörungen gezielt gefördert und emotionale Ausbrüche beruhigt statt bestraft werden. Im Erwachsenenalter bestehen die Störungen in schwächerer Form fort; ungeschickte Erziehung kann zu sekundären Erkrankungen führen, es gibt aber auch erfolgreiche Persönlichkeiten.",
    },
    Zusammenfassung {
        datei: "POS-ADS_Sinnvoller_Umgang_damit_im_Kindes-_Und_28.10.2004_2.pdf",
        titel: "POS/ADS – Sinnvoller Umgang damit im Kindes- und Erwachsenenalter",
        text: "Zweite Fassung des Gebenstorfer Vortrags über POS/ADS im Kindes- und Erwachsenenalter, hier mit anderem Text: Ablenkbarkeit kommt stark aus der inneren Gefühlswelt, Studium verlangt Sitzleder, Ritalin, Konzentrationsübungen, Rituale und Sport können helfen. Hohe Sensibilität wird als Hindernis in der Ellbogengesellschaft beschrieben, schlechte Impulskontrolle führe bei Männern zu Aggression nach aussen, bei Frauen eher zu Selbstverletzung. Der Erziehungsstil wirke mit: streng-aggressiv eher Delinquenz, ängstlich-korrekt eher Angst und Depression, hochemotional-erklärend eher Schizophrenie, flexibel-tolerant eher kreative Erwachsene. Abschliessend wird POS als kognitive Normvariante bezeichnet, die man nicht zu stark pathologisieren solle.",
    },
    Zusammenfassung {
        datei: "POS_bei_Erwachsenen,_Wie_gehe_ich_mit_dem_POS-ADD_am_22.05.2002.pdf",
        titel: "POS bei Erwachsenen – Wie gehe ich mit dem POS/ADD am Arbeitsplatz um?",
        text: "Das frühkindliche POS, heute ADD oder ADS genannt, wird als leichte funktionelle Hirnstörung ohne Minderintelligenz beschrieben, teils mit isolierten Hochbegabungen. Der Vortrag für eine Eingliederungsstelle in Basel geht die betroffenen Bereiche durch: Aufmerksamkeit, Wahrnehmung, Motorik, Augen-Hand-Koordination, Lernstörungen wie Legasthenie und Dyskalkulie, Impulskontrolle sowie erhöhte Sensibilität. Für Lehrmeister am Arbeitsplatz gibt er praktische Hinweise: ruhige Atmosphäre, kurze und konkrete Instruktionen, Aufmerksamkeit prüfen, bei Ausbrüchen beruhigen statt argumentieren, notfalls ein «Time-out», Stärken fördern. Menschen mit POS gelten dem Vortrag zufolge als gute soziale und emotionale Lehrmeister.",
    },
    Zusammenfassung {
        datei: "POS_bei_Erwachsenen,_Wie_gehe_ich_mit_dem_POS-ADD_am_22.05.2002_2.pdf",
        titel: "POS bei Erwachsenen – Wie gehe ich mit dem POS/ADD am Arbeitsplatz um?",
        text: "Zweite Fassung desselben Vortrags für die Eingliederungsstelle in Basel (mit dem Briefkopf der Praxis Ganglion). Das frühkindliche POS, heute ADD oder ADS, wird als leichte funktionelle Hirnstörung ohne Minderintelligenz dargestellt, die Aufmerksamkeit, Wahrnehmung, Motorik, Lernen, Impulskontrolle und Sensibilität betrifft. Für Lehrmeister am Arbeitsplatz folgen Ratschläge: ruhige Atmosphäre, kurze und konkrete Informationen, Beruhigung statt Moralisieren bei Ausbrüchen, Time-out, Stärken fördern. Textlich entspricht sie weitgehend der ersten Fassung.",
    },
    Zusammenfassung {
        datei: "POS_im_Erwachsenenalter_26.08.2004.pdf",
        titel: "",
        text: "Zwei Texte in einer Datei: ein Kurztext zur Ankündigung des Gebenstorfer Abends über den Umgang mit POS/ADS bei Kindern und Erwachsenen und das Vortragsskript zu POS im Erwachsenenalter. Behandelt werden Aufmerksamkeitsstörung und innere Ablenkbarkeit (Studium verlangt Sitzleder und Motivation), hohe Sensibilität als Handicap in der Wettbewerbsgesellschaft sowie schlechte Impulskontrolle, die bei Männern eher nach aussen, bei Frauen eher gegen sich selbst gerichtet sei. Der Erziehungsstil prägt laut Text den Verlauf: strenge Erziehung eher Delinquenz, ängstlich-genaue eher Angststörung und Depression, hochemotionale eher Schizophrenie, flexible mit klaren Richtlinien eher gesunde, kreative Erwachsene. POS im Erwachsenenalter sei eine kognitive Normvariante, mit Sport, Meditation, Musizieren und Konzentrationsübungen ähnlich hilfreich wie Ritalin.",
    },
    Zusammenfassung {
        datei: "POS_im_Kindes-_und_Erwachsenenalter_03.03.2004.pdf",
        titel: "",
        text: "Kurzer Vortrag für den Besuchsdienst PKK über POS (heute ADS/ADHS) im Kindes- und Erwachsenenalter. Davatz beschreibt die Symptomgruppen – Aufmerksamkeits-, Wahrnehmungs-, motorische und Lernstörungen sowie emotionale Hypersensibilität mit schlechter Impulskontrolle – und verweist auf genetische und perinatale Ursachen sowie auf den Nachweis mit bildgebenden Verfahren. Für den Umgang rät sie, Defizite nicht zu bestrafen, Fortschritte am eigenen Leistungsstand zu messen, emotionale Ausbrüche zu beruhigen und Überreizung zu vermeiden; erwähnt werden Psychomotorik, Ergotherapie, Legasthenietherapie und Ritalin. Im Erwachsenenalter blieben die Störungen abgeschwächt bestehen; durch ungeschickte Erziehung können Sekundärschäden wie Persönlichkeitsstörungen, Depression oder Sucht entstehen.",
    },
    Zusammenfassung {
        datei: "Paarbeziehungen_24.02.2004.pdf",
        titel: "",
        text: "Vortrag beim Frauenverein Egliswil über Paarbeziehungen aus der Sicht von Bindungsverhalten und Biologie. Davatz behandelt die Mutter-, Vater- und Elternpaarbeziehung als Grundlage der Kinderentwicklung, die Paarbindung im Dienst von Fortpflanzung und Brutpflege, die Paarbeziehung als Nachholbedarf aus der Kindheit (mit Verweis auf Bert Hellinger) und als Projektion ungelöster Elternkonflikte. Weitere Aspekte sind die Paarbeziehung als narzisstische Selbstbestätigung, als Ort gegenseitigen Wachstums und die heutigen Merkmale (Gleichberechtigung, Doppelbelastung, romantische Leitbilder, hohe Scheidungsrate). Schlussthese: Eine Paarbeziehung braucht Zeit, Geduld, Vertrauen und lebenslange Investition.",
    },
    Zusammenfassung {
        datei: "Patientenautonomie_und_Verwahrlosung_Recht_auf_Freiheit_25.10.2005.pdf",
        titel: "Patientenautonomie und Verwahrlosung – Recht auf Freiheit versus Pflicht zur Hygiene und Ordnung",
        text: "Vortrag beim Kantonalen Spitexverband Aargau zum Spannungsfeld zwischen dem Recht auf persönliche Freiheit und der Pflicht zu Hygiene und Ordnung bei der Pflege alter Menschen. Als Gründe der Verwahrlosung werden Depression und Schizophrenie, Protesthaltung und Sammeltrieb (Messies) genannt, der als Schutzwall und Angstverhalten verstanden wird. Zuerst müsse Kontakt zur Person hergestellt werden, erst danach Hygienemassnahmen; diese solle man als für die eigene Aufgabe wichtig darstellen, nicht als Bevormundung des Patienten. Drohung mit Dienstleistungsentzug sei unprofessionell, Grenzen des Tolerierbaren seien individuell, Angehörige sollen informiert, bei echter Gefahr die Vormundschaftsbehörde eingeschaltet werden. Leitlinie ist die Wahrung der Menschenwürde, auch bei fürsorgerischem Freiheitsentzug.",
    },
    Zusammenfassung {
        datei: "Pflegende_Angehörige_Dureschnufe_dank_Spitex_09.11.2002.pdf",
        titel: "",
        text: "Die Pflege kranker Angehöriger fällt in der Wettbewerbsgesellschaft meist den Frauen zu, die als mittlere Generation zugleich Kinder und Eltern oder Schwiegereltern pflegen und dabei überfordert werden. Der Vortrag am St. Galler Gesundheitstag beschreibt Schuldgefühle, die Tyrannei, die alte Menschen ausüben können, und die Folgen für die Kinder der Pflegenden. Die Spitex habe die Aufgabe, nicht nur zu pflegen, sondern Angehörige zu beraten und zu entlasten. Dafür brauche es Geschick und Geduld, eine neutrale, allparteiliche Haltung sowie Supervision oder Intervision. Die Entlastung der mittleren Generation wirke zugleich präventiv für die junge Generation («Dureschnufe dank Spitex»).",
    },
    Zusammenfassung {
        datei: "Prevention_in_the_field_of_law_and_mental_health_28.10.2026.pdf",
        titel: "",
        text: "Englischer Vortrag zu «Therapeutic Jurisprudence» am 28. Congress of Law and Mental Health in den Niederlanden. Davatz vergleicht die Denkweisen von Juristen (dualistisch, Strafe), Medizinern (statische Diagnosekategorien) und Systemtherapeuten (Konflikt und Prozess) und definiert Prävention als achtsames Eingreifen an kritischen Entwicklungspunkten einer Person oder Familie. Sie plädiert für das Studium individueller Lebensgeschichten Straffälliger statt reiner Statistik und sieht in der Unterstützung von Eltern sensibler, etwa ADS-betroffener Kinder eine wirksame Vorbeugung gegen Delinquenz. Recht und Psychiatrie sollten statt akademischer Machtkämpfe und Gutachten gemeinsam praktische Lösungen erarbeiten und auf Strafe als Korrektur verzichten, wo sie den Entwicklungsprozess verschlechtert.",
    },
    Zusammenfassung {
        datei: "Prävention_sinnvoller_Ansatz_bei_gefährdeten_Familien_mit_03.07.1989.pdf",
        titel: "Prävention – sinnvoller Ansatz bei gefährdeten Familien mit jungen Kindern",
        text: "Kursarbeit von E. Maarse Bäbler zur Prävention bei gefährdeten Familien mit kleinen Kindern, eingeleitet mit einem Märchen über die Grenzen perfekter Erziehungstheorien. Als Belastungsfaktoren nennt sie Schicksalsschläge wie Tod, Trennung oder ein behindertes Kind, die unverarbeitete eigene Kindheit, die Isolation der Kleinfamilie bei abwesenden Vätern und geringer Wertschätzung der Erziehungsarbeit, beengte Wohnverhältnisse, Geldprobleme und schichtspezifische Belastungen. Sie schlägt unter anderem Kontaktgruppen für Familien, kinderfreundliche Wohnumgebungen, Beteiligung der Väter an der Erziehung und die Möglichkeit für Mütter vor, neben dem Kind eigene Interessen zu behalten. Familientheoriekurs II, 1989.",
    },
    Zusammenfassung {
        datei: "Psychisch_Kranke_in_der_Spitex_13.11.2002.pdf",
        titel: "",
        text: "Aus familientherapeutischer Sicht ist der psychisch kranke Mensch oft Symptomträger eines dysfunktionalen Familiensystems; eine Hospitalisation wirke nur kurzfristig als «Sozialchirurgie». Der Vortrag für Spitex St. Gallen knüpft an die Sozialpsychiatrie der 68er-Jahre an und plädiert für die Pflege zuhause, mit Anleitung der Angehörigen und einer fachkundigen Fallführung. Die Spitex-Fachfrau soll sich zunächst im System orientieren, ohne Partei zu ergreifen, die Beziehung zum Patienten als wirksamstes Instrument nutzen und die Diagnose eher beiseite lassen. Einen Behandlungsauftrag, der nicht stimmt, soll sie nicht gehorsam ausführen, sondern zurückmelden.",
    },
    Zusammenfassung {
        datei: "Psychisch_Kranke_in_der_Spitex_13.11.2002_2.pdf",
        titel: "",
        text: "Zweites Exemplar des Vortrags für Spitex St. Gallen über psychisch Kranke in der Spitex; auf Seite 3 steht eine handschriftliche Notiz zu Kontrolle und Medikamentenabgabe. Inhaltlich: Psychisch Kranke sind aus familientherapeutischer Sicht oft Symptomträger des Familiensystems, weshalb die Pflege zuhause mit Unterstützung der Angehörigen und einer Fallführung angestrebt wird. Die Spitex-Fachfrau soll unparteiisch bleiben, die Beziehung zum Patienten und zur Familie als wichtigstes Instrument pflegen, Diagnosen nicht in den Vordergrund stellen und unstimmige Aufträge zurückgeben.",
    },
    Zusammenfassung {
        datei: "Psychisch_krank_scheininvalid_18.06.2005.pdf",
        titel: "",
        text: "Vortrag zum 20-jährigen Jubiläum der Stiftung Guyerweg, die Wohnen für psychisch Behinderte anbietet; die Referentin ist Stiftungspräsidentin. Sie kritisiert das materialistische Menschenbild der Medizin, in dem sichtbare Schäden anerkannt, seelisches Leiden aber angezweifelt werde, und wendet sich gegen die Sparpolitik, die Psychotherapie und Rehabilitation ins Zentrum der Kostendebatte rückt. Psychische Krankheiten seien dynamische Störungen des Gehirns als sozialem Organ und Langzeitkrankheiten; Medikamente seien nur Gehhilfen, die Hauptarbeit leisten der aktive Patient und sein Therapeut als Coach. Psychisch Kranke seien keine Scheininvaliden. Zum Schluss würdigt sie die Arbeit des Teams und die Wiedereingliederung der Bewohner.",
    },
    Zusammenfassung {
        datei: "Psychische_Krankheiten_und_ihre_Auswirkungen_auf_die_08.11.2007.pdf",
        titel: "Psychische Krankheiten und ihre Auswirkungen auf die betroffene Person",
        text: "Vortrag vor der Vereinigung Schweizer Amtsvormünder (8.11.2007) mit einem Überblick über psychische Krankheiten und ihre Folgen für die Betroffenen. Gegliedert wird nach Gefühlsstörungen (Depression als «Verliererkrankheit», Angst und Panik), Verhaltensstörungen (Zwänge als angstbindendes Verhalten, Persönlichkeitsstörungen, die sich in der Pubertät festsetzen) und Denkstörungen (Schizophrenie als «Systemoverflow» nach langem emotionalem Stress). Wahnhafte Bilder hätten einen metaphorischen Wert; die Denkstörung lasse sich nicht mit Logik, sondern nur über den Abbau des emotionalen Stresses angehen.",
    },
    Zusammenfassung {
        datei: "Psychoseerleben_in_der_Literatur_29.10.2026.pdf",
        titel: "",
        text: "Kurze Literaturliste nach Asmus Finzen (1993) zum Psychoseerleben in der Literatur. Genannt werden autobiografische Berichte Betroffener und Angehöriger, etwa von Dorothea Buck, Sylvia Plath und Susan Sheehan, sowie Texte von Büchner, Melville, Schnitzler, Handke und Bernhard, die psychotisches Erleben spiegeln. Aus dem französischen Sprachraum wird Philippe Djian angeführt. Christian Müller habe in einer Sammlung literarische Dokumente schizophrener Psychosen und anderer psychischer Störungen zugänglich gemacht.",
    },
    Zusammenfassung {
        datei: "Psychosomatische_Erkrankungen_am_Beispiel_von_15.08.1988.pdf",
        titel: "",
        text: "Kurstext von M. Schlaepfer zu psychosomatischen Erkrankungen am Beispiel von Herz-Kreislauf-, Verdauungs- und Bewegungsapparatstörungen. Nach Jores wird Krankheit als schrittweise, multifaktoriell entstehender Prozess verstanden, und die «Organsprache» (etwa Herz als Repräsentant der Liebe) gibt Hinweise auf den seelischen Anteil. Beschrieben werden typische Persönlichkeitsmerkmale bei Herzinfarkt, Herzrhythmusstörungen, Herzphobie und Darmerkrankungen sowie Schmerz am Bewegungsapparat als Konversionssymptom, bei dem Affekte in körperliche Symptome umgewandelt werden. Kurs in Familientheorie, August 1988.",
    },
    Zusammenfassung {
        datei: "Pubertät_18.03.2004.pdf",
        titel: "",
        text: "Die Pubertät gilt als Prüfung für die Selbstsicherheit der Eltern, Hauptthema ist der Ablösungskonflikt. Der Vortrag beschreibt den Machtkampf zwischen Jugendlichen und Eltern, der zur Ablösung dient: Die Eltern sind die Stärkeren und sollen ihrem Kind «Welpenschutz» geben, hinter dem Imponiergehabe stehe oft Angst. Empfohlen werden gemeinsam ausgehandelte Regeln statt dauernder Befehle sowie eine klare elterliche Haltung statt Verboten bei Alkohol, Drogen und Sexualität. Extreme Berufs-, Partner- und Peergroup-Wahlen deutet die Referentin als Zeichen übermässiger Kontrolle; psychische Störungen der Pubertät zeigten eine gestörte Ablösung an. Vortrag am Familienforum Münchenstein.",
    },
    Zusammenfassung {
        datei: "Pubertät_Rollenverlust_der_Eltern_24.04.2005.pdf",
        titel: "",
        text: "Die Pubertät als wichtigste Übergangsphase vom abhängigen Kind zum autonomen Erwachsenen, mit Entwicklung nach aussen (Autonomieinstinkt, Grenzen testen) und nach innen (Selbstunsicherheit, Verletzlichkeit, oft hinter Imponiergehabe verdeckt). Im Zentrum steht der Rollenverlust der Eltern: Sie müssen Kontrolle sowie Vater- und Mutterinstinkt loslassen, was bei Krankheit des Kindes schwerfällt. Der Ablösungskonflikt sei notwendig; verläuft er destruktiv, droht eine Persönlichkeitsstörung, bei zu wenig Widerstand läuft der Jugendliche ins Leere, bei zu viel wird die Persönlichkeit geschwächt. Tipps an Eltern: klare Haltung mit Regeln als Leitlinien, nicht mehr erziehen, sondern Beziehung pflegen, Verlieren können und Regeln immer wieder neu aushandeln. Das Skript trägt kein Vortragsdatum.",
    },
    Zusammenfassung {
        datei: "Referat_über_ADS_29.08.2007.pdf",
        titel: "",
        text: "Zum ADHS-Referat vor der Schule Neuenhof (29.8.2007): AD(H)S gilt als vererbte funktionelle Normvariante des Gehirns, die durch Umwelteinflüsse stark geprägt wird; da 70 % der Betroffenen im Erwachsenenalter eine zusätzliche Diagnose tragen, sei der Umgang mit den Kindern entscheidend. Empfohlen werden ein ruhiger, emotional entspannter Erziehungsstil, positive Handlungsanweisungen statt Verbote, kurze klare Ansagen, feste Regeln und Rhythmen sowie Beruhigung statt Bestrafung. Der dritte Abschnitt ordnet Erziehungsstile möglichen Folgeerkrankungen zu, etwa Überstimulation und Schuldinduktion der Schizophrenie, rigide Strafen der Delinquenz, unterdrückte Emotionalität der Borderline-Persönlichkeit und Essstörungen.",
    },
    Zusammenfassung {
        datei: "Scheidungsfamilien_Zusammengewürfelte_Familien_29.10.1991.pdf",
        titel: "",
        text: "Cut-off wird in diesem Kursskript von U. Davatz (Familientherapie-Kurs I, 1991) als Lebenserhaltungsreaktion bei Scheidung und in Elternbeziehungen erklärt, meist aus der Beziehung zur eigenen Ursprungsfamilie heraus. Therapeutisch geht es darum, Verpasstes bei den Eltern, und sei es nur gedanklich, nachzuholen. Für Kinder von Geschiedenen und Alleinerziehenden besteht die Gefahr der Fusion mit dem Elternteil, der Parentifizierung und der Rolle des Symptomträgers. Pathologische Cut-offs zwischen Eltern und Kindern treten bei zu enger Bindung, zu langen Dienstleistungen der Eltern oder stark abweichenden Begabungen auf. Ein Cut-off kann dann überlebensnotwendig sein.",
    },
    Zusammenfassung {
        datei: "Schichtung_Klasse_26.10.1987.pdf",
        titel: "",
        text: "Kursunterlage von B. Baumann zu sozialer Ungleichheit und Schichtung. Sie stellt drei Erklärungsansätze für den Ursprung sozialer Ungleichheit vor (Struktur-Funktionalismus, Konflikttheorie, Klassentheorie nach Marx) und beschreibt Schichtung über Prestige und Macht sowie über erwerbbare und zugeschriebene Statusmerkmale. Ein Schichtmodell für Deutschland (Oberschicht bis «sozial Verachtete») und die Möglichkeiten der Aufwärtsmobilität, vor allem über Bildung, werden erläutert. Schliesslich geht es um die Auswirkungen der Schichtzugehörigkeit auf äussere Lebensqualität (Wohnen, Arbeitsplatz, Krankheit, Erziehung) und innere Lebensqualität (Suizid, psychische Störungen) in einem familientherapeutischen Kurs.",
    },
    Zusammenfassung {
        datei: "Schizophrenie_Autoaggression_und_Autismus_27.04.2007.pdf",
        titel: "",
        text: "Vortrag am AWZ über Schizophrenie, Autoaggression und Autismus. Die Referentin, nach eigenen Angaben seit 37 Jahren mit Schizophreniekranken und ihren Familien befasst, erklärt die schizophrene Reaktion als Folge langen emotionalen Stresses, der nicht adäquat ausgedrückt werden kann: Das emotionale Hirn läuft über, der kognitive Teil dekompensiert. Autoaggression entsteht, wenn die Selbstverteidigung eines Jugendlichen im Ablösungskonflikt unterbunden wird und sich nach innen kehrt; Depression und Suizid seien ihr extremster Ausdruck. Autismus wird als passiver Selbstschutz und Rückzug vor Reizüberflutung verstanden, vergleichbar einem Totstellreflex, oft in einem überstimulierenden Umfeld; er tritt bei Kindern als Form von Schizophrenie auf, aber auch bei Erwachsenen als Extremzustand.",
    },
    Zusammenfassung {
        datei: "Sexualverhalten_Sexualstörungen_16.10.1990.pdf",
        titel: "",
        text: "Davatz erklärt Sexualverhalten aus der Verhaltensbiologie: Es dient der Arterhaltung und Paarbindung, ist triebgesteuert, beim Menschen aber lernabhängig; männliches Verhalten liegt nahe bei Jagd und Aggression, weibliches ist rezeptiv und auf Verführung ausgerichtet. Sexualstörungen führt sie auf Unterdrückung und Fehlentwicklung der Sexualität zurück, etwa durch Gruppen, die so ihre Kohäsion erhöhen, oder durch Eltern, die das Sexualverhalten der Kinder kontrollieren. Homosexualität, lesbische Beziehungen, Exhibitionismus, Pädophilie und Inzest werden jeweils mit familiären Konstellationen und Beziehungsmustern in Verbindung gebracht. Es handelt sich um ein Kursskript für Familientherapie II vom 16.10.1990 in Stichworten, dessen Deutungen aus der damaligen Zeit stammen.",
    },
    Zusammenfassung {
        datei: "Sind_Mütter_die_besseren_Beraterinnen_28.10.2026.pdf",
        titel: "",
        text: "Artikel für die Zeitung «Clic» der Mütterberaterinnen zur Frage, ob Mütter die besseren Beraterinnen sind. Abgewogen werden Vorteile (rasche Nähe und Empathie) und Nachteile (zu starkes Leiten durch die eigene Erfahrung, Konkurrenz zwischen Müttern) einer Beraterin, die selbst Mutter ist. Anhand erstgebärender Schimpansenmütter und des Brauchs in Grossfamilien argumentiert Davatz, dass sozialer Kontakt und Lernen am Modell wichtiger seien als Beratung, da junge Mütter in Kleinfamilien oft isoliert leben. Sie hält fest, dass beide Beraterinnen Vor- und Nachteile haben, zuerst eine Beziehung aufbauen sollten und die Väter einbeziehen müssen.",
    },
    Zusammenfassung {
        datei: "Sinn_und_Zweck_der_Mütter-_und_Väterberatung_12.06.2002.pdf",
        titel: "",
        text: "Für die Delegiertenversammlung der Mütter- und Väterberatung Brugg begründet der Vortrag, warum Investitionen in diese Stelle langfristig rentabel sind. Mangelnde elterliche Kompetenz verursache Kosten in Kette: Hospitalisation von Babys, Krankheiten bei Kindern und Eltern, Fremdplatzierungen, Jugendmassnahmen bis hin zu Gefängnis im Erwachsenenalter. Eine Langzeitstudie aus Kanada zeige, dass Unterstützung gefährdeter Familien ab der Geburt ein Vielfaches an späteren Kosten spare («1 Franken Prävention, 100 Franken Reparation»). Die Beraterinnen wirken durch Hausbesuche, Vermittlung im Familiensystem und Zuweisung weiterer Hilfen als Primärprävention.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_in_menschlichen_Systemen_oder_Weitergabe_29.10.1991.pdf",
        titel: "«Soziale Vererbung» in menschlichen Systemen oder Weitergabe des emotionellen Prozesses über mehrere Generationen hinweg",
        text: "Soziale Vererbung bezeichnet in diesem Skript von U. Davatz (Familientherapie-Kurs I, 1991) die Weitergabe von Verhaltensmustern und emotionalen Prozessen über Generationen, im Unterschied zur genetischen Vererbung. Sie wird als Prägungsprozess im emotionalen System beschrieben, der schneller anpassbar, aber schwer umzuprogrammieren ist, und als multifaktorieller, interaktiver Prozess mit Teufelskreisen. Familienthemen, an der Emotionalität erkennbar, werden über Wertvorstellungen und Erziehung weitergegeben. Das Erkennen dieser Themen ist wichtig, sobald das System dysfunktional wird, denn Therapie bedeutet Umprogrammieren der automatischen Muster. Die letzte Seite zeigt ein Muster-Genogramm mit den üblichen Symbolen als Vorlage.",
    },
    Zusammenfassung {
        datei: "Symbiose_zwischen_Mutter_und_Kind_sowie_symbiotische_22.09.2004.pdf",
        titel: "Symbiose zwischen Mutter und Kind sowie symbiotische Beziehung in der Paarbeziehung",
        text: "Fortbildungsvortrag am Psychiatrischen Zentrum Biel über Symbiose zwischen Mutter und Kind und in der Paarbeziehung. Ausgehend von Mahler, Klein und Bowen (Fusion) wird die natürliche Symbiose, in der der Energiefluss auf die nächste Generation gerichtet ist, von der pathologischen unterschieden, in der das Kind zum Versorger der Eltern wird und sich nicht ablösen darf. Bei Paaren würden unbefriedigte Kindheitsbedürfnisse über die Partnerschaft nachgeholt; fehlende Respektierung individueller Grenzen führe zu Kränkung, Aggression und Beziehungsgewalt. Therapeutisch gilt: die Bedürfnisse der Eltern auf anderem Weg als über das Kind erfüllen, bei Paaren beide in ihrer Autonomie stärken und keine Verhandlungstaktik oder Mediation anwenden.",
    },
    Zusammenfassung {
        datei: "Text_für_Werbung_Familientherapie_28.10.2026.pdf",
        titel: "",
        text: "Kurzer Werbetext zur Familientherapie: Sie wird auch Systemtherapie genannt und betrachtet Symptome im Kontext des Familiensystems, wo sie einen kommunikativen Wert haben. Die Intervention setzt zuerst beim Verstehen des Symptoms an und richtet sich vor allem an die «Gesunden», meist Eltern und gesunde Geschwister, weil der Patient das schwächste Glied sei. Das Setting ist flexibel, Familientherapie eigne sich auch bei körperlichen Krankheiten mit psychosomatischen Anteilen und lasse sich auf Arbeitssysteme in Supervision und Consulting übertragen.",
    },
    Zusammenfassung {
        datei: "Therapeutische_Jurisprudenz_27.11.2002.pdf",
        titel: "",
        text: "Der Vortrag für das Psychiatrie-Zentrum Biel ordnet die therapeutische Jurisprudenz in das menschliche Sozialverhalten ein: genetische «Hardware» (Kampf, Dominanz, Unterwerfung, Fluchtverhalten) und kulturelle Prägung, dazu das Gesetz als «zweite soziale Software». Psychiatrie habe einen sozialen Auftrag der Normalisierung, Individualtherapie setze am Einzelnen, Familientherapie an der ganzen Gruppe an. Therapeutische Jurisprudenz soll im Gegensatz zur strafenden Lernen und Entwicklung in einem sicheren juristischen Rahmen ermöglichen; das Gesetz sei oft zu grob und aus vorpsychologischer Zeit. Gefordert wird die Zusammenarbeit von Juristen, Psychiatern, Psychologen und Institutionen, wobei jede Disziplin ihr Paradigma behält; die psychiatrische Gutachtertätigkeit wird kritisch beurteilt.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_schwierigen_Familiensituationen_23.04.2002.pdf",
        titel: "",
        text: "Für Spitex-Fachpersonen, die im Territorium des Patienten pflegen, ist die Beziehung zum Patienten das wichtigste therapeutische Instrument. Der Vortrag behandelt vier Erschwernisse: Non-Compliance des Patienten, Einmischung der Angehörigen, Beziehungskonflikte zwischen Patient und Angehörigen sowie die eigene Überforderung. Statt Machtkampf oder Rückzug wird empfohlen, die Lebensgeschichte zu verstehen, Angehörige als Ressource einzubeziehen, nicht Richter zu spielen und die eigene Hilflosigkeit einzugestehen. Als Pflegende soll man sich nicht ins Familiensystem einbauen lassen, sondern eine gewisse objektive Position bewahren; das wird an Praxisbeispielen geübt.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026.pdf",
        titel: "",
        text: "Artikel für die Spitex Zeitung zur Weiterbildung «Umgang mit schwierigen Familiensituationen in der Spitex» (23.4.2002). Familien seien durch eine schwere Krankheit oft überfordert, sodass Spitex-Fachpersonen in schwierige Beziehungsgeflechte geraten. Als Regeln nennt Davatz, nicht nur den designierten Patienten, sondern die ganze Familie einzubeziehen, emotional neutral zu bleiben und sich die Lebensgeschichten erzählen zu lassen. Die eigene Person sei das wichtigste Instrument, man werde zum «Familienmitglied auf Zeit»; Intervision und Humor helfen dabei. Am Schluss stehen Literaturhinweise zu Bowen, McGoldrick, Tomann und Weiss.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_schwierigen_Familiensituationen_in_der_Spitex_28.10.2026_2.pdf",
        titel: "",
        text: "Zweite Fassung des Artikels für die Spitex Zeitung über den Umgang mit schwierigen Familiensituationen, grösser gesetzt und mit Autorenzeile von U. Davatz, ohne Literaturhinweise. Inhaltlich entspricht sie der ersten: Familien sind bei schwerer Krankheit überfordert, Spitex-Fachpersonen sollen die ganze Familie statt nur den designierten Patienten einbeziehen, emotional neutral bleiben und sich Lebensgeschichten erzählen lassen. Die eigene Person ist das wichtigste Instrument; Intervision, Humor und systemische Konzepte aus einer Weiterbildung helfen, nicht in Machtkämpfe zu geraten.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_Ängsten_06.06.2002.pdf",
        titel: "",
        text: "Angst gilt als natürliches Alarmsignal, das sich verselbständigen und sozial ansteckend eskalieren kann. Der Vortrag für eine Spitex-Weiterbildung beschreibt die drei Verhaltensmuster Flucht, Kampf/Imponieren und Totstellreflex/Unterwerfung sowie die körperlichen Reaktionen (Kreislauf, Muskeltonus, Hormon- und Immunsystem, Schlaflosigkeit). Ängstliche Pflegende und ängstliche Patienten können sich gegenseitig zu einer «Teufelsspirale» aufschaukeln; verkannte Angst führt zu Machtkämpfen. Empfohlen werden, die Angst des Patienten wahrzunehmen und beruhigend zu reagieren, die eigene Angst einzugestehen und Hilfe zu holen.",
    },
    Zusammenfassung {
        datei: "Unser_Teenager_wirkt_komisch,_Pubertät_oder_eine_beginnende_21.11.2002.pdf",
        titel: "Unser Teenager wirkt «komisch», Pubertät oder eine beginnende Psychose?",
        text: "Der Vortrag für den VASK Aargau fragt, ob ein auffälliger Teenager sich in der Pubertät oder in einer beginnenden Psychose befindet. Er listet die typischen Problemfelder der Adoleszenz auf (Selbstreflexion, Geschlechterrolle, Berufsfindung, Ablösung, Autonomie, Versagensangst, Peergruppe, Rückzug, Gefühlsschwankungen) und häufige Fehlreaktionen der Eltern wie Überfokussierung, Kontrolle und gekränkte Gegenreaktion. Die Schizophrenie entstehe bei lang anhaltendem emotionalem Stress, wenn der emotionale Hirnteil den kognitiven überflutet, und sei beeinflussbar und behandelbar. Eltern sollen ihre eigene Angst und den Druck zurücknehmen, sich wieder der Partnerbeziehung zuwenden, den Vater einbeziehen und früh fachliche Hilfe holen.",
    },
    Zusammenfassung {
        datei: "Verstecktes_Leiden_Suchtkranke_und_Pflegende_in_der_Spitex_18.04.2002.pdf",
        titel: "Verstecktes Leiden: Suchtkranke und Pflegende in der Spitex",
        text: "Sucht wird als fehlgelaufene Beziehung verstanden: Der Suchtkranke tauscht die Abhängigkeit von einer Bezugsperson gegen jene von einer Substanz. Der Vortrag für eine Spitex-Weiterbildung zeigt, wie moralische Verurteilung und Fremdkontrolle durch Fachpersonen Lügen, Verleugnen und Flucht auslösen. Empfohlen werden eine wertneutrale Haltung, keine Verantwortung für die Abstinenz, das Defokussieren der Sucht als blosses Symptom und eine gute emotionale Abgrenzung. Die Kontrolle soll erst bei akutem Kontrollverlust übernommen werden, und zwar ohne Strafcharakter.",
    },
    Zusammenfassung {
        datei: "Verwahrlosung_17.08.2006.pdf",
        titel: "",
        text: "Kurzes Vortragsskript für die Spitex Fricktal über Verwahrlosung mit Handnotizen am Rand. Körper- und Nestpflege sind Grundbedürfnisse; Verwahrlosung kann Abwehr gegen Übergriffe sein (Teenager, aber auch kranke Erwachsene), Ausdruck eines schlechten Selbstwertgefühls bei Depression, Folge von Demenz oder Teil des Messie-Syndroms. Hilfe kann dabei selbst als Übergriff erlebt werden oder das Selbstwertgefühl weiter senken; in solchen Fällen solle eher die Persönlichkeit des Patienten gestärkt werden, bei Demenz sei sorgfältiges, schreckfreies Vorgehen nötig. Die Wegnahme von Dingen bei Messies werde als Angriff auf die Persönlichkeit erlebt. Der Umgang verlange Sensibilität und Taktgefühl, weil man in den Persönlichkeits- und Intimbereich eingreife.",
    },
    Zusammenfassung {
        datei: "Vom_Paar_zur_Familie_Fruchtbarkeit_und_Unfruchtbarkeit_22.10.2002.pdf",
        titel: "Vom Paar zur Familie – Fruchtbarkeit und Unfruchtbarkeit",
        text: "Der Vortrag in einer Praxis behandelt die biologische Funktion der Paarbeziehung (Sicherung der Brutpflege) und das «Bonding» ohne Kinder, in dem oft unerfüllte Bedürfnisse aus der Kindheit am Partner nachgeholt werden. Kommt ein Kind, werden die seelischen Ressourcen knapp; Mann und Frau fühlen sich vernachlässigt, und das Kind wird leicht zur Bedürfnisbefriedigung der Eltern, der Energiefluss kehrt sich um. Zur Fruchtbarkeit vertritt die Vortragende die These, dass bei fehlender seelischer «Nestwärme» die Zeugungsfähigkeit ausbleiben könne, vergleichbar mit Magersucht. Sie plädiert dafür, zuerst das seelische Nest zu bauen, bevor man die Fruchtbarkeit künstlich herbeiführt.",
    },
    Zusammenfassung {
        datei: "Vom_Umgang_mit_Cut-off_(Beziehungsabbruch)_und_dessen_29.10.1992.pdf",
        titel: "Vom Umgang mit «Cut-off» (Beziehungsabbruch) und dessen Überwindung",
        text: "Anonyme Kursarbeit im Familientheorie-Kurs II (1992): Der Verfasser setzt sich mit dem Cut-off (Beziehungsabbruch) in der eigenen Familie auseinander, die er über mehrere Generationen als Tradition von Beziehungsabbrüchen erlebt. Als konkretes Vorgehen nimmt er sich vor, in heiklen Momenten im Gespräch zu bleiben und den anderen erzählen zu lassen. Theoretisch stützt er sich auf Bowen und auf Nagys «Familien-Konti», auf denen offen oder verdeckt über Soll und Haben Buch geführt wird. Weitere Themen sind unerledigte Verletzungen, die Rolle der Geschlechterbilder in der Familie und der Unterschied zwischen Kontakt- und Beziehungsabbruch. Das Exemplar trägt Handnotizen.",
    },
    Zusammenfassung {
        datei: "Vorwort_zu_dieser_Arbeit_von_Stephan_Meyer_08.02.2002.pdf",
        titel: "",
        text: "Vorwort von U. Davatz (8.2.2002) zur Arbeit eines selbst Betroffenen über das frühkindliche POS, das heute als ADD bezeichnet wird. Sie ordnet den umstrittenen Begriff als altbekanntes Krankheitsbild ein, das sich mit bildgebenden Verfahren darstellen lasse, und plädiert dafür, die neurologische Störung ernst zu nehmen. Wer sie durch harte, bestrafende Erziehung «heilen» wolle, füge den Betroffenen zusätzlich ein psychisches Trauma zu. Die Arbeit sei für Lehrer, Eltern, Sozialämter, Hausärzte und Arbeitgeber von Lehrlingen lesenswert.",
    },
    Zusammenfassung {
        datei: "Warum_so_viele_Ess-Störungen_in_der_heutigen_08.03.2003.pdf",
        titel: "",
        text: "Vortrag über die Frage, warum Essstörungen in der Überflussgesellschaft so häufig sind. Davatz versteht sie als zeitgebundene «Modeerscheinung» und leitet sie aus der sozialen und machtbezogenen Bedeutung des Essens ab: Die Magersucht sei im Ablösungskonflikt der Pubertät ein Machtmittel der Jugendlichen gegen die Eltern, die Bulimie unterdrücke Aggression, die in der Familie keinen Platz hat, die Fettsucht ersetze verbalen Austausch und soziale Belohnung. Therapeutisch empfiehlt sie, den Machtkampf aus dem Bereich Essen herauszunehmen, das Essen zu enttabuisieren statt zu kontrollieren und je nach Störung Raum für Auseinandersetzung oder Gespräche zu schaffen. Ein Symptom müsse sich von der Umwelt abheben, um Aufmerksamkeit zu erregen – im Überfluss also als Hungern, Erbrechen oder Überessen.",
    },
    Zusammenfassung {
        datei: "Was_bedeutet_mir_Lebensqualität_Was_ist_sie_mir_wert_04.06.2002.pdf",
        titel: "Was bedeutet mir Lebensqualität? Was ist sie mir wert?",
        text: "Der Vortrag für den Katholischen Frauenbund Untersiggenthal stellt Lebensqualität dem Qualitätsmanagement gegenüber: Sie lasse sich weder produzieren noch quantitativ messen oder kaufen, sondern nur leben. Als Zerstörer nennt er Stress, dauernde Unzufriedenheit, unglückliche Beziehungen, überhöhte Ansprüche an sich selbst, Neid, zu viel Wettkampf, Unbehagen in der eigenen Rolle und fehlende Zeit. Fehlende Lebensqualität führe zu Krankheit, Aggression oder Ersatz durch Drogen, Medikamente und Konsum, weshalb gute Lebensqualität die beste Suchtprävention sei. Empfohlen werden Zeit zum Geniessen, Bescheidenheit, Mut zur Veränderung und Pflege der Beziehungen.",
    },
    Zusammenfassung {
        datei: "Was_bewirkt_das_Auftreten_einer_lebensbedrohlichen_Krankheit_27.02.2000.pdf",
        titel: "Was bewirkt das Auftreten einer lebensbedrohlichen Krankheit oder die Geburt eines behinderten Kindes in der Familie – mit besonderer Berücksichtigung der Geschwister",
        text: "Jahresarbeit einer Ärztin im Kurs II (1999) darüber, was das Auftreten einer lebensbedrohlichen Krankheit oder die Geburt eines behinderten Kindes in der Familie bewirkt, mit besonderer Berücksichtigung der Geschwister. Aus neun Fallbeispielen aus ihrer Praxis, die im Anhang beschrieben sind (Namen und Daten sind im Exemplar durch erfundene ersetzt), schliesst sie, dass Akzeptanz, Trauerarbeit und die Vermeidung einer Überfokussierung wichtig sind. Zu viele Therapien können dem Kind schaden, die Eltern, besonders die Mütter, brauchen mehr Begleitung, und die Geschwister müssen gestärkt werden. Therapeuten sollten systemisch denken und mehr mit der Familie arbeiten, um «Zweiterkrankungen» vorzubeugen. Im Anhang stehen auch die Anmerkungen aus der Kursgruppe und von U. Davatz.",
    },
    Zusammenfassung {
        datei: "Was_machen_wir_mit_den_ewig_Unverstandenen,_Abgewiesenen_16.10.2002.pdf",
        titel: "Was machen wir mit den ewig Unverstandenen, Abgewiesenen, Störenfrieden und Querulanten, die später auch gefährlich werden können?",
        text: "Einführende Gedanken zu einem Workshop an der Tagung «Therapeutische Jurisprudenz» über Menschen, die sich ständig unverstanden und abgewiesen fühlen und später gefährlich werden können. Gewalttaten bauen sich laut Vortrag über Monate oder Jahre auf: auf frühen narzisstischen Kränkungen, gehäuften Lebenskränkungen (Jobverlust, Scheidung) und einem impulsiven Temperament, wobei POS/ADD-Betroffene unter Delinquenten häufiger seien. Kleine, unbedachte Kränkungen durch offizielle Stellen können dann die Aggression auslösen. Empfohlen werden höfliches Verhalten, keine zusätzliche Machtausübung, Ohnmacht und Empathie zeigen, Zeit gewinnen, sich für die Geschichte der Person interessieren, Bezugspersonen einbeziehen und mit anderen Stellen zusammenarbeiten.",
    },
    Zusammenfassung {
        datei: "Was_und_wieviel_kann_die_Mütter-_und_Väterberatung_zur_28.10.2026.pdf",
        titel: "Was und wieviel kann die Mütter- und Väterberatung zur Prävention beitragen?",
        text: "Artikel für die Zeitung «Clic» darüber, was die Mütter- und Väterberatung zur Prävention beitragen kann. Davatz beschreibt Störfaktoren der Mutter-Kind-Beziehung: soziale Isolation, chronischer Partnerkonflikt, Depression und postpartale Psychose, umgekehrtes Sorgeverhalten des Kindes (Parentifizierung), Familienbelastungen und traumatische Geburtserlebnisse. Daraus leitet sie Aufgaben der Beraterin ab: behutsamer Beziehungsaufbau, Zuhören, Vermittlung zwischen den Eltern, Weg in eine Therapie und Einbezug der Grosseltern oder einer Ersatzmutter. Sie argumentiert, dass jeder in diese Arbeit investierte Franken sich über gesparte Gesundheits- und Sozialkosten vielfach auszahle, und verweist auf eine kanadische Langzeitstudie zur Delinquenz.",
    },
    Zusammenfassung {
        datei: "Welche_Chancen_bietet_die_familienergänzende_Betreuung_von_06.11.2003.pdf",
        titel: "Welche Chancen bietet die familienergänzende Betreuung von Kindern der Öffentlichkeit und der Wirtschaft?",
        text: "Stichwortartiger Vortragstext für die FDP Villmergen zu den Chancen der familienergänzenden Kinderbetreuung für Öffentlichkeit und Wirtschaft. Davatz geht von sinkenden Geburtenraten aus und argumentiert, dass Kinder für gut ausgebildete, beidseitig berufstätige Paare nur dann attraktiv bleiben, wenn es zusätzliche Betreuung gibt, etwa Krippen, Hütedienste und Mütter- und Väterberatung. Solche Angebote seien eine präventive Langzeitinvestition, die Fehlentwicklungen und Reparaturkosten spart. Genannt werden ausserdem flexiblere Arbeitswelt, Abzug der Betreuungskosten bei den Steuern, Kritik an Überprofessionalisierung und Normierung sowie die Schlussfolgerung, gute Sozialpolitik sei auch gute Wirtschaftspolitik.",
    },
    Zusammenfassung {
        datei: "Weshalb_habe_ich_einen_18_Jahre_älteren_Mann_geheiratet_28.10.1993.pdf",
        titel: "Weshalb habe ich einen 18 Jahre älteren Mann geheiratet?",
        text: "Persönliche Kursarbeit einer Teilnehmerin der Familientherapie-Weiterbildung 1993 (Leitung U. Davatz) zur Frage, weshalb sie einen deutlich älteren Mann geheiratet hat. Mit dem familienorientierten Ansatz nach Bowen geht sie dem Zusammenhang mit der Vaterthematik nach, auch vor dem Hintergrund des Todes ihres Vaters. Sie beschreibt Spannungen in der Partnerschaft durch ihre berufliche Entwicklung, die vorgeschlagenen Schritte (Interesse an den Arbeiten des Mannes, gemeinsames Hobby, Gespräche mit Mutter und Schwester) und deren Wirkung. Ihr Fazit: Die Familientherapie habe unbewusste Abläufe sichtbar gemacht und das Verständnis für die unterschiedlichen Lebensphasen der Partner gefördert.",
    },
    Zusammenfassung {
        datei: "Wie_erkenne_ich_meine_Achillesferse_Was_sind_meine_typischen_31.10.2002.pdf",
        titel: "Wie erkenne ich meine Achillesferse? Was sind meine typischen Reaktionsmuster bei Stress? Auf welche Konstellationen bin ich speziell empfindlich?",
        text: "Für eine Beratung/Weiterbildung zu «Engagiert pflegen ohne Burnout» geht es darum, die eigene Achillesferse zu erkennen. Die Schwachpunkte hängen mit der Prägung in der Herkunftsfamilie zusammen; Einsicht erleichtert, verändert aber erst durch Übung, am besten im Familienmuster. Typische Stressmuster werden körperlich gedeutet (Nacken, Kopf, Rücken, Bauch, Tennisellbogen) und als Kontroll-, Kampf-, Flucht- oder Totstellverhalten beschrieben. Als Faustregeln gegen Überforderung gelten: in der Begegnung statt im helferischen Aktivismus pflegen, Erfolge geniessen, sich selbst loben, offen kommunizieren, ein eigenes «Credo» formulieren, Grenzen halten und mit Fehlern menschlich umgehen.",
    },
    Zusammenfassung {
        datei: "Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_06.03.2003.pdf",
        titel: "Wie wirkt sich meine Vergangenheit auf unser Kind aus?",
        text: "Für die Elternvereine und die Mütter- und Väterberatung im Bezirk Muri fragt der Vortrag, wie sich die eigene Kindheit auf die Erziehung der Kinder auswirkt. Er unterscheidet drei Erziehungsebenen: Konditionierung (auch durch Liebesentzug, also Angst), Modelllernen und verbale Kommunikation als Vermittlung von Werten. Dazu kommen emotionale Kommunikation (positive, negative, appellative Emotionen), das «double bind» bei widersprüchlichen Signalen und der Unterschied zwischen direkter und indirekter Kommunikation. Die Zuhörer sollen den Kommunikationsstil ihrer Herkunftsfamilie und ihren eigenen reflektieren; der Vortrag schliesst mit dem Satz, der Apfel falle nicht weit vom Stamm, man könne ihn aber weiter rollen.",
    },
    Zusammenfassung {
        datei: "Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_20.10.2003.pdf",
        titel: "Wie wirkt sich meine Vergangenheit auf unser Kind aus?",
        text: "Vortrag von Dr. med. U. Davatz für den Kreis junger Mütter der Mütter- und Väterberatung Bezirk Frick. Sie unterscheidet drei Erziehungsebenen – Konditionierung (etwa durch Liebesentzug und Angst), Modelllernen und verbale Kommunikation als Vermittlung von Wertvorstellungen – und beschreibt, wie sich verbale und emotionale Kommunikation mischen («double bind» bei Widerspruch). Weiter geht es um direkte und indirekte Kommunikation. Die Zuhörerinnen sollen den Kommunikationsstil der eigenen Herkunftsfamilie befragen und erkennen, dass Gelerntes in der eigenen Erziehung wiederkehrt; Schlussbild: Der Apfel fällt nicht weit vom Stamm, man kann ihn aber bewusst etwas weiter rollen.",
    },
    Zusammenfassung {
        datei: "Wie_wirkt_sich_meine_Vergangenheit_auf_unser_Kind_aus_20.10.2003_2.pdf",
        titel: "Wie wirkt sich meine Vergangenheit auf unser Kind aus?",
        text: "Zweite Fassung des Vortrags von Dr. med. U. Davatz für den Kreis junger Mütter der Mütter- und Väterberatung Bezirk Frick, mit dem Titel direkt unter dem Briefkopf und in Aufzählungen gegliedert. Erziehung wird auf drei Ebenen beschrieben: Konditionierung durch Liebesentzug und Angst, Modelllernen sowie verbale Kommunikation als Weitergabe familiärer Werte. Ergänzt durch die Unterscheidung positiver, negativer und appellativer Emotionen, «double bind» und direkter gegenüber indirekter Kommunikation. Eine Fragenliste zum Kommunikationsstil der eigenen Eltern lädt zur Selbstprüfung ein; die Schlussbemerkung ergänzt, man könne den Apfel weiter rollen lassen, wenn man die Voraussetzungen dafür schaffe.",
    },
    Zusammenfassung {
        datei: "Woher_die_vielen_Ess-Störung_in_der_heutigen_Zeit_und_wie_02.09.2006.pdf",
        titel: "Woher die vielen Ess-Störungen in der heutigen Zeit und wie gehen wir damit um?",
        text: "Überarbeitete Fassung des Vortrags vom 8.3.2003 über Essverhalten und Psyche. Essverhalten ist bei sozialen Tieren an das Sozialverhalten gekoppelt; gestörtes Essen wird als fehlgelaufenes Sozialverhalten und Befriedigungsverhalten gedeutet, Essen hat auch eine Machtkomponente. Die Magersucht tritt meist in der Pubertät auf: Wo der Ablösungskonflikt unterdrückt wird, werde die Essensverweigerung zum Machtmittel gegen die Eltern, die um das Überleben kämpfen und dabei gegen die Autonomie des Kindes. Bulimie wird als unterdrückte Aggression gedeutet, Adipositas als Ersatz für verbalen Austausch und soziale Belohnung. Therapeutisch soll der Machtkampf aus dem Essen herausgenommen und die Kontrolle der Patientin überlassen werden; die Zunahme der Essstörungen führt die Referentin auf ein Übergewicht von Wettbewerb gegenüber Kooperation zurück.",
    },
    Zusammenfassung {
        datei: "Woher_die_vielen_Ess-Störung_in_der_heutigen_Zeit_und_wie_02.09.2006_2.pdf",
        titel: "Woher die vielen Ess-Störungen in der heutigen Zeit und wie gehen wir damit um?",
        text: "Entwurf mit Handkorrekturen zum Vortrag vom 2.9.2006 über Essverhalten und Psyche; der Titel lautet noch «Warum so viele …», die Seiten 3 bis 5 sind stark überarbeitet. Inhaltlich entspricht er der Endfassung: Essverhalten als Teil des Sozialverhaltens, Essen als Machtmittel, Magersucht als Machtkampf im Ablösungskonflikt der Pubertät, Bulimie als unterdrückte Aggression, Adipositas als Ersatz für sozialen Austausch. Therapeutisch soll das Essen tabuisiert, die Kontrolle der Patientin überlassen und bei Bulimie Raum für Auseinandersetzungen ausserhalb des Essens geschaffen werden. In der Schlussbemerkung werden Essstörungen mit zu wenig seelischer Nahrung und unausgewogenem Sozialverhalten in der Überflussgesellschaft erklärt; die handschriftlichen Änderungen betreffen vor allem diesen Teil.",
    },
    Zusammenfassung {
        datei: "Zusammenarbeit_unter_Sozialtherapeutischen_Systemen_bei_30.04.2002.pdf",
        titel: "Zusammenarbeit unter sozialtherapeutischen Systemen bei gefährdeter Mutter-Kind-Beziehung",
        text: "Bei gefährdeter Mutter-Kind-Beziehung streiten Helfersysteme oft um Macht über das Kind, obwohl der Brutpflegeinstinkt eigentlich dessen Wohl leiten sollte. Der Vortrag vor dem Psychiatriestützpunkt Biel skizziert ein systemisches Vorgehen bei fraglicher Kindsplatzierung: die Mutter zuerst stützen, professionelle Helfer nicht mit ihr konkurrieren lassen und, falls nötig, die Fremdplatzierung sorgfältig vorbereiten. Eine Person, am besten jene mit der besten Beziehung zur Mutter, soll die Führung übernehmen; die Hierarchie der Institutionen darf nicht entscheiden. Fehlt eine klare Führung, droht dem Kind laut Vortrag die soziale Verwahrlosung.",
    },
    Zusammenfassung {
        datei: "Überbrückung_vom_Cut-off_in_meiner_Herkunftsfamilie_12.1991.pdf",
        titel: "Überbrückung vom «Cut-off» in meiner Herkunftsfamilie",
        text: "Kursarbeit eines Teilnehmers im Familientheorie-Kurs II (1991, Leitung U. Davatz) über die Überbrückung eines Cut-offs in der eigenen Herkunftsfamilie, insbesondere zum Vater. Der Verfasser definiert Cut-off als Beziehungsabbruch mit Schutzfunktion, aber ohne Möglichkeit, Konfliktlösung zu lernen, und beschreibt autoritäre Familiensysteme mit Tabus, Mythen und Entweder-oder-Mustern. Aus dem Kurs zieht er den Schluss, dass man nur sich selbst verändern kann. Durch ein klärendes Gespräch, positive Konnotation und Gleichwertigkeit gegenüber der Autorität liess sich der Cut-off überbrücken, was neue Anpassungen im ganzen System erlaubte.",
    },
    Zusammenfassung {
        datei: "Übersicht_der_Kursarbeiten_Familientherapie_29.10.2026.pdf",
        titel: "",
        text: "Tabellarische Übersicht der Kursarbeiten aus den Familientherapiekursen, geordnet nach Themen mit Jahr (1987 bis 2001). Rubriken sind unter anderem Cut-off, Familienbeziehungen und Ersatzfamilien, Fusion und Differenzierung, Generationen (Dreiecksbeziehung nach Bowen), Schuldgefühle, Soziale Vererbung, Suchtverhalten, Schizophrenie, Kommunikation unter Spannung sowie Sonderthemen wie ADS bei Erwachsenen, Depression als Verliererkrankheit und Inzest. Am Schluss stehen die Teilnehmerlisten der Kurse I bis III.",
    },
];
