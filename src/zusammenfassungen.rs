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
        datei: "ADHS_ist_keine_Krankheit_09.11.2026.pdf",
        titel: "",
        text: "Aufsatz von U. Davatz (Kompetenzzentrum für ADHS und Schizophrenie, ohne Datum, nach Juli 2018), ausgehend von Kohortenstudien, nach denen Schizophrenie, Depression, Autismus, bipolare Störung, Essstörungen und ADHS dieselben veränderten Gen-Loci teilen. Sie deutet ADS und ADHS nicht als Krankheit, sondern als genetisch vererbte Neurotypen, einen introvertierten und einen extravertierten Persönlichkeitstyp, der dickköpfig, impulsiv und sehr verletzlich ist und im Beziehungsumfeld schnell Konflikte auslöst. Das formbare Gehirn passt sich über Erfahrung an; scheitern erlernte Muster und die Stressreflexe Kampf, Flucht und Totstellen, entstehen unter dem Anpassungsdruck psychische und körperliche Krankheiten, weshalb 75 bis 80 Prozent der Erwachsenen mit ADS/ADHS psychiatrische Hilfe brauchen. Die Ursache der psychiatrischen Erkrankungen sieht sie im erzieherischen Umfeld; Diagnosen nach Symptomen sagen nichts über ihre Entstehung, darum fordert sie eine sorgfältige Familien- und Lebensgeschichte. Kompetente Beratung von Familien und Schulen mit ADS- und ADHS-Kindern wäre die erste echte Primärprävention der Psychiatrie.",
    },
    Zusammenfassung {
        datei: "ADHS_ist_keine_Krankheit_09.11.2026_2.pdf",
        titel: "",
        text: "Zweite, anders umbrochene Fassung des Aufsatzes von U. Davatz; der Text ist derselbe. ADS und ADHS werden als genetisch bestimmte Neurotypen verstanden, nicht als Krankheit: Erst wenn das erzieherische Umfeld mit ihrer Sensibilität und Impulsivität nicht zurechtkommt, entstehen über Konfliktspiralen psychische Krankheiten. Die rein neuropharmakologische Behandlung verlangt die Anpassung allein vom Patienten und lässt das Beziehungsumfeld als Veränderungsfaktor ungenutzt. Kompetente Beratung von Familien und Schulen wäre eine wirksame Primärprävention.",
    },
    Zusammenfassung {
        datei: "ADS_und_Borderlinepersönlichkeitsstörung,_ihre_Auswirkung_im_30.08.2004.pdf",
        titel: "ADS und Borderlinepersönlichkeitsstörung, ihre Auswirkung im Familiensystem",
        text: "Skript von U. Davatz für den Familienkurs I vom 30.8.2004. Kinder mit ADS, ADHS oder POS fallen durch Aufmerksamkeitsstörung, Lernstörungen, Hypersensibilität und mangelnde Impulskontrolle auf und werden deshalb leicht als ungehorsam, dumm oder böse bestraft und zum Sündenbock der Familie. Welche Folgekrankheit entsteht, hängt vom Erziehungsstil ab: viel redender, Angst machender Druck führe zur Schizophrenie, rigides Strafen zu Delinquenz, ängstliche Kontrolle zu Phobien, Depressionen, Psychosomatik und eventuell zur Borderline-Störung. Borderline-Frauen mit starker Erregbarkeit, die ihre Impulse unterdrücken mussten, wenden sie gegen sich selbst, etwa durch Ritzen oder Bulimie, und stossen andere verletzend ab, worauf das Umfeld mit Erziehungsversuchen und Liebesentzug reagiert. Therapeutisch soll man sie nie erziehen wollen, sondern authentisch bleiben, sich ohne emotionale Distanz abgrenzen und sie wie ewig Pubertierende behandeln.",
    },
    Zusammenfassung {
        datei: "Ablösung_erwachsener_Kinder_von_ihren_betagten_Eltern_17.12.2012.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 17.12.2012. Viele Erwachsene sind nicht vollständig von ihren Eltern abgelöst, und wenn diese betagt und pflegebedürftig werden, tritt die emotionale Abhängigkeit verstärkt hervor. Im Alter werden Verhaltensweisen zur Karikatur: Eltern, die mit Erpressung erzogen haben, setzen ihre Abhängigkeit, ihr Leiden, Schuldgefühle, besonderes Lob oder Vorwürfe als Druckmittel ein. Die erwachsenen Kinder fürchten, sich nach dem Tod der Eltern Vorwürfe zu machen, erwarten noch immer Verständnis, haben nach jeder Gegenwehr ein schlechtes Gewissen und vernachlässigen sich selbst. Die Ablösung lässt sich nachholen, indem man kein Verständnis mehr erwartet, Vater und Mutter einzeln gegenüber einen klaren Standpunkt bezieht, ohne sie überzeugen zu wollen, und zuerst eine Situation sachlich schildert, bevor man das eigene Gefühl einbringt.",
    },
    Zusammenfassung {
        datei: "Ablösung_von_der_Autorität_28.10.1991.pdf",
        titel: "",
        text: "Die Funktion der männlichen Autorität in der patriarchalen Familie steht im Zentrum dieses Kursskripts von U. Davatz (Familientheorie Kurs II/91): Sie ordnet und kontrolliert nach innen und schützt die Familie nach aussen. Setzt sich ein adoleszentes Kind mit dieser Autorität auseinander, bleiben ihm vier Möglichkeiten: sich fügen, sich durch Rückzug oder Weglaufen entziehen (Cut-off), den Konflikt austragen und dabei in Kontakt bleiben (Differenzierung) oder die Autorität zerstören. Die fruchtbare Auseinandersetzung führt nach Bowen zur Entwicklung einer eigenen Autorität mit eigenen Wertvorstellungen. Bei zu starker oder fehlender Autoritätsfigur, etwa im antiautoritären Erziehungsmuster, fehlt dem Kind ein Gegenüber für diese Auseinandersetzung.",
    },
    Zusammenfassung {
        datei: "Ablösungskonflikt_von_betagten_Eltern_mit_Starrköpfigkeit_06.09.1999.pdf",
        titel: "Ablösungskonflikt von betagten Eltern mit Starrköpfigkeit oder Loyalitätskonflikt zwischen den Generationen",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 1999 (6.9.1999) über die Ablösung von alten Eltern, die nach Bowen ein lebenslanger, nie abgeschlossener Differenzierungsprozess ist. Zuerst soll man das Wertsystem der Eltern aus ihrem Kontext verstehen und respektieren, dann die eigenen Werte erarbeiten, die Unterschiede als Spannung aushalten und sich den Eltern über Besuche, Telefonate oder Briefe bewusst aussetzen. Typische Fallen sind gegenseitige Erziehungsversuche, endlose Streitereien und wechselseitig ausgelöste Schuldgefühle, bis hin zu Krankheit oder Todesdrohungen der Eltern, die einen in der Differenzierung innehalten lassen. Unbewusste Loyalität zu den Eltern kann an der Wurzel psychosomatischer und psychischer Erkrankungen stehen, weshalb eine gewisse Dysloyalität zur Selbstdifferenzierung gehört. Therapeutisch gilt es, Schuldgefühle auszuhalten, langsam, aber stetig vorzugehen und sich von einem Coach beim Objektivieren helfen zu lassen.",
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
        datei: "Ambivalenz_und_Pattsituation_15.10.2018.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I 2018 vom 15.10.2018. Ambivalenz, einer der vier Grundbegriffe Bleulers zur Schizophrenie, erklärt sie neuropsychologisch: Unter Stress öffnet das Gehirn viele Handlungsmöglichkeiten, entschieden wird aber vom Gefühl, «vom Herzen», und dazu braucht es Verantwortung. Wer von den Eltern nicht abgelöst ist, pendelt zwischen dem, was erwartet wird, und dem, was er selbst will, später auch gegenüber der öffentlichen Meinung, und gerät ohne eigene, subjektive Moral in eine Pattsituation. Menschen mit ADS und ADHS nehmen sich aus Impulsivität oft zu wenig Zeit, ihr Herz zu befragen, und haben als Kinder selten selbst entscheiden dürfen. Therapeutisch sollen sie lernen, auf sich zu hören und für ihren Entscheid einzustehen, begleitet von einer geduldigen Therapie, die nicht drängt, auch nicht auf Druck von Krankenkassen, KESB oder IV.",
    },
    Zusammenfassung {
        datei: "Angst,_Panikattacken_und_Zwangsverhalten_03.09.2018.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I 2018 vom 3.9.2018. Angst ist ein natürliches Warnzeichen; in der Psychiatrie geht es aber um vorgestellte Gefahren, die oft aus einem angstmachenden Erziehungsstil stammen: Die verinnerlichten elterlichen Verbote lösen Angst vor Strafe und Dysloyalität aus, sobald man seinen eigenen Weg gehen will, und als Angst vor der Angst weitet sie sich über Ausweichverhalten immer weiter aus. Die Verhaltenstherapie begleitet den Patienten in die angstmachende Situation, die Systemtherapie führt ihn zurück in die Herkunftsfamilie, wo er vor Vater oder Mutter seinen Standpunkt vertreten soll. Anxiolytika wie Temesta oder Valium dämpfen die Angst, verhindern aber neues Lernen. Zwangsverhalten versteht sie wie Rituale und EMDR als angstbindende Tätigkeit, deren Ursache ebenfalls in der Kindheit liegt und die systemisch in Richtung Selbstbestimmung statt Gehorsam bearbeitet wird.",
    },
    Zusammenfassung {
        datei: "Angst_Panik_Zwang_Sucht_Lüge_18.11.2013.pdf",
        titel: "Angst – Panik – Zwang – Sucht – Lüge",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I vom 18.11.2013. Angst schützt vor realen Gefahren, kann aber auch durch vorgestellte ausgelöst werden und sich zu einem Teufelskreis verselbständigen, der in der Panikattacke überläuft. Zwangsverhalten ist ritualisierte Angstbindung, wie die Rituale an allen grossen Lebensübergängen; auch Suchtmittel bis hin zu Tranquilizern unterdrücken Angst, führen aber über Gewöhnung und Entzug in einen neuen Teufelskreis. Als archaische Angstreaktionen beschreibt sie Flucht, Kampf und Totstellreflex («freeze reaction») und deutet das Lügen nicht moralisch, sondern als intelligentes Täuschungs- und Anpassungsverhalten, mit dem Kinder Eltern und sich selbst schonen. Angst entsteht oft, wenn Temperament und Familienregeln nicht zusammenpassen, etwa wenn ein impulsives Kind in einer Familie lebt, die keine Aggression zulässt; sie soll darum nicht mit Medikamenten oder Programmen unterdrückt, sondern im Familiensystem auf ihre Ursache hin untersucht werden.",
    },
    Zusammenfassung {
        datei: "Angst_Schuld_Sucht_07.05.1987.pdf",
        titel: "Angst – Schuld – Sucht",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Frauenverein (1987) über den Zusammenhang der drei Phänomene. Angst erzeugt Unlust mit körperlichen und geistigen Symptomen, die denen des Drogenentzugs gleichen; als Erziehungsmittel – Drohen mit dem Bölimann, mit Krankheit, Misserfolg oder Liebesentzug – führt sie nur zu Ausweich- oder in der Pubertät zu Rebellionsverhalten, und die Angst der Eltern überträgt sich oder treibt das Kind ins Risiko. Schuldgefühle binden den Einzelnen an die Gruppe, wollen Geschehenes ungeschehen machen und verhindern vorwärtsgerichtete Lösungen. Suchtmittel senken die Angst und erzeugen in hoher Dosis Euphorie; wer mit Angst und Schuld erzieht, verhindert Problemlösung und fördert die Sucht.",
    },
    Zusammenfassung {
        datei: "Angst_Schuldgefühle_Sinnfindung_29.08.1987.pdf",
        titel: "Angst – Schuldgefühle – Sinnfindung",
        text: "Stichworte von U. Davatz für eine Tagung für Eltern drogenabhängiger und drogengefährdeter Jugendlicher (1987). Angst ist ein Alarmsignal, das zu schnellen Lösungen drängt; chronische Angst ohne Lösung macht krank, belastet die Umgebung und lähmt die Kreativität. Hilfreich ist, die Angst wahrzunehmen, ihren Realitätsgehalt mit anderen zu prüfen und eine nicht perfekte, aber durchgeführte Lösung zu wählen. Schuldgefühle versuchen Geschehenes ungeschehen zu machen, halten den Einzelnen in der Gruppennorm und werden über die Erziehung weitergegeben; weniger Angst vor Fehlern heisst mehr Mut zur Handlung. Sinn findet man in den eigenen Rollen eher im Einsatz für etwas als im Kampf gegen die Gegenrolle.",
    },
    Zusammenfassung {
        datei: "Angst_ist_ein_schlechter_Lehrmeister_fürs_Leben_26.03.1987.pdf",
        titel: "Angst ist ein schlechter Lehrmeister für's Leben",
        text: "Stichworte von U. Davatz zu einem Vortrag bei der Mütterberatung (1987). Das Zeitalter der Information macht Eltern nicht nur frei, sondern auch ängstlich, weil sich Fachmeinungen widersprechen und Unglücksmeldungen überhandnehmen. Ob man eine rechte Mutter ist, lässt sich nur spüren, nicht aus Büchern lernen, und ein Vater muss nicht immer recht haben: Erziehung fördert ein natürliches Wachstum und ist kein Machtkampf, und verfehlt das Kind die Ziele der Eltern, ist das Unvermögen und keine Bosheit. Angst entsteht durch unsichere oder wechselhafte Eltern, gegenseitiges Disqualifizieren, Abschreckung und Verbote. Besser seien eigene Instinkte, Erfahrungsräume im geschützten Rahmen, aus Fehlern lernen und Vertrauen ins Kind und ins Leben.",
    },
    Zusammenfassung {
        datei: "Anorexia_09.11.1992.pdf",
        titel: "",
        text: "Einseitiges Stichwortskript von U. Davatz für den Familientherapie-Kurs I 1992 zur Anorexie. Hunger und Essverhalten gelten als Instinktverhalten, das Erziehungspersonen nicht befehlen können, ohne den natürlichen Instinkt zu stören; schlechtes Essen ist nur Symptom allgemeiner Schwierigkeiten. Greifen Eltern erzieherisch ein, entsteht ein Machtkampf, der das Essverhalten weiter stört und das Symptom fixiert. Therapeutisch empfiehlt sie, vom Symptom zu defokussieren, es sogar zu tabuisieren, damit der Instinkt wieder in Gang kommt, und die Persönlichkeitsentwicklung der Eltern zu fördern.",
    },
    Zusammenfassung {
        datei: "Anpassungsmechanismen_innerhalb_der_Kernfamilie_in_09.11.2018.pdf",
        titel: "Anpassungsmechanismen innerhalb der Kernfamilie in Überforderungssituationen",
        text: "Überarbeitete Fassung des Skripts von 1991, von U. Davatz für den Familientherapie-Kurs I 2018 neu gesetzt und ausführlicher formuliert. In Anlehnung an Bowen beschreibt sie drei Mechanismen, mit denen eine Kernfamilie Überforderung auffängt. Der Ehekonflikt entsteht, wenn sich Vater und Mutter gleichzeitig verantwortlich fühlen und ihre Wertvorstellungen samt ungelöster Konflikte der Herkunftsfamilien aufeinanderprallen; er ist ein Dominanzkampf, der umso länger dauert, je egalitärer die Ehe ist, wirkt aber nach der Versöhnung auch verbindend und als Ventil. Gelingt das nicht, wird ein Partner zum Symptomträger mit psychischen, körperlichen oder sozialen Symptomen, oder die Spannungen werden auf die Kinder als schwächste Glieder projiziert. Jeder Mechanismus entlastet vorübergehend; wird einer allein über lange Zeit verwendet, entsteht ein Teufelskreis mit Krankheitscharakter.",
    },
    Zusammenfassung {
        datei: "Anpassungsmechanismen_innerhalb_der_Kernfamilie_in_29.10.1991.pdf",
        titel: "Anpassungsmechanismen innerhalb der Kernfamilie in Überforderungssituationen",
        text: "Das Skript von U. Davatz für den Familientherapie-Kurs I (1991) beschreibt drei Anpassungsmechanismen der Kernfamilie in Überforderungssituationen, in Anlehnung an Bowen. Der Ehekonflikt wird als Dominanzkampf und zugleich als Ventil verstanden. Reicht dies nicht aus, wird einer der Partner zum Symptomträger (psychisch, physisch oder sozial), oder die elterlichen Spannungen werden auf das schwächste Glied, das Kind, projiziert. Jeder dieser Mechanismen entlastet kurzfristig und führt bei ausschliesslicher, langer Anwendung in einen Teufelskreis mit Krankheitscharakter.",
    },
    Zusammenfassung {
        datei: "Arbeiten_mit_ganzen_Familien_als_Mehrpersonensystem_12.11.2018.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I 2018 vom 12.11.2018, das das medizinische Modell der Einzelbehandlung der Familiensystemtherapie gegenüberstellt. Das medizinische Modell sieht den Patienten als Einzelwesen und wirkt mit Medikamenten auf sein Gehirn, obwohl das Gehirn das soziale Organ des Menschen ist. Die Systemtherapie betrachtet die Familie als Organismus und hat nach Boszormenyi-Nagy eine «vielgerichtete Parteilichkeit» für das Wohl des ganzen Systems, denn das Kindswohl hängt von Mutter, Vater und ihrer Beziehung ab. Therapeuten sind keine Advokaten eines Einzelnen und erziehen nicht zur Gesundheit, sondern stärken die Resilienz des Systems, indem sie an den «Lötstellen» der Beziehungen arbeiten, und entscheiden, welche Beziehung gerade an der Reihe ist. Das verlangt ganzheitliche Aufmerksamkeit, Flexibilität statt vorgefasster Muster und einen neugierigen, wissenschaftlichen Geist.",
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
        datei: "Bedeutung_von_Randgruppen_in_unserer_Gesellschaft_20.11.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Club von Berufs- und Geschäftsfrauen (1986). Evolutiv betrachtet gehen neue Entwicklungen meist von der Peripherie aus, nicht vom Zentrum der Macht; auch kulturelle Bewegungen wie das Christentum oder neue wissenschaftliche Ideen kamen von Randgruppen, bis sie zur Mehrheit wurden und erstarrten. Heute würden Minderheiten unterdrückt, wie im Zoo ausgestellt oder benützt, um sich mit dem Einsatz für sie Prestige und politische Macht zu verschaffen. Sie plädiert dafür, Randgruppen zu tolerieren, interessiert zuzuhören und von ihnen zu lernen, im direkten Kontakt zu helfen statt mit Grossprogrammen, und sich der eigenen Zugehörigkeit zu einer Minderheit nicht zu schämen.",
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
        datei: "Behandlung_des_Familiensystems_als_Ganzes_10.12.2018.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I 2018 vom 10.12.2018. Das medizinische Modell behandelt den Patienten als isoliertes Einzelwesen, eine Art «Kaspar Hauser», stellt Diagnosen und unterdrückt Symptome mit Psychopharmaka und Verhaltensprogrammen; wie die Symptome im sozialen Umfeld entstanden sind, bleibt offen, und das Umfeld wird nicht als Ressource genutzt. Wer den Patienten nur zur Anpassung an die Norm motiviert, passt ihn den Werten der eigenen Herkunftsfamilie an. Die Familientherapie betrachtet das System über drei bis vier Generationen als lebendigen Organismus, beobachtet Interaktionsmuster ohne zu werten und übt nach Boszormenyi-Nagy eine «vielgerichtete Parteilichkeit»: Es gibt keine reinen Täter und Opfer. Der Therapeut wirkt als Katalysator, darf klar Stellung beziehen, verlangt aber keinen Gehorsam und keine Compliance; behandelt werden Menschen in ihrem Familiensystem, nicht Symptome.",
    },
    Zusammenfassung {
        datei: "Behandlungsansätze_bei_Erwachsenen_mit_ADS_15.11.1997.pdf",
        titel: "",
        text: "Vortrag von Dr. med. Johanna Krause, Nervenärztin und Psychotherapeutin, am 15.11.1997 in Forchheim für die Elterninitiative zur Förderung hyperaktiver Kinder; sie hatte bemerkt, dass Eltern betroffener Kinder oft selbst Hilfe bräuchten. Nach den Kriterien von DSM-IV und ICD-10 mit den drei Untertypen beschreibt sie, wie sich ADS bei Erwachsenen zeigt, oft verkannt, weil das Syndrom als reine Kinderkrankheit galt. Ausführlich schildert sie Unterschiede zwischen Frauen und Männern, Partnerschaften zweier Betroffener, das Rückzugsbedürfnis nach der Pubertät und das «Superinteresse», mit dem Betroffene ein Ziel mit grosser Energie verfolgen. Die Behandlung lehnt sie an das Schema von Döpfner an: Aufklärung und Beratung, Psychotherapie und psychosoziale Interventionen sowie medikamentöse Therapie. Abgedruckt ist der Text vom Bundesverband der Elterninitiativen.",
    },
    Zusammenfassung {
        datei: "Betreuung_psychisch_Kranker_in_der_Spitex_05.06.2003.pdf",
        titel: "",
        text: "Vortrag für die Spitex Windisch über die Betreuung psychisch Kranker. Davatz versteht psychische Krankheiten als Beziehungskrankheiten und zeichnet den Weg von der Sozialpsychiatrie der 68er-Jahre (Basaglia, Maxwell Jones) zur gemeindenahen Versorgung durch die Spitex nach. Da die Hospitalisation meist Trauma und Stigmatisierung bedeute, sei es besser, Betroffene im natürlichen Umfeld zu belassen. Spitex-Mitarbeitende sollen keine Diagnosen behandeln, sondern eine normale, authentische und nicht auf Helfen ausgerichtete Beziehung anbieten, bei Überforderung eine Fachperson beiziehen und ihre «psychiatrische Unwissenheit» behalten.",
    },
    Zusammenfassung {
        datei: "Beziehungen_Bezugsprobleme_25.11.1987.pdf",
        titel: "Beziehungen – Bezugsprobleme: Schwierigkeiten im Umgang mit Teenager",
        text: "Stichworte von U. Davatz zu einem Vortrag bei ELPOS Aargau (1987). Teenager, ob mit POS oder ohne, fordern Erwachsene heraus: Geschlechtsreife, eigene Persönlichkeit und Intimsphäre, Rückzug, Verletzlichkeit, Stimmungsschwankungen, eigene Werte, Rebellion und Ablösung. Ungünstig sind Überfokussieren oder Leugnen der Geschlechtsreife, Stören der Intimsphäre, endlose Machtkämpfe und das Deuten der Ablösung als Undankbarkeit. Hilfreich sind taktvolle Aufklärung, Respekt vor der Verpuppungszeit, klare eigene Werte ohne Druck, gezielte Prioritäten und Streitgespräche ohne gewinnen zu wollen. Die Ablösung soll den Eltern Freiraum für die eigene zweite Lebenshälfte geben; der wichtigste Prozess ist das Abgrenzen.",
    },
    Zusammenfassung {
        datei: "Bureaucratization_and_De-Bureaucratization_of_Social_Welfare_25.08.1986.pdf",
        titel: "Bureaucratization and De-Bureaucratization of Social Welfare – Democratic Alternatives to the Bureaucratized Welfare State",
        text: "Englisches Original des Vortrags von U. Davatz an der Konferenz des Gottlieb Duttweiler Instituts vom August 1986, mit Zusammenfassung und Literaturliste. Die Schwäche des Wohlfahrts- und Gesundheitswesens liegt für sie in der Prämisse des mächtigen «great provider», der wachsende, dysfunktionale Bürokratien hervorbringt, ängstliche Verwalter auswählt und mit restriktiven Regeln statt Erneuerung regiert. Als Alternative schlägt sie kleine funktionelle Einheiten vor, im Gesundheitswesen aus Ärzten, Pflegenden, Versicherern und Versicherten, mit einer an Gesundheitsprogramme gebundenen Finanzierung. Dazu gehören selbständige Pflegende in der Gemeinde und eine dezentrale Ausbildung der Gesundheitsberufe, die mehr über Beziehungen als über Pathologie lehrt.",
    },
    Zusammenfassung {
        datei: "Burnout_und_Gesellschaft_09.01.2017.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2016, datiert vom 9.1.2017. Der Mensch kennt Kooperation und Wettkampf; in der globalisierten Wirtschaft ist die Welt zu einem einzigen Jagdgebiet geworden, und besonders in Dienstleistungen wie Gesundheits- und Bildungswesen werden Menschen zu immer mehr Leistung in kürzerer Zeit angetrieben, was sie als «moderne Sklavenwirtschaft» bezeichnet. Wer sich vor diesem Druck nicht schützen kann, gerät ins Burnout; fehlt es an Vorgesetzten, die sich für ihre Angestellten verantwortlich fühlen, folgen Ausfälle, Kündigungen, Know-how-Verlust und höhere Gesundheitskosten. Burnout entsteht aber auch aus persönlichen Gründen, wenn jemand unmögliche Ziele verfolgt und sich gegen die eigene Natur anpasst. Diese überfordernden Muster stammen meist aus der Herkunftsfamilie und müssen von dort her aufgeschlüsselt und verändert werden.",
    },
    Zusammenfassung {
        datei: "Bürokratisierung_und_Entbürokratisierung_der_Sozialhilfe_25.08.1986.pdf",
        titel: "Bürokratisierung und Entbürokratisierung der Sozialhilfe – demokratische Alternativen zum bürokratischen Wohlfahrtsstaat",
        text: "Vortrag von U. Davatz an einer Konferenz des Gottlieb Duttweiler Instituts (1986), hier in einer späteren Reinschrift. Sie vergleicht den modernen Wohlfahrtsstaat mit den bürokratischen Vorstaaten im alten Ägypten: Als «ängstlicher grosser Versorger» ziehe er sicherheitsbedürftige Verwalter an, regiere mit einschränkenden Regeln wie ein Auto nur mit Bremsen und begünstige so unverantwortliches, krankes Verhalten. Als Alternative schlägt sie kleine funktionelle Einheiten vor, etwa für die Volksgesundheit aus Ärzten, Pflegenden, Versicherern und Versicherten mit einer eigenen «Gesundheitsbank». Dezentralisierung, selbständige Pflegende in der Gemeinde, eine dezentrale Ausbildung und ein soziomedizinisches Krankheitskonzept sollen Prävention fördern und die Kosten senken.",
    },
    Zusammenfassung {
        datei: "Chronische_Krankheit_in_der_Familie,_Gewalt_und_Alkohol_in_18.12.2000.pdf",
        titel: "Chronische Krankheit in der Familie, Gewalt und Alkohol in der Familie",
        text: "Eine chronische Krankheit verlangt von der Familie eine langfristige Anpassung, die krankheitsfördernd oder hilfreich sein kann, so das Skript von U. Davatz für den Familientherapie-Kurs I 2000. Körperliche Krankheiten lösen Schonverhalten aus, Verhaltensstörungen wie POS, Schizophrenie, Sucht, Gewalt oder Delinquenz eher Übererziehung und Angst vor dem Urteil der Nachbarn; in beiden Fällen dreht sich alles um das kranke Kind, während die Geschwister zu kurz kommen oder eine Ersatzelternrolle übernehmen. Diese Überanpassung verstärkt das Symptom und verhindert Entwicklung, besonders wenn die Eltern das Handicap leugnen. Therapeutisch müssen die Eltern das Handicap annehmen und betrauern, die Überanpassung auf ein normales Mass zurückführen und lernen, vom Kind weder zu viel noch zu wenig zu verlangen; die Geschwister werden aus der Elternrolle entlassen, und der Therapeut begleitet die Angehörigen als Coach.",
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
        datei: "Das_Kind_als_schwächstes_Glied_im_System,_wie_gehen_wir_28.05.1986.pdf",
        titel: "Das Kind als schwächstes Glied im System, wie gehen wir damit um?",
        text: "Vortrag von U. Davatz an einer Tagung «Schule und Prävention» (1986). Die Schule hat zwei teils gegensätzliche Aufgaben: dem Kind Fähigkeiten, Wissen und Lebensbewältigung zu vermitteln und der Gesellschaft die Hierarchie zu erhalten, indem sie das Kind mit Strafe und Belohnung an die Normen anpasst. An Beispielen zeigt sie den Konflikt: Das aufgeweckte Kind stört die Ordnung, das kreative wird auf die Norm korrigiert, das lerngestörte überfordert oder der Lehrer gilt als zu weich, das verhaltensgestörte diszipliniert, wodurch die Lernfähigkeit sinkt. Sie bittet Lehrer um mehr Mut zu individuellen Zielen, Schulbehörden um weniger enge Massstäbe, weil eine Normgesellschaft nicht überlebensfähig sei, und Eltern um Geduld für Wachstum.",
    },
    Zusammenfassung {
        datei: "Das_Konzept_des_Cut-off-Verhaltens_in_einer_Beziehung_oder_09.11.2010.pdf",
        titel: "Das Konzept des «Cut-off»-Verhaltens in einer Beziehung oder die gewaltsame Abtrennung einer Fusions-Beziehung",
        text: "Fassung von 2010 des Skripts von U. Davatz für den Familientherapie-Kurs I, zwischen der ersten von 1987 und der von 2018, mit der sie fast wörtlich übereinstimmt. Bowens «Cut-off» ist das gewaltsame Abwenden aus einer zu intensiv gewordenen Fusionsbeziehung und gehört zum Fluchtverhalten. Beispiele sind Jugendliche, die das Elternhaus abrupt verlassen, Paare, deren Liebe in Hass umschlägt, zerbrochene Geschäftspartnerschaften und Geschwister im Erbstreit; meist folgt eine neue Fusionsbeziehung, und in manchen Familien wird der Cut-off über Generationen zur Tradition. Er kann auch rein emotional im selben Haushalt bestehen und bringt nur momentane Entlastung, die Problematik besteht über die Distanz hinweg weiter.",
    },
    Zusammenfassung {
        datei: "Das_Konzept_des_Cut-off-Verhaltens_in_einer_Beziehung_oder_15.03.2018.pdf",
        titel: "Das Konzept des «Cut-off»-Verhaltens in einer Beziehung oder die gewaltsame Abtrennung einer Fusions-Beziehung",
        text: "Überarbeitete Fassung des Skripts von 1987, von U. Davatz für den Familientherapie-Kurs I 2018 (15.3.2018) neu gefasst. Bowens «Cut-off» ist ein gewaltsames Abwenden aus einer Beziehung, die zu intensiv geworden ist, und gehört zum Fluchtverhalten vor einer emotionalen Fusion. Beispiele sind Jugendliche, die das Elternhaus abrupt verlassen oder hinausgeworfen werden, Paare, deren Liebe in Hass umschlägt und die sich scheiden lassen, zerbrochene Geschäftspartnerschaften und Geschwister, die sich bei Erbstreitigkeiten überwerfen, sodass sich ganze Familienzweige nicht mehr kennen. Auf jeden Cut-off folgt meist eine neue Fusionsbeziehung, und in manchen Familien wird er über Generationen zur Tradition, erkennbar an der Verteilung über Länder und Kontinente; er kann auch rein emotional im selben Haushalt bestehen. Der Cut-off bringt nur momentane Entlastung, die Problematik besteht über die Distanz hinweg weiter.",
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
        datei: "Depression_als_Verliererkrankheit_oder_Totstellreflex_27.09.1999.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I vom 27.9.1999, das die Depression verhaltensbiologisch als arretiertes Kampfverhalten («arrested fight reaction») und damit als Verliererreaktion deutet. Wie ein Reptilienmännchen, das im Kampf verliert, zeigt der depressive Mensch gebeugte Haltung, fehlende Mimik, Lustlosigkeit, negative Gedanken und gestörten Schlaf. Ursachen sind etwa, nie kämpfen gelernt oder immer gehorcht zu haben, Kampf für unwürdig zu halten, niemandem wehtun zu wollen oder Angst vor der Übermacht des anderen. Therapeuten machen oft den Fehler, aufzumuntern, Funktionen abzunehmen, zu überreden und schliesslich wütend zu werden oder Liebe zu entziehen. Hilfreich ist, nur so viel Energie zu investieren, wie man hat, sich vom Patienten auf einem Gebiet helfen zu lassen, in dem er besser ist, und ihm geduldig zu Erfolgen in eigenen Konflikten zu verhelfen.",
    },
    Zusammenfassung {
        datei: "Depression_als_Verliererkrankheit_oder_Totstellreflex_27.09.2004.pdf",
        titel: "",
        text: "Fassung vom 27.9.2004 des Skripts von U. Davatz für den Familientherapie-Kurs I, im Text weitgehend gleich wie die Fassung von 1999, neu gesetzt mit Fusszeile. Die Depression wird verhaltensbiologisch als arretiertes Kampfverhalten und Verliererreaktion gedeutet, mit gebeugter Haltung, Lustlosigkeit, negativen Gedanken und gestörtem Schlaf. Ursachen sind etwa, nie kämpfen gelernt oder immer gehorcht zu haben, Kampf für unwürdig zu halten oder Angst vor der Übermacht des anderen. Statt aufzumuntern oder Aufgaben abzunehmen, soll der Therapeut sich vom Patienten auf einem Gebiet helfen lassen, in dem dieser besser ist, und ihm geduldig zu Erfolgen in eigenen Konflikten verhelfen.",
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
        datei: "Depressionen_05.05.1988.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Haus- und Krankenpflegeverein (1988). Depression beschreibt sie als gefühlsmässiges Wellental, verursacht durch lang dauernde Überforderung, Überanpassung, dauerndes Verlieren im Dominanzkampf einer Partnerschaft oder Unterordnung. Der depressive Mensch soll herausfinden, wo er sich überfordert, sich besser abgrenzen, weniger Verantwortung für andere tragen und sich in Auseinandersetzungen behaupten. Angehörige sollen weder oberflächlich aufmuntern noch ihm Funktionen abnehmen, sondern ihn als Menschen mit eigener Lebensgeschichte sehen, die Beziehung halten und sich selbst abgrenzen.",
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
        datei: "Der_Psychiatriepatient_aus_der_Sicht_des_Arztes_23.06.1988.pdf",
        titel: "",
        text: "Entwurf von U. Davatz für ein Informationsblatt (1988). Psychische Krankheit versteht sie als verzweifelten Anpassungsversuch an eine ausweglose Lebenssituation und als Hilfeschrei; behandelt werden soll darum nie nur das Symptom, sondern immer auch die Lebenssituation. An den Patienten richtet sie, dass keine Heilung ohne seine Mithilfe gelingt und der gute Arzt ihm Kontrolle zurückgibt statt sie zu nehmen. Die Familie bittet sie, nicht nach Schuld zu suchen, sondern Verantwortung neu zu verteilen. Aus Sicht des Patienten beschreibt sie das Erleben als schwarzes Schaf oder Opfer, aus Sicht der Eltern die Ohnmacht, die zu immer neuen Ersatzeltern führt, statt selbst Beratung zu holen.",
    },
    Zusammenfassung {
        datei: "Designerdrogen,_die_modernen_chemischen_Problemlöser_der_26.09.2002.pdf",
        titel: "Designerdrogen, die modernen chemischen Problemlöser der Jugend",
        text: "Designerdrogen werden im Vortrag für den Samariterverein Wölflinswil als Teil der Modekrankheit Drogensucht dargestellt: Weil Süchtige zur Gewohnheit wurden, brauche der Markt neue, attraktiv vermarktete Angebote. Der Begriff «Designer-» oder «Lifestyle-Droge» verharmlose die Substanzen und suggeriere einen Lebensstil. Für Jugendliche sind sie ein billiger privater Nachbau der schnellen chemischen Problemlöser der Pharmaindustrie. Am Beispiel von Ecstasy werden Wirkung und Risiken genannt; das Betäubungsmittelgesetz komme den ständig neuen Substanzen nicht nach. Schutz biete vor allem, wenn Eltern und Erzieher den Glauben an schnelle chemische Lösungen relativieren und Zeit für nachhaltige Problemlösungen vermitteln.",
    },
    Zusammenfassung {
        datei: "Die_Dreieckbeziehung_oder_Triangle_nach_Murray_Bowen_09.11.1987.pdf",
        titel: "Die Dreieckbeziehung oder «Triangle» nach Murray Bowen",
        text: "Ausgangspunkt dieses Skripts von U. Davatz für den Familientherapie-Kurs I 1987 ist das Dominanzverhalten, mit dem Tiere wie Menschen Ressourcen, Territorium oder Partner verteidigen; der Dominanzkampf zwischen zwei Individuen endet mit Sieger und Verlierer. Zieht der Schwächere einen Helfer bei, entsteht die Dreieckbeziehung, die Bowen «Triangle» nannte: Der ursprüngliche Konflikt beruhigt sich scheinbar, die Drittperson führt ihn weiter, und er kann im Kreis herumgeschoben werden. Weil der Kampf so nie entschieden wird, bildet sich keine klare Hierarchie, und es entsteht ein «Perpetuum mobile» des Konflikts. Therapeuten sollen merken, wenn sie hineingezogen werden, und sich wieder lösen, denn ihre Aufgabe ist nicht die des Richters oder Beschützers, sondern die eines Katalysators für die ganze Familie. In der eigenen Familie soll man den Konflikt an die beiden Hauptpersonen zurückgeben und negative Informationen nicht geheim halten, sondern neutral an die richtige «Adresse» weitergeben.",
    },
    Zusammenfassung {
        datei: "Die_Dreieckbeziehung_oder_Triangle_nach_Murray_Bowen_23.04.2018.pdf",
        titel: "Die Dreieckbeziehung oder «Triangle» nach Murray Bowen",
        text: "Überarbeitete Fassung des Skripts von 1987, von U. Davatz für den Familientherapie-Kurs I 2018 (23.4.2018) neu gesetzt; der Inhalt ist derselbe. Aus dem Dominanzkampf zwischen zwei Individuen entsteht eine Dreieckbeziehung, wenn der Schwächere einen Helfer beizieht; der ursprüngliche Konflikt beruhigt sich scheinbar, die Drittperson führt ihn weiter, und er kann im Kreis herumgeschoben werden, ohne je entschieden zu werden. Therapeuten sollen merken, wenn sie hineingezogen werden, und sich wieder lösen, weil sie Katalysator für die ganze Familie sind, nicht Richter oder Beschützer. Im eigenen Familiensystem soll man den Konflikt an die beiden Hauptpersonen zurückgeben und negative Informationen neutral an die richtige «Adresse» weiterleiten.",
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
        datei: "Die_Kunst_des_Fragenstellens_25.10.2010.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 25.10.2010 über das Fragen in der Familientherapie. Inhaltsfragen erfassen die Struktur der Familie über harte Fakten mit dem Genogramm als Richtschnur; wo die Familie nichts weiss oder nichts sagen will, liegen meist Tabus oder ein emotionaler «Cut-off». Prozessfragen fragen zirkulär, etwa was der Partner über jemanden denkt, und verzichten auf das schuldinduzierende «Warum?» zugunsten von was, wann, wo und unter welchen Umständen. Gute Fragen sind neutral, nicht suggestiv, von Neugier und Forschergeist getragen und bringen das festgefahrene System etwas aus dem Gleichgewicht. Teleologische und hypothetische Fragen, etwa was jemand anders machen würde, könnte er sein Leben neu beginnen, bringen verborgene Wünsche und Motive zum Vorschein.",
    },
    Zusammenfassung {
        datei: "Die_Zweierbeziehung_in_der_Ehe_23.11.1992.pdf",
        titel: "",
        text: "Das Skript von U. Davatz (Familientherapie-Kurs I, 1992) beschreibt die Zweierbeziehung in der Ehe als labiles Gleichgewicht, auf dem die Kleinfamilie ihre gesamten Ressourcen aufbaut. Typische Fehlentwicklungen entstehen aus positiven und negativen Erwartungen, die aus der Beziehung zu den eigenen Eltern auf den Partner übertragen werden, sowie aus Erziehungskonflikten durch gegensätzliche Erziehungsstile. Ausserdem werden männliches und weibliches Stress- und Erziehungsverhalten gegenübergestellt. Als stabilisierende Faktoren nennt das Skript gute Beziehungen zur Ursprungsfamilie, eigene Interessen, klare Selbstdarstellung statt Anpassungsversuche am Partner und Zuhörenkönnen.",
    },
    Zusammenfassung {
        datei: "Die_soziale_Vererbung_15.01.2018.pdf",
        titel: "",
        text: "Überarbeitete Fassung des Skripts «Die soziale Vererbung» von 1998, von U. Davatz für den Familientherapie-Kurs I 2018 (15.1.2018) neu gesetzt und sprachlich leicht angepasst; der Inhalt ist derselbe. Fachleute sollen von statischen Diagnosen wegkommen und dynamische Prozessmuster beobachten lernen, mit der Biologie statt der Physik als Vorbild, im Kontext, mit Hypothesen und im Aushalten des Nichtwissens. Menschliches Verhalten ist vor allem erlernt; die in der Familie weitergegebenen Lernprogramme sind kontextgebunden und geraten spätestens in der Ehe in Konkurrenz, wo dann Pathologie statt Anpassung entsteht. Erkannt wird die soziale Vererbung über Genogramm, generationenübergreifende Muster und emotional beladene Familienthemen; die Therapie entkodiert das fehlangepasste Muster und übt mit einem Coach ein neues ein.",
    },
    Zusammenfassung {
        datei: "Die_soziale_Vererbung_19.01.1998.pdf",
        titel: "",
        text: "Um soziale Vererbung zu verstehen, müssen naturwissenschaftlich geschulte Fachleute von statischen Diagnosen wegkommen und dynamische Prozessmuster («patterns») beobachten lernen, so das Skript von U. Davatz für den Familientherapie-Kurs I 1998; Vorbild ist die Biologie statt Physik und Mathematik. Als Grundregeln nennt sie unter anderem den Vorrang des Individuums vor der Statistik, das Denken im Kontext, offene Systeme, teilnehmende Beobachtung, Arbeit mit Hypothesen und das Aushalten des Nicht-Wissens. Menschliches Verhalten ist mehr erlernt als genetisch bestimmt; die in der Familie weitergegebenen Lernprogramme sind kontextgebunden und versagen bei Kontextwechseln, spätestens in der Ehe, wenn verschiedene Familienmuster aufeinandertreffen. Erkannt wird soziale Vererbung über das Genogramm, generationenübergreifende Interaktionsmuster und emotional beladene Familienthemen, vor deren Hintergrund der Indexpatient betrachtet wird. Therapie heisst, das fehlangepasste Muster zu entkodieren und mit einem geduldigen Coach neu einzuüben.",
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
        datei: "Differenzierung_von_der_Ursprungsfamilie_09.11.2018.pdf",
        titel: "",
        text: "Überarbeitete Fassung des Skripts von 1998, von U. Davatz für den Familientherapie-Kurs I 2018 neu gesetzt; der Inhalt ist derselbe. Die Differenzierung von der Ursprungsfamilie gilt nach Bowen als lebenslanger Nachreifungsprozess, in dem man unreflektierte, prägende Reaktionsmuster aufweicht, indem man sich als Erwachsener erneut mit der eigenen Familie auseinandersetzt. Kognitiv geschieht das als «narrativer Rekonstruktivismus», indem man sich die Familiengeschichte von verschiedenen Personen erzählen lässt, auch die Tabu-Geschichten; emotionell, indem man das eigene Verhalten gegenüber Mutter, Vater oder Geschwistern leicht verändert und so das ganze System bewegt. Der Therapeut beschleunigt den Prozess mit Fragen von aussen und begleitet ihn als Coach mit engagierter Neugier.",
    },
    Zusammenfassung {
        datei: "Differenzierung_von_der_Ursprungsfamilie_25.10.1998.pdf",
        titel: "",
        text: "Die Differenzierung von der Ursprungsfamilie ist das Kernstück der Theorie Bowens und ein lebenslanger Prozess der Nachreifung, so das Skript von U. Davatz für den Familientherapie-Kurs I 1998. Unreflektierte Reaktionsmuster aus Erziehung und Familienerfahrung engen die seelische Bewegungsfreiheit ein und lassen sich aufweichen, indem man als Erwachsener zur Ursprungsfamilie zurückkehrt. Bei der kognitiven Differenzierung, dem «narrativen Rekonstruktivismus», lässt man sich die Familiengeschichte von verschiedenen Personen erzählen, ohne vorgefasste Meinung und auch mit Blick auf Tabu-Geschichten. Bei der emotionellen Differenzierung verändert man das eigene Verhalten gegenüber Mutter, Vater oder Geschwistern leicht, was das ganze Familiensystem bewegt und Energie freisetzt. Der Therapeut beschleunigt diesen Prozess durch Fragen von aussen und begleitet ihn als Coach mit engagierter Neugier.",
    },
    Zusammenfassung {
        datei: "Differenzierung_von_der_Ursprungsfamilie_25.10.1998_2.pdf",
        titel: "",
        text: "Zweite, neu gesetzte Fassung des Skripts von U. Davatz für den Familientherapie-Kurs I 1998, mit zentriertem Kopf und Seitenzahlen; der Text ist derselbe. Die Differenzierung von der Ursprungsfamilie gilt nach Bowen als lebenslanger Prozess, in dem man unreflektierte Reaktionsmuster aus der eigenen Familie aufweicht. Kognitiv geschieht das durch das Erzählenlassen der Familiengeschichte aus verschiedenen Perspektiven, emotionell durch leicht verändertes eigenes Verhalten gegenüber Eltern und Geschwistern. Der Therapeut begleitet den Prozess mit Fragen von aussen und als Coach.",
    },
    Zusammenfassung {
        datei: "Differenzierungsprozess_von_der_Ursprungsfamilie_09.11.1987.pdf",
        titel: "",
        text: "Der Differenzierungsprozess von der Ursprungsfamilie ist ein lebenslanger psychosozialer Lernprozess, in dem man sich vom eigenen Familiensystem abgrenzt und an Autonomie gewinnt, ohne den Kontakt zu verlieren, so das Skript von U. Davatz für den Familientherapie-Kurs I 1987. Er läuft in Zweierbeziehungen ab und beginnt je nach Ausgangslage nach Bowen bei der Fusionsbeziehung mit Abgrenzung oder beim «cut-off» mit neuem Brückenschlag. Kern ist, zu wichtigen Themen eine eigene, möglichst emotionsfreie Position zu beziehen, ohne zu überreden, und zugleich zuhören zu können. Heisse und tabuisierte Themen führen leicht in hitzigen Glaubenskrieg oder eisigen Abbruch; man soll sie erst angehen, wenn man zu allen lebenden Familienmitgliedern Kontakt hergestellt und harte Daten gesammelt hat, und nichts forcieren. Das Ergebnis ist ein flexibleres, anpassungsfähigeres Individuum, das seinen eigenen Charakter besser einsetzen kann.",
    },
    Zusammenfassung {
        datei: "Differenzierungsprozess_von_der_Ursprungsfamilie_09.11.1987_2.pdf",
        titel: "",
        text: "Zweite Fassung des Skripts von U. Davatz für den Familientherapie-Kurs I 1987, enger gesetzt und anders umbrochen; der Text ist derselbe. Die Differenzierung von der Ursprungsfamilie wird als lebenslanger psychosozialer Lernprozess beschrieben, der in Zweierbeziehungen abläuft und nach Bowen von der Fusionsbeziehung oder vom «cut-off» ausgeht. Kern sind eine eigene, möglichst emotionsfreie Position und die Fähigkeit zuzuhören; heisse und tabuisierte Themen soll man erst nach dem Aufbau eines persönlichen Beziehungsnetzes und ohne Zwang angehen.",
    },
    Zusammenfassung {
        datei: "Drogenprobleme_in_der_Familie_wie_gehe_ich_damit_um_17.03.1986.pdf",
        titel: "Drogenprobleme in der Familie – wie gehe ich damit um?",
        text: "Stichworte von U. Davatz zu einem Vortrag für eine Elternvereinigung drogenabhängiger Jugendlicher (1986). Soziologisch sieht sie Drogenkonsum als Zeiterscheinung, Mode, Protest und Folge des Zeitalters der «chemischen Wunder», individuell als untauglichen Problemlösungsversuch, Flucht, Trotz, Angstbekämpfung und Grenztesten. In der Familie tritt er als Symptom auf, wo Konflikte unterdrückt oder gehäuft werden, Grenzen fehlen oder Selbstmedikation vorgelebt wird. Eltern rät sie, die eigene Angst ohne Medikamente zu senken, ruhig und auch humorvoll das Gespräch zu suchen, das Drogenkind nicht zum einzigen Lebensinhalt zu machen, die Ehe und die Kontakte zur Familie zu pflegen, nichts zu tabuisieren und sich fachlichen Rat zu holen.",
    },
    Zusammenfassung {
        datei: "Drogenprophylaxe_und_Drogentherapie_25.11.1985.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einer Informationsveranstaltung der Sozialdienste des Bezirks Rheinfelden über Drogen im Aargau (1985). Sie listet die stationären und ambulanten Angebote der Drogentherapie im Kanton auf und beschreibt die Haltung in der Therapie: Widerstand bieten ohne Ablehnung, hohe Frustrationstoleranz, kleine Schritte und kein Kampf gegen die Krankheit. Für die Prophylaxe fordert sie an allen Drogenberatungsstellen präventiv tätige Personen, die Lehrer und Eltern fortbilden und beraten und ihre Erfahrung an die Schulbehörden zurückmelden. Die Aufgabe jedes Bürgers sei, im kritischen Augenblick richtig zu handeln: weniger chemische Schnelllösungen, Kinder mit Frustrationen umgehen lassen, Konflikte lösen statt unterdrücken und sich Hilfe holen – die Problemlösung der Eltern von heute sei die Prophylaxe der Jugendlichen von morgen.",
    },
    Zusammenfassung {
        datei: "Eifersucht_23.08.2010.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I vom 23.8.2010, ausgehend vom Spruch «Eifersucht ist eine Leidenschaft, die mit Eifer sucht, was Leiden schafft». Eifersucht entsteht, wo Mangel an Fürsorge herrscht, und ist ein verstecktes Konkurrenz- oder Dominanzverhalten; bei Erwachsenen tritt sie als Angst vor Partnerverlust oder als Neid auf, wird oft auf den anderen projiziert und kann bis zu Verfolgungsideen gehen. Die tiefere Ursache sieht sie im Familiensystem, das ein Kind zu kurz kommen liess oder funktionalisierte, sodass es als Erwachsener Anspruch auf Wiedergutmachung empfindet. Eifersucht lässt sich nicht über Macht und Kontrolle des Partners lösen, sondern nur intrapsychisch, indem das Defizit aus der Kindheit aufgedeckt und die eigenen Bedürfnisse bewusst werden. Auch die Eifersucht unter Geschwistern führt sie auf Eltern zurück, die sich in deren offene Rivalität einmischen und ein Kind für sich vereinnahmen.",
    },
    Zusammenfassung {
        datei: "Einbezug_von_Familienangehörigen_bei_Depressionen_03.12.2003.pdf",
        titel: "",
        text: "Weiterbildung am Stützpunkt Biel über den Einbezug von Angehörigen bei Depressionen. Davatz geht vom Konzept der «high expressed emotions» (Leff und Vaughn) aus und deutet die Depression familientherapeutisch und ethologisch als Verliererkrankheit im Sinne eines arretierten Kampfverhaltens, das in appellatives Bettelverhalten übergeht. Helfer geraten dabei in eine dominante Position, der Depressive wehrt sich dagegen und macht sie zu «hilflosen Helfern» (sekundärer Machtgewinn). Angehörige sollen deshalb die Helferrolle verlassen, Kampfmuster und Übergriffe erkennen, Störendes aus der Ich-Position ansprechen, nur frei gegebene Hilfe leisten und sich vom Patienten helfen lassen, um dessen Selbstwert zu stärken.",
    },
    Zusammenfassung {
        datei: "Einige_Gedanken_zur_Zusammenarbeit_zwischen_15.04.1988.pdf",
        titel: "Einige Gedanken zur Zusammenarbeit zwischen Gemeindesozialwesen und SPD",
        text: "Stichworte von U. Davatz für eine Arbeitstagung der Gemeindeschreiber des Bezirks Brugg (1988). Sie stellt die Aufgaben des Sozialpsychiatrischen Dienstes (SPD) vor: Nachbetreuung von Langzeitpatienten in Nacht- und Tagesklinik und Ambulatorium sowie Prävention durch Fallsupervision für Gemeindeschwestern, Suchtberater, Mütterberaterinnen und Hausärzte. Da Hilfesuchende meist zuerst zum Gemeindesozialarbeiter kommen, könnte eine engere Zusammenarbeit Schwellenangst abbauen, Krisen früher auffangen und teure Langzeithospitalisationen verhindern. Als Formen nennt sie Sprechstunden des SPD in der Gemeinde, Supervisionsgruppen und die Teilnahme an Sitzungen der Sozialverbände.",
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
        datei: "Eltern_sind_gleichzeitig_Führer_und_Partner_ihrer_Kinder_wie_16.03.1988.pdf",
        titel: "Eltern sind gleichzeitig Führer und Partner ihrer Kinder – wie gehe ich mit dem Spannungsfeld zwischen Autorität und Toleranz um?",
        text: "Vortrag von U. Davatz vor Mütterberatung und Pro Juventute (1988). Eltern haben immer beide Rollen: Als Autorität führen, lehren, setzen Grenzen und Strukturen, sind Vorbild und übernehmen Funktionen, die das Kind noch nicht erfüllen kann, aber flexibel und auf das Kind ausgerichtet. Spontanes Verhalten wie Schlafen, Essen, Fröhlichsein oder keine Angst haben lässt sich nicht befehlen, nur durch Struktur und Atmosphäre ermöglichen. Als Partner sind Eltern Spiel-, Gesprächs-, Kampf- und Entscheidungspartner. Zu viel Autorität macht unselbständig, zu viel Partnerschaft überfordert; je kleiner das Kind, desto mehr Führung, je älter, desto mehr Partnerschaft, und die beiden Rollen sollen klar getrennt bleiben.",
    },
    Zusammenfassung {
        datei: "Elternberatung_von_Schizophrenen_25.11.1996.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I (25.11.1996) mit elf Merkmalen von Familien mit einem schizophrenen Mitglied und je einer Faustregel für die Beratung. Genannt werden ein hoher Angstpegel, Schuldgefühle der Eltern, ein Aktivismus, der Angst bindet, ein latenter, oft geleugneter Ehekonflikt mit gegenseitiger Entwertung, sprunghaftes («overinclusive») Denken, Konfliktscheu, starke Fokussierung auf den Patienten, ungelöste Probleme mit der eigenen Herkunftsfamilie und Überbehütung. Der Therapeut soll Ruhe ausstrahlen, Schuldgefühle ansprechen, den Konflikt wahrnehmen, aber nicht dramatisieren, die Eltern einzeln stärken statt Einigkeit zu erzwingen, innerlich klar strukturiert bleiben und Probleme neutral benennen («better mad than bad»). Er hilft zu defokussieren, gibt eine gewisse Schutzgarantie und braucht viel Geduld; Angehörigengruppen gelten als hilfreich.",
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
        datei: "Emotionelle_Hintergründe_zum_Lernprozess_05.03.1986.pdf",
        titel: "Emotionelle Hintergründe zum Lernprozess – Bezugnahme auf die Situation der Krankenpflege",
        text: "Stichworte von U. Davatz zu einem Vortrag an einer Krankenpflegeschule (1986). Krankheit ist lernbar: Sie entwickelt sich in überfordernden Beziehungen, übernimmt als Symptom die Funktion eines Hilferufs, und chronischer Stress macht krank. Mitleid oder Ablehnung der Pflegenden verstärken über den Reizzustand die Krankheit und das Lernen des Symptoms, während eine beruhigende Haltung und die eigene Gesundheit der Schwester die Heilung und die Mobilisierung gesunder Kräfte fördern. Für junge Lernende zwischen 18 und 25 Jahren warnt sie vor Dominanzkämpfen mit Lehrern, in denen es nur noch um Siegen geht; Lehrer sollen sich zurücknehmen und Lerninhalte offen von verschiedenen Seiten diskutieren.",
    },
    Zusammenfassung {
        datei: "Emotioneller_Prozess_in_der_Kernfamilie_Möglichkeiten_zur_02.02.1987.pdf",
        titel: "Emotioneller Prozess in der Kernfamilie – Möglichkeiten zur Stressabsorption oder Adaptionsmechanismen einer Familie auf Überlastungssituationen",
        text: "Eine Familie wird als lebendiger Organismus beschrieben, der auf Überlastung mit Anpassungsmechanismen reagiert; diese sind zunächst nichts Krankhaftes und werden erst pathogen, wenn sie lange andauern, ohne die Ursache zu beheben (Bild des «run away feedback loop», aufgehobene Monokausalität). Davatz unterscheidet drei Mechanismen einer Kernfamilie unter Stress: den Ehekonflikt als Dominanzkampf, das Krankheitssymptom bei einem Ehepartner (körperlich, psychisch oder sozial, meist beim Unterlegenen oder Überfunktionierenden) und die Weitergabe der elterlichen Stressreaktion an ein Kind, bei dem dann ein Symptom ausbricht. Welcher Mechanismus gewählt wird, hängt weniger von der Belastung als von der «Programmierung» durch die eigene Erziehung ab. Skript für den Familientherapie-Kurs 1987.",
    },
    Zusammenfassung {
        datei: "Empowerment_von_Schmerzpatienten_27.08.2001.pdf",
        titel: "«Empowerment» von Schmerzpatienten",
        text: "Vortrag von U. Davatz im Familienkurs I vom 27.8.2001 über psychosomatischen Schmerz. Das Nervensystem passt den Organismus an sein Umfeld an; körperlicher Schmerz meldet eine Verletzung oder Überforderung eines Organs, psychischer Schmerz eine seelische Überforderung wie Kränkung, Schuld, Trauer oder Angst. Läuft die elterliche Erziehung dem Temperament des Kindes zu sehr entgegen, lernt es, sich zu verleugnen; diese Fehlanpassung kann im Erwachsenenalter wieder Schmerz auslösen, der in den Körper wechselt, je mehr psychischer Schmerz in der Erziehung als unannehmbar galt. Weil der Patient lieber sich selbst schädigt, als den Eltern untreu zu werden, braucht er Stärke und Selbstbewusstsein («Empowerment»), um den Schmerz durch verbale Kommunikation zu ersetzen. Psychosomatische Schmerzen sollen deshalb nicht mit Schmerzmitteln unterdrückt, sondern in Sprache übersetzt werden.",
    },
    Zusammenfassung {
        datei: "Epigenetisches_Potential_in_der_Psychiatrie_09.11.2026.pdf",
        titel: "Epigenetisches Potential in der Psychiatrie – Gravierende Mängel im Denkmodell und Versorgungskonzept der Psychiatrie",
        text: "Ausführlicher Aufsatz von U. Davatz (ohne Datum, mit Literatur bis 2018), ausgehend von genetischen Kohortenstudien, nach denen Schizophrenie, Depression, Autismus, bipolare Störung, Essstörungen und ADS/ADHS dieselben Gen-Loci teilen. Sie deutet ADS und ADHS als angeborene Neurotypen, einen introvertierten, scheuen und verträumten und einen extravertierten, impulsiven, und beschreibt, wie das plastische Gehirn sich in der Interaktion mit dem Beziehungsumfeld epigenetisch formt. Gerät diese Interaktion in der Erziehung aus dem Gleichgewicht, entstehen aus den Neurotypen über archaische Stressreaktionen psychische Krankheiten wie Autismus, Schizophrenie, Depression oder Sucht; dazu verweist sie auf die «intense world»-Theorie und die Verbindung von Kreativität und psychischer Krankheit, etwa am Beispiel von Eduard Einstein, und auf erfolgreiche Lebensläufe von Menschen mit diesen Eigenschaften. Das bio-psycho-soziale Modell der Psychiatrie bleibt ihrer Ansicht nach symptomorientiert und lässt das Umfeld ungenutzt. Sie fordert, das epigenetische Potential über die Beratung von Familien und Schulen für Prävention und Versorgung zu nutzen.",
    },
    Zusammenfassung {
        datei: "Epigenetisches_Potential_in_der_Psychiatrie_09.11.2026_2.pdf",
        titel: "Epigenetisches Potential in der Psychiatrie – Herleitung psychischer Krankheiten aus ADS und ADHS",
        text: "Andere, gestraffte Fassung des Aufsatzes von U. Davatz (ohne Datum, mit Literatur bis 2018), hier mit dem Untertitel «Herleitung psychischer Krankheiten aus ADS und ADHS». Ausgangspunkt sind die gemeinsamen Gen-Loci verschiedener psychiatrischer Diagnosen, die sie über die angeborenen Neurotypen ADS und ADHS erklärt: den introvertierten, der unter Stress nach innen flieht, und den extravertierten, der Normen ablehnt und eigene aufstellt. Beide sind hochsensibel; ob daraus gesunde Persönlichkeiten oder psychiatrische Folgekrankheiten entstehen, entscheidet vor allem das erzieherische Beziehungsumfeld. Sie verweist auf Henry Markrams Forschung zu reizverstärkenden Hirnzellen beim Autismus, auf den Zusammenhang von Kreativität und psychischer Krankheit am Beispiel von Eduard Einstein und auf erfolgreiche Lebensläufe wie den des Alibaba-Gründers. Die Forschung sollte deshalb das Beziehungsumfeld und die Interaktion mit diesen Kindern untersuchen.",
    },
    Zusammenfassung {
        datei: "Erkennen_von_Triangulierung_und_wie_kommt_man_wieder_heraus_16.12.2002.pdf",
        titel: "",
        text: "Handout von U. Davatz für den Familienkurs I vom 16.12.2002 darüber, wie Therapeuten in Familienkonflikte hineingezogen werden. Jedes Familienmitglied hofft insgeheim auf Unterstützung, weshalb Boszormenyi-Nagy die «Allparteilichkeit» geprägt hat; je nach Rolle, Institution und Geschlecht neigt man dazu, sich auf die Seite des Schwächeren, des Stärkeren, der Frauen oder des Kindes zu schlagen. Zeichen einer Triangulierung sind extreme Gefühle gegenüber einem Mitglied, das Übernehmen seiner Sichtweise, Distanzlosigkeit, Träume, viel Überzeugungsarbeit, die Richterrolle zwischen «Täter» und «Opfer», eigene Symptome und geheime Telefonanrufe. Heraus kommt man, indem man sich zurücklehnt, den Prozess statt der Personen betrachtet, ihn verlangsamt und den Parteien zurückgibt, zirkulär fragt, Geheimnisse diplomatisch weitergibt und die eigene Prägung hinterfragt. Die Hierarchie im System soll man erkennen und anfangs eher stützen, um die Lage von dort aus langsam zu verändern.",
    },
    Zusammenfassung {
        datei: "Ethnisch_gemischte_Familie_28.08.2000.pdf",
        titel: "",
        text: "Das Skript von U. Davatz (Familientherapie-Kurs I, 2000) behandelt ethnisch gemischte Familien. Die ethnische Zugehörigkeit prägt die Vorstellung von der Normfamilie. Bei Mischehen kommt es in kritischen Situationen zu Territorial- und Machtkämpfen der Kulturen, Religionen und Gebräuche. Zu beachten sind der Cut-off von der Ursprungsfamilie, der in einer Mischheirat steckt, die Geschlechterrollen der jeweiligen Ethnie im Ehekonflikt und die Kindererziehung, meist nach der Ethnie der Mutter. Kinder leben in zwei Kulturen und können in Loyalitätskonflikte geraten; der Therapeut sollte sich die Kulturen erklären lassen.",
    },
    Zusammenfassung {
        datei: "Familie_Bowen_09.11.2026.pdf",
        titel: "",
        text: "Genogramm der Herkunftsfamilie von Murray Bowen über vier Generationen, als Kursunterlage im Querformat. Es zeigt die Urgrosseltern als Gründer eines Warenhauses, den Vater, der nach dem Tod des Schwiegervaters Teilhaber wurde, und die verantwortungsvolle Mutter, die für Kinder, Haushalt und Geschäft sorgte. Murray Bowen ist das älteste von fünf Kindern; alle Söhne halfen im Warenhaus mit, ein Bruder wurde «Kopf des Klans», die jüngste Schwester die «starke Frau ihrer Generation». Als Indexpatientin ist eine Schwester markiert, die Wunschtochter und Hilfe der Mutter, als einzige nicht im College und wenig lebenstüchtig.",
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
        datei: "Familientherapie_bei_Schizophrenie-Kranken_09.11.2026.pdf",
        titel: "",
        text: "Skript von U. Davatz ohne Datum, das ihre rund dreissigjährige Erfahrung seit 1980 zusammenfasst, also um 2010 entstanden. Schizophrenie gilt ihr als vielstufige Prozesskrankheit eines sozialen Organs, die nur unter Einbezug der Familie behandelt werden kann; sie skizziert die Geschichte von Bowens Forschung am NIMH, der Theorie der «High Expressed Emotions» von Leff und Vaughn und der Psychoedukation. Von Bowens Zentrum kam sie 1980 nach Königsfelden mit der Idee der Früherfassung, gründete 1982 die VASK im Aargau und leitet seither eine Angehörigengruppe. Schizophrenie sieht sie als «maligne Pubertät» eines funktionalisierten Kindes, das in einen latenten Elternkonflikt eingebunden und symbiotisch an die Mutter gebunden ist. Der Familientherapeut soll den Partnerkonflikt klären, dem Vater die Führung in der Auseinandersetzung geben, die Mutter in ihrer Eigenständigkeit stützen und den Patienten zur Autonomie coachen, bis er auch seine Medikamente selbst handhabt.",
    },
    Zusammenfassung {
        datei: "Familientherapie_nach_Bowen_mit_einer_Person_im_02.03.2007.pdf",
        titel: "Familientherapie nach Bowen mit einer Person im sozial-psychiatrischen Setting",
        text: "Stichwortartige Unterlage für eine Weiterbildung zur Familientherapie nach Bowen mit einer Einzelperson im sozialpsychiatrischen Setting. Ausgangspunkt: psychiatrische Krankheiten sind funktionelle Hirnstörungen und Ausdruck gestörter Beziehungen, beeinflussbar durch Medikamente, durch Psychoanalyse oder durch Familientherapie. Der Einzelne wird im Familienkontext gesehen und mit dem Auftrag in die Familie geschickt, Muster zu beobachten und kleine Verhaltensänderungen zu erproben, was das ganze System verändere. Genannte Bowen-Konzepte sind Fusion, Cut-off, Triangle, Mehrgenerationenperspektive, Projektion und Differenzierung. Bei psychotischen, Borderline- und Suchtpatienten werden eher die Eltern beraten, bei Ehetherapie wird an der Entwicklung der Partner in Bezug auf ihre Herkunftsfamilie gearbeitet.",
    },
    Zusammenfassung {
        datei: "Flexibilisierung_der_Vaterrolle_10.2011.pdf",
        titel: "",
        text: "Persönlicher Bericht eines Kursteilnehmers, eines Assistenzarztes, vom Oktober 2011 (Name pseudonymisiert), den die Geburt seiner Tochter und ein autoritärer Chef dazu brachten, die Beziehung zu seinem Vater zu untersuchen. Er schildert den Vater als distanziert, rechthaberisch, ungeduldig und zwanghaft ordentlich, aber auch kreativ, und erzählt dessen Jugend im Zweiten Weltkrieg in Italien, mit Bombenangriffen und einer Gefangennahme durch deutsche Besatzer. Das Verständnis für diese prägenden Erlebnisse hilft ihm, das Misstrauen des Vaters gegenüber Autoritäten einzuordnen und eigene Muster wie den vorwurfsvollen Ton und das ständige Beschäftigtsein zu erkennen. Als Vater will er Zeit mit den Kindern verbringen, partnerschaftlich kommunizieren, durch Vorbild erziehen und Flexibilität statt Perfektion anstreben.",
    },
    Zusammenfassung {
        datei: "Frauen_in_der_Lebensmitte_02.11.1987.pdf",
        titel: "",
        text: "Vortrag von U. Davatz vor einem Frauenverein (1987). Während Männer in der Lebensmitte auf dem Höhepunkt ihrer Karriere stehen, geraten Frauen oft in eine Krise, weil ihre typische Rolle als Tochter, Partnerin und Mutter ganz auf andere ausgerichtet war. Mit dem Ende der Fruchtbarkeit, dem Auszug der Kinder, schwindender Schönheit und einer vernachlässigten Partnerschaft fallen ihre Aufgaben weg, und sie fühlt sich überflüssig und isoliert. Als Ausweg empfiehlt sie, die Erfahrung aus Familie und Haushalt in eine neue Aufgabe in Beruf, Politik oder Sozialwesen einzubringen, Zeit für sich selbst zu nutzen, Praxis in Theorie umzusetzen und die Partnerschaft auf einer neuen Ebene zu pflegen.",
    },
    Zusammenfassung {
        datei: "Früherfassung_von_Psychosen-Schizophrenie_in_der_09.11.2026.pdf",
        titel: "Früherfassung von Psychosen/Schizophrenie in der hausärztlichen Praxis",
        text: "Skript von U. Davatz ohne Datum, das sich an Hausärzte richtet und zur Früherfassung der Schizophrenie aufruft, deren Vorlaufphase im Schnitt fünf Jahre dauert; aus Angst vor Stigmatisierung keine Diagnose zu stellen, nennt sie einen Kunstfehler. Die präpsychotische Phase zeigt sich durch eine rigide, zwanghafte Abwehr, mit der der Patient jede Hilfe ablehnt; das Risiko steigt bei Adoleszenten mit Drogenkonsum wie Haschisch, LSD, Ecstasy oder Amphetaminen, einem frühkindlichen POS, emotionalen Belastungen in Familie, Schule oder Lehre und chronischem Elternkonflikt. Der Arzt soll die Führung übernehmen, Eltern und Lehrmeister getrennt instruieren, dem Patienten väterlich-bestimmte Anweisungen ohne Diskussion geben und Neuroleptika niedrig dosiert als Nachtmedikation verschreiben. Bei Erstpsychosen im Erwachsenenalter steht meist ein verdeckter Konflikt in Ehe oder Beruf dahinter; langfristig sollen Patient und Familie systemisch begleitet werden.",
    },
    Zusammenfassung {
        datei: "Fusionsbeziehung_oder_Symbiose_02.2018.pdf",
        titel: "",
        text: "Überarbeitete Fassung des Skripts «Fusionsbeziehung oder Symbiose» von 1998, von U. Davatz für den Familientherapie-Kurs I 2018 (Februar 2018) neu gesetzt; der Inhalt ist derselbe. Bowen übertrug den Begriff der Fusion von der symbiotischen Mutter-Kind-Beziehung auf Familie und Kollektive; sie sichert Zusammenhalt und Überleben, wird aber über Angst und schlechtes Gewissen aufrechterhalten, und Angst und Fusion verstärken sich gegenseitig. Eine ängstliche Mutter kann so die Ablösung behindern und Krankheiten wie Schizophrenie, Sucht oder psychosomatische Leiden begünstigen. Therapeutisch soll man nie gegen die Fusion kämpfen, sondern zuerst die Angst senken, die Fusion positiv benennen, geduldig sein und selbst eine eigene Position halten.",
    },
    Zusammenfassung {
        datei: "Fusionsbeziehung_oder_Symbiose_09.11.1998.pdf",
        titel: "",
        text: "Bowen hat den Begriff der Fusion in Anlehnung an die symbiotische Mutter-Kind-Beziehung (Mahler, A. Freud) geprägt und auf die ganze Familie und andere Kollektive ausgedehnt, so das Skript von U. Davatz für den Familientherapie-Kurs I 1998. Die Fusion sichert Zusammenhalt und Überleben, wird aber über Angst und später über das schlechte Gewissen aufrechterhalten: Fusion und Angst verstärken sich gegenseitig in einer positiven Rückkopplung. Eine ängstliche Mutter kann so die Ablösung des Kindes behindern, was zu Schizophrenie, Sucht oder psychosomatischen Krankheiten führen kann, die wiederum die Fusion rechtfertigen. In der Fusion findet alles soziale Lernen statt, eng fusionierte Systeme sind aber störanfälliger und lassen kaum Autonomie zu. Therapeutisch soll man nie gegen die Fusion ankämpfen, sondern zuerst die Angst senken, die Fusion positiv benennen, geduldig bleiben und selbst eine eigene Position halten, ohne die Beziehung abzubrechen.",
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
        datei: "Genuss_ohne_Sucht_Motivation_zum_Leben_als_Prävention_13.11.1986.pdf",
        titel: "Genuss ohne Sucht – Motivation zum Leben als Prävention",
        text: "Vortrag von U. Davatz an der Tagung «Suchtverhalten in der Wohlstandsgesellschaft» des Gottlieb Duttweiler Instituts (1986), mit Titelblatt und Inhaltsverzeichnis des Tagungsbands. Prävention versteht sie nicht als Kampf gegen Laster oder Suchtmittel, sondern als Bejahung des Lebens. Aus der Werbung liest sie zwei Verkaufsstrategien ab – Suchtmittel als Belohnung nach Stress und als Verstärker positiver Gefühle – und zeigt, dass Suchtmittel in Beziehungen Distanz schaffen, von Mutter und Kind bis zu Arzt und Patient. Sie befriedigen die Bedürfnisse nach Anerkennung und Abgrenzung, entfernen aber von deren natürlicher Befriedigung. Für Mütter, Lehrer, Ärzte, Paare und Bürger folgert sie: mehr loben, sich abgrenzen, Gefühle zulassen, Vielfalt und Spiel fördern und weniger auf Chemie setzen.",
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
        datei: "Geschwisterliebe_Geschwisterstreit_09.11.2012.pdf",
        titel: "Geschwisterliebe – Geschwisterstreit",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2012 über die Geschwisterbeziehung als längste Beziehung im Leben. Die Angst der Eltern vor Eifersucht führt oft dazu, das ältere Kind vom Baby fernzuhalten, was gerade erst Aggression auslöst; da Geschwister nach dem Prinzip der Kin-Selection auf gegenseitige Unterstützung angelegt sind, sollen Eltern dem Sozialinstinkt vertrauen und das ältere Kind sorgfältig in den Umgang mit dem jüngeren einführen. Natürlicher Geschwisterstreit ist ein soziales Lernfeld für Selbstbehauptung und Konfliktlösung, das ausgeglichene Eltern mit Regeln begleiten können. Gestört wird er, wenn die elterlichen Ressourcen nicht reichen: durch zu schnelles Unterdrücken, die Suche nach Täter und Opfer, einen Sündenbock, ein Kind mit Handicap oder das Einspannen eines Kindes in die eigenen Probleme, was bis zum Erbstreit über Generationen führen kann. Bei bösartigem Streit sollen Eltern zuerst ihre eigenen ungelösten Probleme direkt angehen.",
    },
    Zusammenfassung {
        datei: "Geschwisterliebe_Geschwisterstreit_21.10.2013.pdf",
        titel: "Geschwisterliebe – Geschwisterstreit",
        text: "Fassung vom 21.10.2013 des Skripts von U. Davatz für den Familientherapie-Kurs I; der Text entspricht fast wörtlich der Fassung von 2012. Die Angst der Eltern vor Eifersucht lässt sie das ältere Kind vom Baby fernhalten, was erst Aggression auslöst; sie sollen dem angeborenen Sozialinstinkt der Geschwister vertrauen und das ältere Kind behutsam einführen. Natürlicher Geschwisterstreit ist soziales Lernen, gestört wird er, wenn die elterlichen Ressourcen nicht reichen: durch Unterdrücken, die Suche nach Täter und Opfer, Sündenböcke oder das Einspannen eines Kindes in die eigenen Probleme. Bei bösartigem Streit sollen Eltern zuerst ihre eigenen ungelösten Probleme angehen.",
    },
    Zusammenfassung {
        datei: "Geschwisterposition_29.10.2000.pdf",
        titel: "",
        text: "Die Position in der Geschwisterreihe prägt nach diesem Skript von U. Davatz (Familientherapie-Kurs I, 2000) die Persönlichkeit. Es stellt die acht Geschwisterpositionen des Psychologen Walter Toman vor und beschreibt Merkmale der Ältesten (Verantwortung, Hüter der Tradition), der Jüngsten (kompetitiv, kreativ, Regelbrecher) und der Mittleren («Sandwich»-Kinder mit flexibler Orientierung). Fällt ein Ältester aus seiner Rolle, kann ein mittleres Geschwister die Rolle des funktionellen Ältesten übernehmen. Therapeutisch sollen Älteste entlastet, Jüngste zur Verantwortung aufgefordert und Mittlere bei der Klärung ihrer Rolle unterstützt werden.",
    },
    Zusammenfassung {
        datei: "Geschwisterpositionen_ihre_Vor-_und_Nachteile_21.08.2016.pdf",
        titel: "Geschwisterpositionen – ihre Vor- und Nachteile",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2016 (21.8.2016) über die Geschwisterposition als wichtigen Umweltfaktor, der jedem Kind einen eigenen Platz in der Familie gibt. Älteste übernehmen gerne Führung und Verantwortung, lassen sich aber ungern entthronen; Jüngste sind ehrgeizig und kreativ, tun sich aber schwer mit Verantwortung; Mittlere sind flexibel, wissen aber oft nicht, wohin sie gehören («Sandwichposition»); Einzelkinder sind selbstsicher, können aber schlecht Konflikte austragen. Das Geschlecht der Geschwister, die Einstellung der Eltern zu Söhnen und Töchtern und die Dominanz unter den Eltern verschieben diese Rollen. Auch die Rivalität zwischen Eltern und Kindern sowie zwischen den Eltern selbst beeinflusst die Konstellation. Alle Positionen haben Vor- und Nachteile und sind durch starke Lebensumstände veränderbar.",
    },
    Zusammenfassung {
        datei: "Gesundheitsschwester_wo_ist_ihr_Platz_im_heutigen_18.03.1988.pdf",
        titel: "Gesundheitsschwester – wo ist ihr Platz im heutigen Gesundheitswesen?",
        text: "Stichworte von U. Davatz für einen Weiterbildungstag der Gesundheitsschwestern (1988). Negativ gesehen sei die Gesundheitsschwester Handlangerin der Hausärzte, Aushängeschild der Spitex-Politik, Dienerin aller Herren, Sozialpolizistin und Einzelkämpferin an der Front. Im Spital zur Hierarchie erzogen, falle es ihr schwer, nach oben neutral Rückmeldung zu geben; sie solle sich als freie Mitarbeiterin in funktionellen Beziehungen verstehen. Unterstützung finde sie in Supervision, bei Kolleginnen und durch Öffentlichkeitsarbeit, Wirkung durch projektbezogene statt starre Organisation. Eine grosse Chance sieht sie in der Prävention: als heimliche Familienberaterin an den kritischen Momenten der Familie.",
    },
    Zusammenfassung {
        datei: "Gewalt_im_Familiensystem_22.07.2013.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I vom 22.7.2013, ausgehend davon, dass die meisten Morde in Beziehungssystemen wie der Familie geschehen. Sie unterscheidet Aggression, eine spontane Stressreaktion auf eine Bedrohung von Partner, Territorium, Existenz oder Würde, von Gewalt, bei der jemand seine Machtstellung berechnend missbraucht, um Aggression auszuleben. In engen Familienbeziehungen kennt man die Schwächen des anderen und nutzt sie im Machtkampf aus, körperlich oder, häufiger bei Frauen, verbal durch Beschämung; Gewalt von Eltern gegen Kinder kann sich später wiederholen, Partnergewalt wurzelt oft in uneingestandener Abhängigkeit und legitimiert sich als «richtende» Gewalt, und Teenager oder Geschwister werden gewalttätig, wenn Eltern sie einengen oder ungerecht behandeln. Gewalt soll nicht vorab moralisch verurteilt, sondern über die dahinterliegenden Verletzungen verstanden werden, bevor eine gewaltfreie Lösung gesucht wird; blosse Unterdrückung erzeugt neue Gewalt, und die natürlichen Hierarchien und Grenzen in der Familie müssen hergestellt werden.",
    },
    Zusammenfassung {
        datei: "Gibt_es_das_auch_Depressionen_bei_geistig_behinderten_28.10.2026.pdf",
        titel: "Gibt es das auch? Depressionen bei geistig behinderten Jugendlichen und Erwachsenen",
        text: "Vortragstext von Ursula Davatz «Gibt es das auch? Depressionen bei geistig behinderten Jugendlichen und Erwachsenen» (Thema des Insieme-Vortrags vom 10.3.2003). Depression wird als «Verliererkrankheit» verstanden; geistig Behinderte erlebten sich nicht von sich aus als Verlierer, würden es aber durch Über- oder Unterforderung ihres Umfelds. Anhand des dreiteiligen Gehirns (Reptilien-, limbisches und kognitives Hirn) wird erklärt, dass ihr emotionales Gehirn fein wahrnimmt, sie nicht intellektualisieren können und ehrlich reagieren. Aufgezählt werden Umfeldfaktoren wie unverarbeitete Akzeptanz der Behinderung, Ehrgeiz, Bevormundung, Ungeduld, gereizte Stimmung und abruptes Unterbinden des Sexualverhaltens; geraten wird zu mehr Einfühlung und Aufmerksamkeit.",
    },
    Zusammenfassung {
        datei: "Herkunftsfamilie_und_Identitätsfindung_19.08.2013.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I vom 19.8.2013. Die soziale Vererbung prägt den Menschen stärker als die genetische, wobei die Persönlichkeit aus der Wechselwirkung von Anlage und Umfeld entsteht (Epigenetik). Eltern geben Werte und Erwartungen weiter und erteilen, oft unausgesprochen, Aufträge an die nächste Generation: Vollendungsaufträge bei unerfüllten Wünschen, Lösungs-, Verhinderungs-, Helfer-, Sühne- und Vermittleraufträge je nach ungelösten Problemen, schwarzen Schafen, kranken Eltern, erlittenem Unrecht oder Elternkonflikt. Welches Kind welchen Auftrag übernimmt, hängt oft von der Geschwisterposition ab, dazu kommen äussere Einflüsse wie Krieg, Migration oder Wirtschaftskrisen. Je mehr Funktionen ein Kind für die Familie übernehmen muss und je weniger es zu ihr passt, umso schwerer findet es seine eigene Identität; Schizophrene sieht sie als Kinder, die daran gehindert wurden, und die Familientherapie soll der Familie zu besseren Problemlösungen verhelfen.",
    },
    Zusammenfassung {
        datei: "Hilfe,_mein_Kind_wird_gemobbt_07.11.2002.pdf",
        titel: "",
        text: "Mobbing unter Kindern wird im Vortrag für den Elternzirkel Reinach als Gruppenmechanismus beschrieben: Eine gestresste Gruppe wählt einen Sündenbock, um sich von allem Dysfunktionalen zu «säubern». Gefährdet ist, wer aus der Gruppe herausragt, positiv wie negativ (Aussehen, Kleidung, Verhalten, Glauben); POS-Kinder seien durch ihr Störverhalten besonders exponiert. Eltern sollen weder zur Anpassung drängen noch das Kind als Opfer verteidigen, sondern verschiedene Sichtweisen aufzeigen und dabei emotional voll hinter dem Kind stehen. Kommt der Gruppenprozess aus dem Ruder, sollen sie sich ohne Schuldzuweisung an die verantwortliche Person wenden, denn Mobbing sei Zeichen schwacher Führung.",
    },
    Zusammenfassung {
        datei: "How_can_global_perspectives_be_taught_and_learned_Global_04.08.1987.pdf",
        titel: "How can global perspectives be taught and learned: Global versus territorial thinking",
        text: "Englischer Beitrag von U. Davatz für eine Weiterbildungstagung in Dalian, China (1987). Sie leitet das territoriale Denken aus dem Revierverhalten der Tiere ab: Man bleibt in seinem fachlichen, religiösen, ethnischen oder kulturellen Denkrevier und verteidigt es. Globales Denken verlangt dagegen, sich laufend aus dem eigenen Revier zu lösen und die verschiedenen Denkmodelle miteinander zu verbinden, ohne eines zu verallgemeinern; es ist nicht dasselbe wie universelles Denken. Gelernt wird es am ehesten in multidisziplinären Teams mit gemeinsamen Aufgaben und in einer dezentralen Bildung, deren Institutionen zusammenarbeiten statt nur zu konkurrieren. So entstehe eine Koevolution der Denkmodelle statt Revierkämpfe.",
    },
    Zusammenfassung {
        datei: "How_can_global_perspectives_be_taught_and_learned_Global_04.08.1987_2.pdf",
        titel: "How can global perspectives be taught and learned: Global versus territorial thinking (zweites Exemplar)",
        text: "Zweites Exemplar des englischen Beitrags von U. Davatz für die Weiterbildungstagung in Dalian, China (1987), mit anders gesetztem Kopf, handschriftlichem Datum und einer Randnotiz. Sie stellt dem territorialen Denken, das wie das Revierverhalten der Tiere das eigene fachliche, religiöse oder kulturelle Denkrevier verteidigt, das globale Denken gegenüber, das sich laufend aus dem eigenen Revier löst und die verschiedenen Denkmodelle verbindet. Gefördert werde es durch multidisziplinäre Teams mit gemeinsamen Aufgaben und eine dezentrale, kooperative Bildung.",
    },
    Zusammenfassung {
        datei: "Innere_Beheimatung_12.2010.pdf",
        titel: "",
        text: "Kurzer persönlicher Erfahrungsbericht eines Kursteilnehmers vom Dezember 2010 (Name pseudonymisiert). Er beschreibt, wie sich mit Motivation und wenigen gezielten Interventionen ein ganzes System zu verändern beginnt, in der Gruppe wie in seiner eigenen Auseinandersetzung mit seiner Entwurzelung und dem daraus entstandenen Leidensdruck. Das Genogramm erlebte er als hilfreiches Werkzeug, um sein Familiensystem zu verstehen und einen psychischen und körperlichen Lähmungszustand zu analysieren. So gelang es ihm, aus einem jahrelangen Teufelskreis auszubrechen und eine neue innere Beheimatung mit Raum für Zukunft und Kreativität zu finden.",
    },
    Zusammenfassung {
        datei: "Interview_am_Radio_DRS_anlässlich_der_Pro_Mente_Sana-Tagung_10.11.1985.pdf",
        titel: "Interview am Radio DRS anlässlich der Pro Mente Sana-Tagung 1985",
        text: "Abschrift eines Radiobeitrags von Radio DRS über die Jahrestagung der Stiftung Pro Mente Sana (1985) zur Frage, was von der Sozialpsychiatrie zu erwarten ist. Der Genfer Psychiater Gaston Garrone fordert, die psychiatrischen Kliniken müssten verschwinden; zwanzig Betten auf 100'000 Einwohner genügten, wenn Kranke in ihrem Umfeld Hilfe bekämen. Der Bericht stellt dem den Aargau gegenüber, wo der Sozialpsychiatrische Dienst unter der Leitung von U. Davatz mit 18 Stellen im Schatten der Klinik arbeitet und wegen eines Anstellungsstopps nur auf Kosten der Klinik wachsen könnte. Zur Sprache kommen auch die Angst der Bevölkerung vor Psychiatriepatienten in der Nachbarschaft und die Forderung, mehr für die Betroffenen zu tun.",
    },
    Zusammenfassung {
        datei: "Inzest_25.10.1999.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I/1999 (25.10.1999). Das Inzesttabu gilt in allen Kulturen und Religionen und schützt biologisch vor Inzucht, wurde aber etwa in Adelshäusern zur Erhaltung des Besitzes umgangen. Um therapeutisch helfen zu können, verlangt sie eine neutrale statt moralische Haltung: Inzest tritt auf, wenn eine Familie emotional an ihre Grenzen kommt, sich aber keine Hilfe von aussen holen darf; der Täter ist eine bedürftige Person, und das Opfer wird doppelt geschädigt, durch den Missbrauch und durch Schuldzuweisung statt Anerkennung. Sie plädiert für einen therapeutischen statt strafrechtlichen Umgang, für Anerkennung des Opfers, andere Wege zur Befriedigung der Bedürfnisse des Täters und für die Konfrontation der Bezugsperson, die das Kind hätte schützen sollen. Aufdeckung soll langsam, ohne Sensationslust und ohne Öffentlichkeit geschehen, weil verdrängter Inzest zu Psychose, Sucht oder psychosomatischen Krankheiten führen kann.",
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
        datei: "Jugend_und_Drogen_16.01.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Elternabend an einer Oberstufenschule (1986). Für Jugendliche bedeutet die Droge Ausbruch aus der engen Wirklichkeit, Flucht vor Problemen, Abenteuer, Zugehörigkeit, Nervenkitzel und Macht über ahnungslose Erwachsene; für Erwachsene ist sie der Grund allen Übels, die grösste Gefahr und Anlass zu Angst und Panik. In Wirklichkeit sei sie weder Paradies noch Hölle, sondern eine Nahtstelle und ein Symptom des Konflikts zwischen den Generationen. Gefährlich werde Drogenkonsum, wenn Jugendliche ihn als Problemlösung einsetzen oder Erwachsene in der Droge die Ursache aller Probleme sehen, denn beides verhindere echte Problemlösung und Wachstum.",
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
        datei: "Kindheitstraumatas_heilen_ablösen_21.07.2014.pdf",
        titel: "Kindheitstraumatas heilen / ablösen",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2014 vom 21.7.2014. Der Traumabegriff geht auf Freud zurück; traumatisch wirkt jedes Ereignis, das nicht verarbeitet werden kann, und Kinder sind dafür anfälliger, manchmal aber auch widerstandsfähiger als Erwachsene. Als Kindheitstraumata nennt sie unter anderem den Verlust einer Bezugsperson, dauernden Elternkonflikt, den Tod oder die schwere Krankheit eines Familienmitglieds, Existenzsorgen, Migration aus Kriegsgebieten, schlechte Passung zwischen Eltern und Kind, Ausgrenzung in der Schule und ungeschickten Umgang mit Lernstörungen oder ADHS. Bei Defiziten im Familiensystem übernehmen Kinder je nach Position und Temperament Funktionen für die Familie; diese Funktionalisierung behindert ihre Entwicklung und zeigt sich oft erst in der Ablösung während der Pubertät als Krankheit. Systemtherapeutisch wird die Funktionalisierung über das Genogramm verstanden und gewürdigt, bevor Verhaltensmuster verändert werden; geheilt wird über die Veränderung des Familiensystems, nicht beim Symptom.",
    },
    Zusammenfassung {
        datei: "Kommunikation_unter_Spannungssituationen_09.11.1994.pdf",
        titel: "",
        text: "Kommunikationsmuster sind situationsabhängig, auch wenn sich in Familien eine länger konstante Kommunikationskultur einspielt, so das Skript von U. Davatz für den Familientherapie-Kurs I 1994. Kommunikation wird als zielgerichtete Beeinflussung definiert, bei der Akteur und Reakteur oft nicht zu trennen sind, etwa wenn ein Ehepartner auf frühere Verletzungen durch die Mutter reagiert. Auf der Verhaltensebene unterscheidet sie Dominanz-, Wettkampf-, Verführungs-, Ausweich- und Abwehrkommunikation sowie Mischformen. Unter Spannung zeigen sich je nach Rolle typische Muster: der ängstliche Patriarch befiehlt, die ängstliche Mutter lockt ins Nest zurück, Ehepartner weichen aus oder wehren ab, oft über das Kind als Dreieck, und Kinder weichen aus oder lügen. Die in der Forschung gemessenen Muster (communication deviance, affective style, Triangulation, expressed emotions) gelten als Indikatoren für Stress in der Familie, der die Symptome verstärkt.",
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
        datei: "List_of_Participants_Visions_of_Higher_Education,_Dalian_03.08.1987.pdf",
        titel: "List of Participants: Visions of Higher Education – Trans-National Dialogues: Praxis and Prospects, Dalian 1987",
        text: "Teilnehmerliste der dritten internationalen Konferenz «Visions of Higher Education – Trans-National Dialogues: Praxis and Prospects», die das Gottlieb Duttweiler Institut vom 3. bis 7. August 1987 in Dalian, China, durchführte. Aufgeführt sind Hochschullehrer und Bildungsfachleute aus den USA, Europa und China mit Funktion und Institution, darunter U. Davatz als leitende Ärztin des Sozialpsychiatrischen Dienstes der Klinik Königsfelden. An dieser Tagung hielt sie ihren Beitrag über globales und territoriales Denken.",
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
        datei: "Machtkampf_in_der_pflegerischen_Beziehung_05.02.1987.pdf",
        titel: "",
        text: "Unterlage von U. Davatz für eine Fortbildung des Pflegeberufsverbands SBK (1987). Aus der Tierwelt leitet sie vier Grundmuster ab: Verteidigung, Flucht, kreatives Ausweichen und Dominanzverhalten, das um Futter, Positionen oder Territorien kämpft und mit Sieger und Verlierer endet. In der Pflege liegt die Macht der Helfer im abhängigen Patienten; der Kampf gegen die Krankheit wird zum Sieg des Helfers. Wehrt sich der Patient gegen seine Abhängigkeit, entsteht ein verbitterter Machtkampf mit Ablehnung, Fixierung, hohen Medikamentendosen oder Verboten. In Rollenspielen sollen Pflegende lernen, weniger auf den Machtkampf einzusteigen und Vertrauen entstehen zu lassen.",
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
        datei: "Migration,_Migrantenfamilien_und_Clanstrukturen_22.10.2012.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 22.10.2012. Der Mensch ist eine wandernde Spezies, die als Clan oder als Einzelner migriert, wenn die Umstände ungünstig werden. Bei Migrationsfamilien fragt sie stets, wovon weg und worauf zu gewandert wurde, wer Initiant und wer Mitwanderer war; neben wirtschaftlichen Motiven gibt es immer emotionale, etwa die Flucht vor einem Familienkonflikt oder einer schweren Rolle als schwarzes Schaf oder überforderter Ältester. Die Schweiz wird dabei oft als «Paradies» überhöht; enttäuschte Erwartungen äussern sich in Krankheitssymptomen und appellativem Jammerverhalten, das Helfer irritiert und rein somatisch behandelt wird. Erst das Erfragen der Wandermotive deckt das Missverhältnis zwischen Erwartung und Realität auf und wird dem Leiden gerecht; der Abschnitt zum therapeutischen Umgang ist in der Vorlage nur angekündigt.",
    },
    Zusammenfassung {
        datei: "Migration,_Migrantenfamilien_und_Clanstrukturen_22.10.2012_2.pdf",
        titel: "",
        text: "Vollständige Fassung des Skripts von U. Davatz vom 22.10.2012, in der der therapeutische Teil ausgeführt ist und ein Teil über Clanstrukturen dazukommt. Wie in der ersten Fassung fragt sie nach Wandermotiven und enttäuschten Erwartungen an das Auffangland, die sich als Krankheitssymptome und Jammerverhalten zeigen. Therapeutisch soll man Migranten in ihrem Wertsystem begegnen und über etwas aus ihrem Land ein «Joining» herstellen, bevor man die Grenzen der Medizin im Auffangland aufzeigt. In Clanfamilien begrüsst man zuerst das Familienoberhaupt, achtet die Hierarchie, zieht bei patriarchalen Systemen eine nicht überhebliche männliche Autoritätsperson bei und tritt weder zu dominant noch zu schwach auf. Krank wird oft das Mitglied, das sich vom Clan lösen möchte, aber nicht darf; goldene Regel ist, der Clanstruktur mit Respekt zu begegnen und sie nicht sofort umbauen zu wollen.",
    },
    Zusammenfassung {
        datei: "Migrationsprobleme_in_der_Psychiatrie_15.05.1996.pdf",
        titel: "",
        text: "Vortrag von U. Davatz vom 15.5.1996 in der Supervision am Psychiatrischen Stützpunkt Biel. Sie spricht bewusst nicht von Ethnopsychiatrie, sondern von Wanderproblematik und Anpassung: Der Mensch ist ein Wanderer, der aufbricht, wenn die Lebensbedingungen ungünstig werden, früher in ganzen Völkern, heute einzeln oder in Teilfamilien; manche nutzen das Umziehen als Bewältigungsstrategie, und der Cut-off nach Bowen gilt als emotionale Migration innerhalb der Familie. Psychologisch fragt sie, wovon weg und worauf zu gewandert wurde: Wer die Heimat verherrlicht, kann sich nicht anpassen, wer das neue Land idealisiert, wird enttäuscht und ist nirgends mehr zuhause; therapeutisch ist das jeweils zu relativieren. Weiter unterscheidet sie aktive Wanderer von Mitwanderern, deren «Seele nachwandern» muss, Beutezüge von echten Auswanderungen mit der Frage nach Rückkehr oder Integration und die Wanderhaltung der verschiedenen Generationen.",
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
        datei: "Neue_psychosoziale_Erkrankungen_bei_Kindern_und_Jugendlichen_10.11.1985.pdf",
        titel: "",
        text: "Abschrift eines Vortrags von U. Davatz in Heidelberg (1985) über den Einfluss des Schulsystems auf neue psychosoziale Erkrankungen von Kindern und Jugendlichen. Sie setzt einem eindimensionalen Krankheitsdenken ein evolutives, interaktives Verständnis entgegen: Kinder wachsen von selbst, wenn man ihnen Freiraum und Nahrung gibt, und medizinische Diagnosen können durch Etikettierung selbst krank machen. Das Recht auf gleiche Bildung werde zum Zwang zu gleicher Leistung, und Teilleistungsstörungen in Motorik oder Lernen würden durch Sonderbehandlung und Druck zu Verhaltensstörungen. An anonymisierten Fallbeispielen zeigt sie, wie Schule, Familie und Behörden einen Teufelskreis von Frustration, Rückzug und Ausschluss in Gang setzen, und plädiert dafür, Kinder weniger bestrafend und mehr nach ihrem Hintergrund zu verstehen.",
    },
    Zusammenfassung {
        datei: "Normierungswahn_und_Perfektionismus_in_unserer_Gesellschaft_20.10.2014.pdf",
        titel: "Normierungswahn und Perfektionismus in unserer Gesellschaft – Auswirkungen auf das Familien-System",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2014 vom 20.10.2014. Normierung und Perfektionismus stehen eher am Ende einer kulturellen Entwicklung und zeigen einen erhöhten Angstpegel im Kollektiv, denn Zwangshandlungen binden Angst; eine zu stark normierte, homogene Gesellschaft verliert ihr kreatives Element und wird dekadent. Der Druck trifft besonders Familien, die von der Norm abweichen, etwa kreative Familien oder solche mit ADHS; können Eltern ihre Kinder nicht davor schützen, werden diese in psychische Krankheiten abgedrängt, und auch die Psychiatrie erfindet unter Normierungsdruck neue Diagnosen für Jugendliche in der Persönlichkeitsbildung. Therapeuten sollen den Normierungszwang einer Familie erkennen und den Kindern Raum und Zeit für mehr Eigenständigkeit geben, als «Geburtshelfer» der eigenen Persönlichkeit. Dazu müssen Eltern oft ihre eigene Pubertät und unvollständige Ablösung anschauen, denn das Leben ist ein Seiltanz zwischen Anpassung und Eigensinn.",
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
        datei: "Patchwork_Familie_und_die_Rolle_von_Tieren_im_Familiensystem_22.09.2014.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2014 vom 22.9.2014, eine spätere Fassung ihres Themas Patchworkfamilie, erweitert um die Rolle von Haustieren. In Patchworkfamilien müssen Partner- und Elternrolle sauber getrennt werden: Der leibliche Elternteil trägt die Verantwortung für wichtige Entscheidungen allein und darf höchstens den anderen leiblichen Elternteil zuziehen, nie den neuen Partner, sonst entsteht ein Dreieckskonflikt. Stiefmütter, die schnell die Mutterrolle übernehmen wollen, und Stiefväter, die als Autorität Ordnung schaffen wollen, stossen auf Ablehnung; sie dürfen als Erwachsene ihre Werte und Hausregeln vertreten, aber nicht erziehen, und Kinder müssen den neuen Partner nicht lieben, nur höflich sein. Haustiere können Gefühle tragen, die man Partner oder Kindern nicht zeigt, stehen aber zuunterst in der Hackordnung und zeigen Familienkonflikte oft früh an.",
    },
    Zusammenfassung {
        datei: "Patchworkfamilie_09.11.2000.pdf",
        titel: "",
        text: "Patchworkfamilien sind aus mehreren Familien zusammengesetzt, konfliktanfälliger und verlangen einen stärker individualisierten Erziehungsstil, so das Skript von U. Davatz für den Familientherapie-Kurs I/2000. Die emotionale Verantwortung für die Erziehung trägt der leibliche Elternteil; der Stiefelternteil soll erst eine Beziehung zum Kind aufbauen und nicht einspringen, wenn sein Partner überfordert ist, sonst endet es im Partnerkonflikt. Unter Stiefgeschwistern soll man Unterschiede in den Bindungsgefühlen zugeben statt sie zu leugnen. Ab der Pubertät soll eine Begegnung mit dem abwesenden Elternteil behutsam ermöglicht werden, ohne das Kind dazu zu drängen oder den Weg durch abwertende Aussagen zu verbauen. Der Therapeut hilft der Familie, allmählich klare Strukturen zu finden, ohne sie für anfängliche Turbulenzen zu verurteilen.",
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
        datei: "Prozess-Management_versus_Case-Management_in_der_Medizin_und_17.10.2016.pdf",
        titel: "Prozess-Management versus Case-Management in der Medizin und insbesondere bei der Behandlung von Schizophreniepatienten",
        text: "Skript von U. Davatz für den Familientheorie-Kurs I vom 17.10.2016. Ausgehend von der Biologie, in der Zellen und Organe ohne übergeordneten Manager zusammenwirken, kritisiert sie das «männlich dominierte» Denken der Medizin, aus dem das von Krankenkassen geförderte Case-Management entstanden ist: eine direktive, auf Kosteneffizienz statt Heilung ausgerichtete Verwaltung chronisch Kranker. Schizophreniepatienten sieht sie als Menschen, die in einer «malignen Pubertät» steckengeblieben sind und sich gegen jedes Managen mit Widerstand wehren, was dann als mangelnde Compliance gilt. Der Therapeut soll stattdessen ein standhaftes Gegenüber auf gleicher Augenhöhe sein, nichts wollen, aber klar Stellung beziehen, damit der Patient seine Ablösung nachholen kann; zugleich brauchen die Eltern Begleitung beim Loslassen, ohne Befehle. Als Prozessmanager hält der fallführende Therapeut die Fäden in der Hand, kommuniziert offen mit anderen Fachleuten und fördert den Entwicklungsprozess, statt ihn zu kontrollieren.",
    },
    Zusammenfassung {
        datei: "Prävention_im_Kanton_Aargau_21.06.1989.pdf",
        titel: "",
        text: "Grundsatzpapier von U. Davatz (1989) zur Prävention im Aargau, ausgelöst durch die Zunahme verhaltensauffälliger und suchtgefährdeter Jugendlicher. Prävention heisst für sie Früherfassung durch Eltern, Kindergärtnerinnen, Lehrer und Lehrmeister, denen eine leicht erreichbare Fachberatung zur Seite stehen soll. Wichtiger als reine Suchtprophylaxe ist eine Erziehung zur aktiven Konfliktbewältigung, die auch psychische und körperliche Krankheiten verhindert. Präventivarbeiter sollen als neutrale Krisenhelfer bei Problemen in Schule, Lehrerkollegium und Familie beigezogen werden, gestützt auf den Drogenbericht der Eidgenössischen Betäubungsmittelkommission. Am Schluss stehen die regionalen Jugend- und Drogenberatungsstellen.",
    },
    Zusammenfassung {
        datei: "Prävention_sinnvoller_Ansatz_bei_gefährdeten_Familien_mit_03.07.1989.pdf",
        titel: "Prävention – sinnvoller Ansatz bei gefährdeten Familien mit jungen Kindern",
        text: "Kursarbeit von E. Maarse Bäbler zur Prävention bei gefährdeten Familien mit kleinen Kindern, eingeleitet mit einem Märchen über die Grenzen perfekter Erziehungstheorien. Als Belastungsfaktoren nennt sie Schicksalsschläge wie Tod, Trennung oder ein behindertes Kind, die unverarbeitete eigene Kindheit, die Isolation der Kleinfamilie bei abwesenden Vätern und geringer Wertschätzung der Erziehungsarbeit, beengte Wohnverhältnisse, Geldprobleme und schichtspezifische Belastungen. Sie schlägt unter anderem Kontaktgruppen für Familien, kinderfreundliche Wohnumgebungen, Beteiligung der Väter an der Erziehung und die Möglichkeit für Mütter vor, neben dem Kind eigene Interessen zu behalten. Familientheoriekurs II, 1989.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_Alltag_25.11.1985.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Frauenverein (1985). In Medien und Öffentlichkeit diene die Psychiatrie dazu, Angst und Sensation zu erzeugen, Gesellschaftskritik zu üben oder sich politisch mit dem Dienst am Ärmsten zu profilieren. In ihrem eigenen Alltag gehe es um Lebensgeschichten und Tragödien, um die Begleitung in kritischen Augenblicken und die Unterstützung von Familie, Arbeitgebern und Nachbarn. Psychiatriepatienten lösen Angst und Distanz, überfordernden Helferwillen, unehrliche Schonung, Ohnmacht oder Aggression aus. Sinnvoll sei es, sich selbst abzugrenzen und doch Beziehung anzubieten, den Patienten so normal wie möglich zu behandeln, klar Stellung zu beziehen und eigene Emotionen zurückzuhalten.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_täglichen_Leben_10.03.1988.pdf",
        titel: "",
        text: "Vortrag von U. Davatz bei der Elternvereinigung psychisch Kranker (EPK) in Windisch, dem Standort der Psychiatrischen Klinik (1988). Psychische Krankheit ist für sie eine zeitlich verlängerte, verstärkte Normvariante menschlichen Verhaltens. Sie beschreibt fünf Reaktionen der Umwelt auf Psychiatriepatienten – Angst, Ablehnung, Helfen, Neugier und Sündenbock – und ihre Folgen für den Patienten, der sich verängstigt, abgelehnt, bevormundet, verwendet oder verzweifelt fühlt. Fachleute zeigen dieselben Reaktionen in anderer Prägung, etwa das Verschanzen hinter Status und Theorie. Hilfreich sind ein möglichst normaler Umgang, Selbstbeherrschung statt Mitagieren und viele Bürger, die Patienten auf natürliche Weise mittragen, ergänzt durch private Trägervereine wie die EPK.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_täglichen_Leben_15.09.1986.pdf",
        titel: "",
        text: "Abschrift der Tonbandaufnahme eines Vortrags von U. Davatz in der Reihe «Psychiatrie im täglichen Leben» der Elternvereinigung psychisch Kranker (1986). Sie wählt als Schwerpunkt die Rolle der Frau und beschreibt, was von Frauen erwartet wird: Kinder gebären und erziehen, für die Gefühlswelt und die Harmonie in der Familie sorgen, sich anpassen und dem Mann zur Karriere verhelfen – und wie ihnen bei Misserfolg die Schuld zugeschoben wird, etwa an der Krankheit eines Kindes, was die Familientherapie zurückweist. Sie stellt dem die männlich geprägte Ordnung mit Struktur, Strafe und Gerechtigkeit gegenüber, die sich auch in der Kirche zeigt. Unterschiede zwischen den Partnern sollen sich ergänzen statt bekämpfen, sonst wird das Kind hin- und hergerissen; voneinander zu lernen sei eine hohe Kunst, die menschliche Reife verlangt.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_täglichen_Leben_15.09.1986_2.pdf",
        titel: "Psychiatrie im täglichen Leben (überarbeitete Fassung)",
        text: "Überarbeitete Reinschrift desselben Vortrags von U. Davatz in der Reihe «Psychiatrie im täglichen Leben» der Elternvereinigung psychisch Kranker (1986), sprachlich geglättet gegenüber der Tonbandabschrift. Im Mittelpunkt steht die Rolle der Frau: die Erwartungen an sie als Mutter, Hüterin der Gefühlswelt und anpassungsfähige Partnerin, die Schuldzuweisungen an Mütter psychisch Kranker, die die Familientherapie zurückweist, und die männlich geprägte Ordnung aus Struktur, Gesetz und öffentlicher Macht. Die Polarisierung zwischen den Geschlechtern könne bis zur Zerstörung gehen und das Kind zerreissen; besser sei es, vom Andersartigen zu lernen und sich daneben als eigenständig zu zeigen, was grosse menschliche Reife verlange.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_täglichen_Leben_24.03.1988.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag bei der Elternvereinigung psychisch Kranker (EPK) in Baldingen (1988) über den Alltag des Sozialpsychiaters. Der Psychiatriepatient ist für sie weder Unmensch noch Monster, sondern ein Mensch mit Problemen und einer besonderen Lebensgeschichte, der mit unüblichen Lösungsstrategien reagiert; ein neuer Krankheitsbegriff sei nötig. Als Probleme nennt sie falsche Vorstellungen von Psychiatriepatienten, zu grossen Helferwillen im Umfeld, der in Erschöpfung und Ausgliederung endet, die Stigmatisierung durch die Gesellschaft und fehlende betreute Wohnplätze. Sie wünscht sich mehr ambulante Psychiatrie unter Einbezug des Umfelds, mehr Arbeits- und Wohnplätze und mehr Unterstützung für die EPK.",
    },
    Zusammenfassung {
        datei: "Psychiatrie_im_täglichen_Leben_oder_Psychiatrie_in_27.04.1988.pdf",
        titel: "Psychiatrie im täglichen Leben oder Psychiatrie in Alltagssprache",
        text: "Vortrag von U. Davatz in der Reihe der Elternvereinigung psychisch Kranker (EPK) vor Landfrauen (1988). Sie fragt, warum psychische Krankheit als Schande gilt, körperliche aber nicht, und beschreibt sie als überstarke und abnorm lange Reaktion des Fühlens, Denkens oder Verhaltens auf eine belastende Lebenssituation, etwa Manie, Depression, Grössenwahn oder Verfolgungswahn. Solche Reaktionen haben eine korrigierende oder alarmierende Funktion. Man geniert sich dafür, weil man die Kontrolle über Gefühle, Gedanken und Verhalten verliert, und Kontrollverlust gleich Gesichtsverlust ist. Betroffene sollen sich früh Hilfe holen, die Umgebung soll Kranke nicht als Sonderlinge ansehen.",
    },
    Zusammenfassung {
        datei: "Psychiatrische_Probleme_beim_Zusammenleben_von_Jung_und_Alt_29.09.1988.pdf",
        titel: "",
        text: "Vortrag von U. Davatz bei der Elternvereinigung psychisch Kranker (EPK) in Frick (1988). Der Generationenkonflikt ist zeitlos und als treibende Kraft der gesellschaftlichen Entwicklung nicht abzuschaffen. In der Ablösungsphase erleben Eltern die Rebellion der Kinder als Kränkung; die Kinder passen sich wieder an, brechen ab oder weichen in Krankheit wie Sucht, Psychose oder Suizid aus, was die Abhängigkeit erneut verstärkt. Umgekehrt behalten pflegebedürftige betagte Eltern oft ihre Autorität über die erwachsenen Kinder, die zwischen den Loyalitäten zu Eltern, Partner und eigenen Kindern erkranken können. In beiden Fällen rät sie zu Abgrenzung und Loslassen, damit beide Seiten eine menschliche Entwicklung durchmachen.",
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
        datei: "Psychisch_kranke_Eltern_26.11.2012.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 26.11.2012. Die Familie funktioniert als Organismus nach dem Prinzip der Kin-Selection, und der Mensch braucht als «sekundärer Nesthocker» (Portmann) lange elterliche Fürsorge. Sind Eltern mit ungelösten Problemen ihrer Herkunftsfamilie beschäftigt, leidet diese «Brutpflege»: Eine depressive Mutter führt zu distanzierter, eine Mutter mit Borderline-Störung oder Schizophrenie zu unsicherer Bindung, chronische Ehekonflikte zu gespaltener Loyalität. Die Kinder gleichen die Mängel schon früh praktisch und emotional aus oder schützen sich durch Rückzug und andere Bezugspersonen; sie können daran erkranken oder besonders stark werden. Therapeuten sollen die Mängel etwas wettmachen und zusätzliche Bezugspersonen organisieren, etwa einen «Götti» der Caritas, damit die Kinder entlastet werden.",
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
        datei: "Repetition_Genogramme_und_ihre_Symbole_09.11.2012.pdf",
        titel: "Repetition: Genogramme und ihre Symbole",
        text: "Einzelne Seite aus einem Lernjournal «Systemische Sozialpädagogik» eines Aufbaukurses 2011/12, nicht von U. Davatz verfasst. Sie zeigt die gängigen Genogrammsymbole für Mann, Frau, unbekanntes Geschlecht, Haustier, Adoptiv- und Pflegekind, Schwangerschaft, Fehlgeburt, Abtreibung, Tod sowie Zwillinge und eineiige Zwillinge. Für Beziehungsqualitäten führt sie ein Beispielsystem mit Linien für emotionale Nähe, abgebrochene Beziehung, Verschmelzung, Distanz und Konflikt an und vermerkt, dass die Systeme dafür nicht ganz einheitlich sind. Am Schluss lädt eine leere Übung dazu ein, weitere Symbole für Beziehungsarten zu sammeln.",
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
        datei: "Schizophrenie_die_Familiendynamik_der_inneren_Scheidung_26.11.2001.pdf",
        titel: "Schizophrenie – die Familiendynamik der inneren Scheidung",
        text: "Skript von U. Davatz für den Familienkurs I/2001 vom 26.11.2001, das die Schizophrenie entwicklungsgeschichtlich statt diagnostisch betrachtet. Der Schizophrene bleibt in der Pubertät stecken und schafft die Ablösung nicht ganz; die Krankheit wird als Fluchtkrankheit, Denkstörung und Stresskrankheit mit Übererregung (hyperarousal) beschrieben. Im Umfeld entsteht chronischer Stress durch eine unterdrückte, konflikthafte Paarbeziehung der Eltern, die sich gegenseitig disqualifizieren, Konflikte meiden oder autoritär unterdrücken und dem Kind kein klares Referenzsystem bieten; diese kühle Distanz nennt sie «innere Scheidung», mit Double-bind-Situationen als Folge. In der Angehörigenberatung sucht sie die Stressquelle, dämpft das ängstliche Überengagement (high EE), stellt Entwicklung statt Krankheit in den Vordergrund und hält ihren Kurs, ohne böse zu werden. Auch die Eltern durchlaufen einen Lernprozess, der Jahre dauern kann.",
    },
    Zusammenfassung {
        datei: "Schuld-_und_Schamgefühle_Sühne_09.11.2011.pdf",
        titel: "Schuld- und Schamgefühle – Sühne",
        text: "Skript von U. Davatz für den Familienkurs I 2011. Schuldgefühle sind der innere «Hirtenhund», der den Einzelnen an die verinnerlichten Regeln seines Kollektivs bindet; schuldig gesprochen wird man von aussen, schuldig fühlt man sich innen, und beides fällt nicht immer zusammen. Wer zu hohe Ansprüche an sich hat, fühlt sich schnell schuldig, während Kinder, etwa mit ADHS, die zu strenge Regeln nicht einhalten können, sie gar nicht verinnerlichen und später ohne Schuldgefühle abweichen. Sühne, wie Kirchen sie anbieten, taugt therapeutisch nicht; Scham betrifft alle Bereiche, die jemand nicht öffentlich machen will, etwa Schwäche, Fehler oder eine neue kreative Idee, und tritt auf, wo man sich seiner Sache noch nicht sicher ist. Therapeutisch sollen Schuldgefühle gegenüber den Eltern durch Differenzierung von der Herkunftsfamilie abgebaut werden, denn wer Familienregeln übertritt, ohne dazu zu stehen, schädigt sich oft psychosomatisch.",
    },
    Zusammenfassung {
        datei: "Schuldgefühle_Funktion_und_Dysfunktion_02.12.1991.pdf",
        titel: "Schuldgefühle – Funktion und Dysfunktion",
        text: "Kurstext von V. M. vom 2.12.1991 über die Entstehung und Wirkung von Schuldgefühlen. Sie entstehen, wenn man sich zu wenig durchsetzt oder sich selbst nicht wahrnimmt, und führen zur Anpassung an die Umwelt; Angst und Schuld verstärken sich dabei gegenseitig. Der Text vermutet, dass Erwachsene ihre eigenen Schuldgefühle auf die Kinder übertragen, etwa indem ein Familienmitglied als unantastbar dargestellt wird, und dass dieses patriarchale Muster die Entwicklung zum Stillstand bringt. Wer die Ursachen erkennt und die zugeschobene Schuld dorthin zurückgibt, wo sie entstanden ist, durchläuft einen grossen Prozess, der zu mehr Freiheit, eigenen Wertvorstellungen und dem Ablegen der Rolle des «braven Kindes» führt.",
    },
    Zusammenfassung {
        datei: "Selbsthilfe_Eigenverantwortung_oder_Laientherapie_04.03.1986.pdf",
        titel: "Selbsthilfe: Eigenverantwortung oder Laientherapie?",
        text: "Streitgespräch im Tages-Anzeiger (1986) zwischen U. Davatz und Jakob Bösch, Mitbegründer des Teams Selbsthilfe Zürich, über Gesprächsselbsthilfegruppen. Davatz bejaht die individuelle Selbsthilfe des aktiven, selbstverantwortlichen Patienten, kritisiert aber Laiengruppen für psychisch Kranke: Ihre Gruppendynamik könne zerstörerisch sein, selbsternannte Leiter unterdrückten andere, und statt Laien einzusetzen solle man bessere Therapeuten fordern. Bösch hält entgegen, Laienführer dominierten weniger als Fachleute, die Gruppen hülfen Betroffenen, ihre Krankheit anzunehmen, und hätten präventive Wirkung. Am Schluss stehen Hinweise auf Kontaktstellen und ein Beratungstelefon.",
    },
    Zusammenfassung {
        datei: "Sexualverhalten_Sexualstörungen_16.10.1990.pdf",
        titel: "",
        text: "Davatz erklärt Sexualverhalten aus der Verhaltensbiologie: Es dient der Arterhaltung und Paarbindung, ist triebgesteuert, beim Menschen aber lernabhängig; männliches Verhalten liegt nahe bei Jagd und Aggression, weibliches ist rezeptiv und auf Verführung ausgerichtet. Sexualstörungen führt sie auf Unterdrückung und Fehlentwicklung der Sexualität zurück, etwa durch Gruppen, die so ihre Kohäsion erhöhen, oder durch Eltern, die das Sexualverhalten der Kinder kontrollieren. Homosexualität, lesbische Beziehungen, Exhibitionismus, Pädophilie und Inzest werden jeweils mit familiären Konstellationen und Beziehungsmustern in Verbindung gebracht. Es handelt sich um ein Kursskript für Familientherapie II vom 16.10.1990 in Stichworten, dessen Deutungen aus der damaligen Zeit stammen.",
    },
    Zusammenfassung {
        datei: "Sexueller_Missbrauch_Wie_heilt_man_ihn_19.09.2016.pdf",
        titel: "Sexueller Missbrauch – Wie heilt man ihn?",
        text: "Skript von U. Davatz für den Familientheorie-Kurs I vom 19.9.2016. Sexualität dient der Fortpflanzung, dem Wohlbefinden und der Bindung unter Gleichgestellten, kann aber auch Dominanzverhalten sein; von Missbrauch spricht sie, wenn ein Stärkerer einen Abhängigen zur Sexualität zwingt, verführt oder überredet, auch wenn dieser scheinbar freiwillig mitmacht, innerhalb der Familie von Inzest. Inzest tritt in eng geschlossenen Familien auf, deren Ressourcen nicht reichen und die sich keine Hilfe von aussen holen; Täter und Kind sind durch ein Geheimnis verbunden, die Mutter als stille Dulderin hätte das Kind schützen müssen, bei Missbrauch ausserhalb der Familie der Vater. Therapeutisch verlangt sie eine neutrale Haltung, die das Ungleichgewicht der Bedürfnisse über drei Generationen herausarbeitet und so die Last auf das ganze System verteilt. Das Opfer darf dem Täter und den versagenden Beschützern das Unrecht vorhalten, der Täter soll sich entschuldigen; eine öffentliche Verurteilung bringe meist keine Heilung, Rache und Strafe hätten keine heilende Wirkung.",
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
        datei: "Sogenannt_verhaltensgestörte_resp._verhaltensauffällige_25.09.1987.pdf",
        titel: "Sogenannt verhaltensgestörte resp. verhaltensauffällige Schüler",
        text: "Stichworte von U. Davatz zu einem Vortrag an einer Lehramtsschule (1987). Als verhaltensauffällig gelten Kinder, die den Unterricht stören, unter ihren Möglichkeiten leisten, sich zurückziehen, fehlen, häufig streiten, Ess- oder Suchtprobleme haben. Sie zählt die Reaktionen von Lehrern auf – ignorieren, psychologisieren, strafen, ausschliessen, lächerlich machen, Angst machen, sich mit den Eltern verbünden, weiterweisen, selbst verzweifeln oder kapitulieren – und hält fest, dass Strafe und Überfokussierung die Störung verstärken. Hilfreich sei, den Hintergrund im Umfeld zu erkunden, Kollegen oder den schulpsychologischen Dienst beizuziehen, Eltern- und Klassengespräche zu führen und stigmatisierende Abklärungen nur dann anzuordnen, wenn Beratung folgt.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_09.11.1994.pdf",
        titel: "",
        text: "Kurzes Skript von U. Davatz für den Familientherapie-Kurs I 1994, das soziale Vererbung als Form der Anpassung erklärt: Während Viren und Bakterien sich genetisch anpassen, tun es höhere Säugetiere und der Mensch über ihr Verhalten. 80 bis 90 Prozent der täglichen Verhaltensmuster sind automatisiert und im Austausch mit dem Umfeld konditioniert; diese Weitergabe von Generation zu Generation nennt sie «soziale Vererbung», weil sie so präzise ist wie die genetische. Ändert sich das Umfeld, werden die Muster unangepasst und können zu körperlichen und psychischen Krankheiten führen. Aufgabe der Familientherapeuten ist dann, die Entstehung dieser Muster anhand des Genogramms zu analysieren und eine Intervention zur Restrukturierung zu planen.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_Wie_kann_ich_diesen_Prozess_unterbrechen_25.10.1993.pdf",
        titel: "Soziale Vererbung – Wie kann ich diesen Prozess unterbrechen?",
        text: "Soziale Vererbung soll nicht unterbrochen, sondern behutsam umgelenkt werden, denn vererbt wird auch viel Gutes, so das Skript von U. Davatz für den Familientherapie-Kurs I 1993. Zuerst ist der Prozess genau zu studieren: mit einem Schema über mindestens drei Generationen, durch Anhören der verschiedenen Familienmitglieder, mit individuellen statt oberflächlicher Erklärungsmodelle und indem man sich selbst als Mitbeteiligten beobachtet. Persönlich geht man vor, indem man eine typische Situation aufsucht, einen eigenen Verhaltensbaustein durch einen anderen ersetzt, dem Widerstand der Familie standhält und das neue Verhalten in Ursprungsfamilie, Partnerschaft und Beruf wiederholt. Mit dem veränderten Verhalten ändert sich auch das Denken, und ein neues Programm geht an die nächste Generation. Ein Fallbeispiel zeigt, wie sich Alkoholprobleme über Generationen fortsetzen und bei der Tochter als Bulimie wiederkehren.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_in_menschlichen_Systemen_oder_Weitergabe_29.10.1991.pdf",
        titel: "«Soziale Vererbung» in menschlichen Systemen oder Weitergabe des emotionellen Prozesses über mehrere Generationen hinweg",
        text: "Soziale Vererbung bezeichnet in diesem Skript von U. Davatz (Familientherapie-Kurs I, 1991) die Weitergabe von Verhaltensmustern und emotionalen Prozessen über Generationen, im Unterschied zur genetischen Vererbung. Sie wird als Prägungsprozess im emotionalen System beschrieben, der schneller anpassbar, aber schwer umzuprogrammieren ist, und als multifaktorieller, interaktiver Prozess mit Teufelskreisen. Familienthemen, an der Emotionalität erkennbar, werden über Wertvorstellungen und Erziehung weitergegeben. Das Erkennen dieser Themen ist wichtig, sobald das System dysfunktional wird, denn Therapie bedeutet Umprogrammieren der automatischen Muster. Die letzte Seite zeigt ein Muster-Genogramm mit den üblichen Symbolen als Vorlage.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_in_menschlichen_Systemen_oder_Weitergabe_29.10.1991_2.pdf",
        titel: "«Soziale Vererbung» in menschlichen Systemen oder Weitergabe des emotionellen Prozesses über mehrere Generationen hinweg",
        text: "Zweite, anders gesetzte Fassung des Skripts von U. Davatz für den Familientherapie-Kurs I 1991, ohne die Genogramm-Vorlage; der Text ist derselbe. Soziale Vererbung gibt Verhaltensmuster über Lernprozesse zwischen den Generationen weiter, schneller anpassbar als die genetische, aber träge und im emotionalen System verankert. Innerhalb der Familie geschieht sie über Erziehung und emotionale Konditionierung; Symptome brechen meist bei emotional belasteten Familienthemen aus. Familienthemen herauszufinden wird wichtig, sobald ein System nicht mehr funktioniert, denn erst dann lassen sich alte Muster umprogrammieren.",
    },
    Zusammenfassung {
        datei: "Soziale_Vererbung_oder_Weitergabe_von_menschlichen_26.03.1988.pdf",
        titel: "«Soziale Vererbung» oder Weitergabe von menschlichen Verhaltensmustern über mehrere Generationen hinweg innerhalb eines Familiensystems",
        text: "Workshop-Unterlage von U. Davatz zur Familientherapie an der Orientierungstagung «Bedeutende Psychotherapieformen der Gegenwart» (1988). Leben ist ein evolutiver Prozess in dauernder Wechselwirkung mit der Umwelt und darum einmalig; allgemein gültige Regeln sind im Menschlichen schwer zu finden, wie sie es mit Bowen dennoch versucht. Weil der Mensch sich jeder konstruierten Regel anpasst, beweist deren Bestätigung wenig. Regeln erkennt nur, wer das monokausale Denken ablegt, prozesshaft denkt, lange und möglichst angstfrei beobachtet und nicht zu schnell helfen will. Sichtbar werden die Muster im Familiendiagramm, dessen leere Vorlage beiliegt.",
    },
    Zusammenfassung {
        datei: "Sozialpsychiatrie_11.12.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz für eine Fortbildung an der Psychiatrischen Klinik Königsfelden (1986). Die Sozialpsychiatrie betrachtet den Patienten nicht als isolierte pathologische Einheit, sondern in Familie, Nachbarschaft, Arbeit, Freizeit, Schule und religiös-ethischer Umwelt. Krankheit ist für sie ein lang andauerndes, personenspezifisches Reaktionsmuster als Anpassung an Stress in Beziehungen, begrenzt durch den Bau des Gehirns und erlernte Muster. Diagnostisch zählt darum die Lebensgeschichte mehr als die Krankheitsgeschichte; therapeutisch verändert man entweder die Stressoren durch Beratung des Umfelds oder die Reaktionsmuster des Patienten. Je jünger und kränker der Patient, desto eher Umfeldberatung; die Klinik dämpfe oder stimuliere dagegen meist nur chemisch, ohne den Stressor anzugehen.",
    },
    Zusammenfassung {
        datei: "Spitex_in_der_Psychiatrie_18.03.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Haus- und Krankenpflegeverein (1986). Die spitalexterne Pflege wachse in der somatischen Medizin, stecke in der Psychiatrie aber in den Kinderschuhen; «Spitex» heisse dort meist nur «Ex-Spit», also frühere Entlassung. Die Folgen seien Drehtürpsychiatrie, Verelendung wie in Italien und den USA und laienhafte Betreuung in kommerziellen Wohnangeboten. Statt neuer Kliniken oder grosser staatlicher Zentren fordert sie hochqualifizierte Fachleute, die Hausärzte, Gemeindeschwestern, Hauspflegerinnen, Suchtberater und Fürsorger beraten. So könnten Patienten in Familie, Wohnung und Arbeitsplatz bleiben, Krisen und Ersthospitalisationen würden vermieden.",
    },
    Zusammenfassung {
        datei: "Streit_unter_Geschwistern_19.11.1998.pdf",
        titel: "",
        text: "Vortrag von U. Davatz für die Elternbildung Reinach vom 19.11.1998. Geschwister kämpfen natürlicherweise um Mutterliebe, väterliche Anerkennung und Ressourcen, kennen aber durch die Geburtsreihenfolge ihren Platz; Eltern, die aus Gerechtigkeitssinn das Jüngere gegen das Ältere schützen oder ein Kind mit einem eigenen ungelebten Auftrag bevorzugen, verstärken den Konflikt, der sich bis zum Erbstreit über Generationen fortsetzen kann. Bei gehäuftem Streit sollen Eltern nicht zu schnell eingreifen, sondern prüfen, ob sie selbst Täter-Opfer-Situationen schaffen, ein Kind bevorzugen, zum Sündenbock machen oder in die Elternrolle drängen, und ob Ressourcen und gegenseitige Unterstützung fehlen. Sie rät, ältere Kinder bei einer Geburt nicht wegzugeben, ihre Neugier am Baby zu nutzen, Konflikte nach Lösungen statt nach Schuld anzugehen und notfalls ohne Schuldzuweisung zu trennen. Streit unter Geschwistern gehört zum sozialen Lernen und sollte nicht als unmoralisch verurteilt werden.",
    },
    Zusammenfassung {
        datei: "Strukturerkennung_innerhalb_der_Familie_Opferrolle_25.06.2001.pdf",
        titel: "Strukturerkennung innerhalb der Familie – Opferrolle",
        text: "Skript von U. Davatz für den Familientherapiekurs I/2001 vom 25.6.2001. Um die Struktur einer Familie zu erkennen, braucht es eine beschreibende, nicht wertende, quasi amoralische Haltung, für die man die eigenen Wertvorstellungen ständig hinterfragen muss; «Opferrolle» ist deshalb kein systemischer Begriff. Systematisch erfasst wird die Familie über ein Genogramm von drei Generationen, die Beschreibung der Bezugspersonen und der weitergegebenen Werte, die Analyse der Paar- und Geschwisterbeziehungen (Dominanz, Kampf, Rückzug, Fusion, Cut-off), der Rollenmuster und schliesslich der Interaktion beider Herkunftssysteme über die Zeit. Therapeutisch soll man bei den hierarchisch wichtigsten Personen eingreifen, also bei Eltern und Gesunden statt beim Kind oder Patienten, wertneutral und strukturell statt nach Täter und Opfer. Der Therapeut ist Helfer, nicht Richter: «Jeder Täter war einmal ein Opfer», und beide versuchen, ihn auf ihre Seite zu ziehen.",
    },
    Zusammenfassung {
        datei: "Sucht_aus_systemischer_Sicht_16.12.2013.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 16.12.2013, ab Seite 2 mit dem Zwischentitel «Suchtverhalten in Familien – Sucht und Schizophrenie». Sucht beginnt meist in der Pubertät, wenn der Jugendliche die Abhängigkeit von den Eltern gegen die von einer Substanz tauscht, im Glauben, diese besser im Griff zu haben; sie dient als selbstzerstörerische Abgrenzung gegen elterliche Übergriffe, auf die Eltern mit mehr Kontrolle reagieren. Sucht läuft über das Belohnungssystem und ersetzt fehlende alltägliche Befriedigung; Suchtfamilien haben schlechte Konfliktlösungsstrategien, und Verdrängen, Leugnen und Lügen gehören dazu. Therapeutisch empfiehlt sie keine Symptombehandlung, sondern die Stärkung des Familiensystems und eine Auseinandersetzung auf Augenhöhe; Eltern müssen ihren Kontrollzwang aufgeben, Rückfälle gehören zum Verlauf, und Kontrollen dienen nur der Realitätsprüfung. Schizophrenie wird bei Jugendlichen oft durch Cannabis und andere Drogen ausgelöst und dann mit Suchtmitteln selbst behandelt, im Kern geht es aber um eine misslungene Ablösung von einem zu kontrollierenden Elternhaus.",
    },
    Zusammenfassung {
        datei: "Sucht_aus_systemischer_Sicht_24.09.2012.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 24.9.2012, eine frühere, kürzere Fassung des Skripts vom Dezember 2013 ohne den Teil über Suchtfamilien und Schizophrenie. Sucht beginnt meist in der Pubertät, wenn Jugendliche die Abhängigkeit von ihren Bezugspersonen gegen die von einer Substanz oder einem Ritual tauschen; der Ablösungskonflikt spielt sich dann im Symptom ab statt zwischen Patient und Eltern. Der Systemtherapeut soll das Symptom nicht in den Mittelpunkt stellen, sondern das Familiensystem und beide Eltern stärken, damit eine Auseinandersetzung auf Augenhöhe und die Ablösung möglich werden, notfalls mit dem therapeutischen Umfeld als Ersatz. Er darf keine emotionale Verantwortung für Rückfälle übernehmen, nicht über sie enttäuscht sein, Lügen nur behutsam aufdecken und Kontrollen nicht als Therapie verstehen.",
    },
    Zusammenfassung {
        datei: "Suchtmechanismen_und_Essen_27.07.1998.pdf",
        titel: "",
        text: "Überlebenswichtige Verhaltensweisen wie Essen, Trinken, Schlafen oder Sexualität sind mit einem Suchtmechanismus gekoppelt, der sie mit einem Glücksgefühl belohnt; derselbe Endorphin-Mechanismus wirkt bei extremem Stress, so das Skript von U. Davatz für den Familientherapie-Kurs 1998. Weil Essen belohnt, wird es in der Erziehung als Belohnung und Strafe eingesetzt, und wer die Nahrung verteilt, ob Mutter oder Politiker, hat Macht. In der Ablösung kann ein Kind gegen eine kontrollierende Mutter nur über Selbstschädigung gewinnen: Die Anorexie ist ein Machtkampf um das Essen, der durch vermehrte Kontrolle, später durch Gewichtskontrolle und Sondendrohung der Ärzte, zum Teufelskreis wird; Drogensucht folgt demselben Muster. Bei Erwachsenen deutet sie Fresssucht als Ersatz für fehlende Kommunikation, als Selbstbelohnung oder verschluckte Aggression und Bulimie als Ausdruck von Abneigung («es kotzt mich an»).",
    },
    Zusammenfassung {
        datei: "Symbiose_zwischen_Mutter_und_Kind_sowie_symbiotische_22.09.2004.pdf",
        titel: "Symbiose zwischen Mutter und Kind sowie symbiotische Beziehung in der Paarbeziehung",
        text: "Fortbildungsvortrag am Psychiatrischen Zentrum Biel über Symbiose zwischen Mutter und Kind und in der Paarbeziehung. Ausgehend von Mahler, Klein und Bowen (Fusion) wird die natürliche Symbiose, in der der Energiefluss auf die nächste Generation gerichtet ist, von der pathologischen unterschieden, in der das Kind zum Versorger der Eltern wird und sich nicht ablösen darf. Bei Paaren würden unbefriedigte Kindheitsbedürfnisse über die Partnerschaft nachgeholt; fehlende Respektierung individueller Grenzen führe zu Kränkung, Aggression und Beziehungsgewalt. Therapeutisch gilt: die Bedürfnisse der Eltern auf anderem Weg als über das Kind erfüllen, bei Paaren beide in ihrer Autonomie stärken und keine Verhandlungstaktik oder Mediation anwenden.",
    },
    Zusammenfassung {
        datei: "Süchte_Suchtprobleme_13.01.1988.pdf",
        titel: "Süchte – Suchtprobleme",
        text: "Stichworte von U. Davatz zu einem Vortrag bei der Elternvereinigung ELPOS (1988). Suchtverhalten definiert sie als einseitige Ersatzbefriedigung, Flucht, Problemlösung mit untauglichen Mitteln und chemische Bekämpfung der Angst. Gebahnt wird es durch unterdrückte Gefühle in der Kindheit, durch das Vorleben eigener Süchte wie Rauchen, Alkohol, Fernsehen oder Arbeit und durch dauernden Stress in der Familie. Bei Sucht der eigenen Kinder rät sie zu Angstreduktion, Blick auf die ganze Familie, Defokussieren der Sucht, objektiver Hilfe und keiner falschen Scham; bei eigener Sucht zu anderen Befriedigungen, Problemlösung und weniger Anpassung.",
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
        datei: "Therapie_von_Essstörungen_31.08.1998.pdf",
        titel: "",
        text: "Skript von U. Davatz vom 31.8.1998 zum therapeutischen Umgang mit Essstörungen, gegliedert nach Angehörigen und Patienten. Eltern, Freunde und nicht kundige Fachleute versuchen es mit Zureden, dann mit heftiger Emotionalität (high EE) und schliesslich mit Wut, Strafe oder Beziehungsabbruch, was das Symptom nur verschlimmert; dahinter steht eine Angst, die sie selbst in Zwangsverhalten treibt. Der Therapeut soll diesen Teufelskreis unterbrechen, etwa indem er das Thema Essen tabuisiert, und bei ängstlichen, misstrauischen Eltern nicht in den Machtkampf um die Kontrolle einsteigen, sondern ihre Angst ins Zentrum stellen. Mit der Patientin oder dem Patienten soll an der Autonomie statt am Essen gearbeitet werden, ohne ungebetene Ratschläge, mit voller Akzeptanz und Ernstnehmen der verzerrten Selbstwahrnehmung. Belohnung und Strafe über Freiheit oder Sondenernährung lehnt sie ab.",
    },
    Zusammenfassung {
        datei: "Trauma_und_Migration_21.11.2016.pdf",
        titel: "",
        text: "Stichwortskript von U. Davatz für den Familientheorie-Kurs I vom 21.11.2016. Migranten wandern immer von etwas weg und auf etwas zu: vor politischer Unruhe, wirtschaftlicher Not, Familienkonflikten und Cut-off oder persönlichen Niederlagen. Die hohen Erwartungen an das Auffangland werden oft enttäuscht, was zu Aggression auf beiden Seiten und zu psychosomatischen Krankheiten führt; diese Erwartungshaltung soll herausgearbeitet werden, ebenso die kulturellen Unterschiede, die verklärte Heimat und das verteufelte Auffangland. Bei Kriegstraumata rät sie zu empathischer, aber sachlicher Traumatherapie ohne allzu viel Mitleid. Für die neue Beheimatung sollen Sitten und Werte des Auffanglandes klar erklärt und die Unterschiede ohne Wertung aufgezeigt werden, mit Wertschätzung für beide Kulturen.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_Psychiatriepatienten_ausserhalb_der_Klinik_03.06.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor den Sozialtätigen eines Bezirks (1986). Im Sozialstaat überschneiden sich viele Institutionen mit unklaren Zuständigkeiten, was zu Kompetenzstreit, Doppelspurigkeit, Über- und Unterbetreuung und schliesslich zum Fallenlassen der Patienten führt. Der Patient stammt oft aus einer überengagierten Familie, passt sich ängstlich an und erlebt im sozialen Netz dasselbe Muster wieder; trotz vieler Helfer vereinsamt er. Die Betreuer rivalisieren, fühlen sich gegeneinander ausgespielt und lehnen ihn am Ende ab. Voraussetzungen für eine gute Zusammenarbeit sind regelmässiger Kontakt, gegenseitige Anerkennung der Kompetenz, eine klar bestimmte Hauptbezugsperson und Kenntnis der Vorgeschichte.",
    },
    Zusammenfassung {
        datei: "Umgang_mit_Religion_18.08.2014.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familientherapie-Kurs I 2014 vom 18.8.2014. Der Mensch hat als soziales Wesen «soziale Gene» und organisiert sich in Kollektiven mit Hierarchie und Regeln; ethnische Kollektive sind genetisch, religiöse nur mental verbunden und können darum über Mission rasch wachsen, was Christentum und Islam in einer kriegerischen Zeit mit vielen vaterlosen Familien zu einem Rekrutierungsinstrument machte. Sie unterscheidet in-group-Verhalten mit Dominanz und Empathie, das sich Religionen zunutze machten, von brutalem out-group-Verhalten gegenüber Fremden und Ausgestossenen. In einer globalisierten Welt gebe es eigentlich keine out-group mehr, nur noch allgemeine Menschenrechte; Religion gehöre darum ins Private und nicht in die Politik. Auf die Familientherapie übertragen hat jede Familie ihre in-group-Regeln, die umso strenger sind, je ängstlicher sie ist; Differenzierung heisst, Tabus zu brechen, aber in Kontakt zu bleiben und durch Empathie die Angst im System zu senken.",
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
        datei: "Umgang_mit_schwierigen_Mitarbeitern_08.05.1987.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag bei einem Grossverteiler (1987). Auf schwierige Personen reagiert die Umwelt meist mit Schonung – Watte, extreme Toleranz, stundenlanges Zuhören, Hilfsangebote – oder mit Belehrung – Zurechtweisen, Appelle, Befehle, Kontrolle, Drohungen. Beides verstärkt die Symptome, die Schonung durch Belohnung und Sonderrolle, die Belehrung durch Konflikt und Isolierung, und endet oft im Ausstoss durch Kündigung oder Klinik oder im Davonlaufen. Hilfreich sind eigene Abgrenzung, klare Stellungnahmen und Entscheidungen, Kontrolle der eigenen Gefühle und ein neutraler Kontakt zur Person.",
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
        datei: "Verhalten_in_der_Partnerbeziehung,_welche_Rückschlüsse_09.11.2011.pdf",
        titel: "Verhalten in der Partnerbeziehung, welche Rückschlüsse können auf die Ursprungsfamilie gemacht werden",
        text: "Skript von U. Davatz für den Familienkurs I 2011 darüber, wie die frühe Interaktion mit den Eltern das Beziehungsverhalten Erwachsener und sogar die Hirnstrukturen prägt. Als «Gewohnheitstier» erwartet der Mensch in der Partnerschaft, was er kennt: Wer verwöhnt wurde, erwartet vom Partner dieselbe Verwöhnung, wer nicht ernst genommen oder im Stich gelassen wurde, rechnet wieder damit und kämpft oder geht auf Distanz, und der Konflikt ist vorprogrammiert. Funktionalisierte Kinder, die die Defizite ihrer Eltern ausgleichen mussten, bringen als Erwachsene eine kompensatorische Erwartungshaltung mit und fordern ihren Nachholbedarf mit grosser Emotionalität ein, was oft als Borderline-Persönlichkeit diagnostiziert wird und in Trennungen und neuen Partnerschaften mit denselben Forderungen endet. Diese Erwartungen müssen nicht beim gleichen Geschlecht bleiben, und wer sie in der Herkunftsfamilie erkennt, kann Patient und Partnerkonflikt besser verstehen.",
    },
    Zusammenfassung {
        datei: "Vernissage_Malgruppe_Rheinfelden_Ansprache_06.01.1987.pdf",
        titel: "Vernissage Malgruppe Rheinfelden – Ansprache von Frau Dr. med. Davatz",
        text: "Ansprache von U. Davatz zur Vernissage einer Malgruppe der Gesundheits-Beratungsstelle Rheinfelden (1987), in Stichworten. Kunst sei mehr als Können: lebendiger Ausdruck, einmalig, fehlerhaft, spielerisch und nicht kontrollierbar, während die technisch geschulte Gesellschaft das Verständnis für lebendige Prozesse verliere. Beim Malen komme man mit der eigenen Wandlungsfähigkeit in Kontakt; das Fliessenlassen und Entstehenlassen befreie Kräfte, um mit Krankheit und Schwierigkeiten besser fertig zu werden. Auf der zweiten Seite klebt ein Zeitungsbericht über die Ausstellung der zwölf Malerinnen, der auch ihre Ansprache erwähnt.",
    },
    Zusammenfassung {
        datei: "Vernissage_der_Malgruppe_im_Kurzentrum_in_Rheinfelden_06.01.1987.pdf",
        titel: "",
        text: "Stichwortblatt von U. Davatz zu ihrer Ansprache an der Vernissage einer Malgruppe in Rheinfelden (1987). In einer Gegenüberstellung grenzt sie Leben von toter Materie und Technik ab: Leben ist einmalig statt repetitiv, fehlerhaft statt präzis, abwechslungsreich, spielerisch, gewachsen statt konstruiert, komplex und wandlungsfähig statt starr. Malen und künstlerisches Schaffen bringen den lebendigen Prozess näher, lassen Eigenes entstehen und schöpfen daraus neue Lebenskräfte und Anstösse zur Veränderung. Kreativ sein heisse nicht perfekt sein, sondern den Mut haben, das Leben leben zu lassen.",
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
        datei: "Viel_Wissen_um_Krankheit_macht_noch_nicht_gesund_20.03.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz für eine Diskussionsrunde an einer Kantonsschule (1986), eine kritische Betrachtung der Präventionspolitik. Aufklärung befreit von Angst vor Naturphänomenen und von manchen Infektionen, Aufklärung über Krankheiten aber macht Angst, und Angst vor Krankheit macht krank; Präventionskampagnen dienten oft eher der Publizität der Absender und wirkten höchstens wie eine neue Mode. Wirksamer sei es, im kritischen Augenblick richtig zu handeln: Prävention als Lebens- und Konflikthilfe dort, wo sie nötig ist, und lebensfreundliche statt nur produktionsfreundliche Strukturen. Präventivfachleute sollen an Knotenpunkten wie Schule, Lehrbetrieb, Kindergarten und Mütterberatung beraten, und jede medizinische Person kann helfen, indem sie nicht auf das Symptom fokussiert.",
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
        datei: "Vorbereitung_der_Eltern_auf_die_Ablösungsphase_13.01.1988.pdf",
        titel: "",
        text: "Vortrag von U. Davatz bei der Elternvereinigung ELPOS (1988). Die beste Vorbereitung auf die Ablösung ist das intensive Erleben der früheren Lebensmomente: Geburt, Stillen und genügend Zeit für das Kind in seinen Übergangsphasen; Verpasstes führt oft zu Schuld- und Insuffizienzgefühlen, die das Loslassen erschweren. Zusätzlich erschwert wird die Ablösung, wenn das Kind eine gestörte Ehe zusammenhält, krank, behindert oder besonders begabt ist, unerfüllte Wünsche der Eltern tragen soll, die Eltern in der Erziehung uneins sind oder es als Partnerersatz bei einem Alleinerziehenden lebt. Für jeden Fall nennt sie Gegenmittel, etwa Verluste offen besprechen, Überbehütung vermeiden, Ehekonflikte selbst angehen, klare Kompetenzbereiche festlegen und nichts nachholen wollen.",
    },
    Zusammenfassung {
        datei: "Vorträge_und_Veröffentlichungen_09.11.1988.pdf",
        titel: "Vorträge und Veröffentlichungen U. Davatz 1988",
        text: "Verzeichnis der Vorträge und Veröffentlichungen von U. Davatz aus dem ersten Halbjahr 1988, 20 nummerierte Einträge mit Titel, Anlass und Datum. Mehrfach erscheint die Reihe «Psychiatrie im täglichen Leben» der EPK in verschiedenen Aargauer Gemeinden. Daneben stehen Vorträge zu Sucht und Suchtberatung, zur Ablösung der Jugendlichen, zu Autorität und Toleranz in der Erziehung, zur Familientherapie, zu Depressionen und zum Hospitalismus, eine Podiumsdiskussion zur Prävention sowie ein Beitrag zur Tablettensucht in der Zeitschrift «Sozialarbeit & Suchtprobleme». Die Liste zeigt die breite Vortragstätigkeit vor Laien, Pflegenden, Behörden und Fachleuten.",
    },
    Zusammenfassung {
        datei: "Vortragsliste_1986_bis_1988_09.11.1988.pdf",
        titel: "",
        text: "Tabellarische Liste der Vorträge von U. Davatz von Januar 1986 bis März 1988, mit Datum, veranstaltender Gesellschaft oder Verein und Thema. Die Anlässe reichen von Schulpflegen, Frauen- und Elternvereinen über Krankenpflegeschulen und Fortbildungen bis zum Gottlieb Duttweiler Institut, Coop Schweiz und einer Weiterbildungstagung in Dalian, China. Im Vordergrund stehen Drogen- und Suchtprobleme Jugendlicher, der Umgang mit psychisch Kranken, Sozialpsychiatrie und Erziehungsfragen. Die letzten Einträge von Anfang 1988 sind von Hand durchgestrichen, ein Datum und ein Thema sind handschriftlich korrigiert.",
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
        datei: "Warum_sucht_die_heutige_Jugend_so_oft_die_Flucht_in_die_20.09.1987.pdf",
        titel: "Warum sucht die heutige Jugend so oft die Flucht in die Sucht?",
        text: "Abschrift eines Vortrags von U. Davatz in Widen (1987). Sucht versteht sie als Fluchtreaktion vor Druck, und sie sucht die Ursachen bei allen Beteiligten statt nur bei den Eltern. Die Kleinfamilie mit ein, zwei Kindern setzt das Kind stärker unter Druck als eine Grossfamilie, Erziehungsratgeber verunsichern die Eltern, überfüllte Lehrpläne und der Drang zu höherer Ausbildung belasten die Schule. Dahinter steht eine Gesellschaft, die Fortschritt nur als quantitatives Wachstum begreift und alles kontrollieren will, obwohl sich Lebendiges nicht kontrollieren lässt. Sie fordert, mehr Geld und Gedanken in die Prävention statt in die oft aussichtslose Heilung zu stecken, und versteht Wachstum beim Erwachsenen als Veränderung statt als Zunahme.",
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
        datei: "Was_sind_die_Besonderheiten_im_Alter_Was_sind_unsere_07.11.1985.pdf",
        titel: "Was sind die Besonderheiten im Alter? Was sind unsere Schwierigkeiten im Umgang mit alten Menschen?",
        text: "Stichworte von U. Davatz für eine Weiterbildung der Sportleiterinnen der Pro Senectute (1985). Alter bedeute einerseits Verlust an Flexibilität bis zur Sturheit, Verlangsamung, Gedächtnisverlust, Angst vor dem Tod und Überzeichnung der eigenen Charakterzüge, andererseits Erfahrung, Weisheit, Gelassenheit und eine spielerische Freude am Leben. Schwierig im Umgang seien die begrenzte Belastbarkeit, die Versuchung, auf Sturheit mit eigener Sturheit zu antworten, Ablösungsprobleme und die Umkehr der Abhängigkeit. Das Turnen sei ein guter Einstieg, um Beziehungen zu fördern; eine Gruppenaufgabe fragt, was im Umgang mit alten Menschen am meisten Mühe und am meisten Freude macht.",
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
        datei: "Welche_Patienten_können_im_Wohnheim_aufgenommen_werden_28.05.1986.pdf",
        titel: "Welche Patienten können im Wohnheim aufgenommen werden?",
        text: "Stichworte von U. Davatz zu einem Vortrag an der Generalversammlung der Elternvereinigung psychisch Kranker (1986). Geeignet für ein Wohnheim sind Patienten, die nicht akut dekompensiert sind, einen geregelten Tagesablauf schaffen, ihre Medikamente selbständig nehmen, gruppenfähig und für eine Teilzeitarbeit fähig sind, eine Langzeitbleibe suchen und mit Einverständnis der Eltern selbst motiviert sind. Nicht geeignet sind akut suizidale, drogensüchtige oder akut psychotische Patienten, solche, die nur eine Notschlafstelle suchen, schon selbständig wohnen, nicht arbeiten wollen oder nicht gruppenfähig sind. Zudem muss der Patient in die bestehende Gruppe passen, und die Gruppe darf nicht gerade in einer Krise sein.",
    },
    Zusammenfassung {
        datei: "Weshalb_habe_ich_einen_18_Jahre_älteren_Mann_geheiratet_28.10.1993.pdf",
        titel: "Weshalb habe ich einen 18 Jahre älteren Mann geheiratet?",
        text: "Persönliche Kursarbeit einer Teilnehmerin der Familientherapie-Weiterbildung 1993 (Leitung U. Davatz) zur Frage, weshalb sie einen deutlich älteren Mann geheiratet hat. Mit dem familienorientierten Ansatz nach Bowen geht sie dem Zusammenhang mit der Vaterthematik nach, auch vor dem Hintergrund des Todes ihres Vaters. Sie beschreibt Spannungen in der Partnerschaft durch ihre berufliche Entwicklung, die vorgeschlagenen Schritte (Interesse an den Arbeiten des Mannes, gemeinsames Hobby, Gespräche mit Mutter und Schwester) und deren Wirkung. Ihr Fazit: Die Familientherapie habe unbewusste Abläufe sichtbar gemacht und das Verständnis für die unterschiedlichen Lebensphasen der Partner gefördert.",
    },
    Zusammenfassung {
        datei: "Wie_bewahre_ich_meine_Frische,_Zuversicht,_Phantasie_und_14.01.1988.pdf",
        titel: "Wie bewahre ich meine Frische, Zuversicht, Phantasie und Kraft in der Betreuung von Langzeitpatienten?",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Spitex-Stützpunkt (1988). Helfen gibt gute Gefühle und Macht, ohne Erfolg aber frustriert und entmutigt es. Wer Langzeitpatienten betreut, braucht darum einen kleinen Erfolgs- und Machtanspruch, Ausdauer wie ein Langstreckenläufer und emotionale Kraft auch aus dem Privatleben. Statt Helfer soll man Begleiter sein: Die Krankengeschichte entmutigt, die Lebensgeschichte der Patienten ist dagegen spannender als jeder Roman und lässt sich mit Neugier und Humor erkunden. Stellvertretend für Angehörige soll man nur im äussersten Notfall handeln und sonst die natürlichen Bezugspersonen stützen.",
    },
    Zusammenfassung {
        datei: "Wie_erkenne_ich_meine_Achillesferse_Was_sind_meine_typischen_31.10.2002.pdf",
        titel: "Wie erkenne ich meine Achillesferse? Was sind meine typischen Reaktionsmuster bei Stress? Auf welche Konstellationen bin ich speziell empfindlich?",
        text: "Für eine Beratung/Weiterbildung zu «Engagiert pflegen ohne Burnout» geht es darum, die eigene Achillesferse zu erkennen. Die Schwachpunkte hängen mit der Prägung in der Herkunftsfamilie zusammen; Einsicht erleichtert, verändert aber erst durch Übung, am besten im Familienmuster. Typische Stressmuster werden körperlich gedeutet (Nacken, Kopf, Rücken, Bauch, Tennisellbogen) und als Kontroll-, Kampf-, Flucht- oder Totstellverhalten beschrieben. Als Faustregeln gegen Überforderung gelten: in der Begegnung statt im helferischen Aktivismus pflegen, Erfolge geniessen, sich selbst loben, offen kommunizieren, ein eigenes «Credo» formulieren, Grenzen halten und mit Fehlern menschlich umgehen.",
    },
    Zusammenfassung {
        datei: "Wie_gehe_ich_mit_psychisch_Kranken_um_04.06.1987.pdf",
        titel: "Wie gehe ich mit psychisch Kranken um?",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Frauenverein (1987). Kranke stehen für Gesunde tiefer auf der sozialen Leiter, und Gesunde reagieren darauf mütterlich schonend, väterlich belehrend und dominant oder geschwisterlich ablehnend. Psychisch Kranke spüren das besonders und fühlen sich dann infantilisiert, bevormundet oder ausgegrenzt; am Arbeitsplatz weichen sie in Absenzen aus. Sie rät zu einem möglichst normalen Umgang: normale statt übermässiger Aufmerksamkeit, eigene Abgrenzung statt Beschützen, Kontakt halten statt ablehnen oder rivalisieren und den Kranken nicht für eigene Machtansprüche verwenden.",
    },
    Zusammenfassung {
        datei: "Wie_kläre_ich_meine_Kinder_auf_über_Sexualverbrechen,_ohne_01.09.1988.pdf",
        titel: "Wie kläre ich meine Kinder auf über Sexualverbrechen, ohne ihnen Angst zu machen?",
        text: "Vortrag von U. Davatz (1988). Vor der Aufklärung über abnormes Sexualverhalten steht die Aufklärung über normales: die Bereitschaft der Eltern, Fragen der Kinder über alle Entwicklungsjahre hinweg zu beantworten, was ein natürliches Verhältnis zur eigenen Sexualität verlangt. Sie unterscheidet das Sexualvergehen, bei dem ein Erwachsener das Kind unter Gewaltandrohung missbraucht, vom Sexualverbrechen, bei dem das Opfer verfolgt wird und Fluchtverhalten den Täter noch antreibt. Kinder sollen sachlich informiert werden, ohne die Täter als Monster darzustellen, denn diese treten meist freundlich auf und stammen oft aus dem Bekanntenkreis. Angst macht kopflos; wirksamer ist ein Kind, das nein sagen darf, sich wehren kann und Selbstvertrauen hat.",
    },
    Zusammenfassung {
        datei: "Wie_schütze_ich_mein_Kind_vor_Sexualverbrechen_22.08.1989.pdf",
        titel: "",
        text: "Vortrag vor einer Gruppe junger Mütter (1989). U. Davatz fragt nicht nur, wie man Kinder schützt, sondern wie Sexualverbrechen entstehen: Sexualität kann als Machtmittel oder als Angstlöser wie ein Suchtmittel dienen, und erlittene sexuelle Frustration wird mitunter an ein neues Opfer weitergegeben. Sie beschreibt typisch männliches und weibliches Sexualverhalten als Muster von Jagd und Abwarten. Den besten Schutz sieht sie nicht in Verboten und Angstmachen, sondern in einer freiheitlichen Erziehung zur Wehrhaftigkeit und Selbständigkeit, die dem Kind seinen inneren Instinkt lässt. Ängstliche, fügsame Kinder ziehen Täter eher an als wehrhafte.",
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
        datei: "Wo_liegen_mögliche_Ansatzpunkte_für_die_Sozial-_und_11.06.1987.pdf",
        titel: "Wo liegen mögliche Ansatzpunkte für die Sozial- und Präventivmedizin?",
        text: "Vortrag von U. Davatz an einer Tagung des Sozialpsychiatrischen Dienstes über Prävention (1987). Prävention versteht sie nicht in erster Linie als Verhütung von Krankheit, sondern als überlegten, kreativen Umgang mit kritischen Situationen, die heute oft reflexhaft wie Notfälle behandelt werden. Eine kritische Situation kündigt sich durch ein körperliches, seelisches oder Verhaltenssymptom an; ein lebendiges System gerät dann leicht in einen Teufelskreis, der bis zum Notfall eskaliert. Als Beispiele nennt sie Schulangst, Bettnässen, Delinquenz, Drogen und Dekompensation in der Ehe sowie natürliche Knotenpunkte wie Geburt, Tod oder Krankheit in der Familie. Das Symptom soll als Signal gedeutet und das ganze System angegangen werden, statt nur das Symptom zu bekämpfen.",
    },
    Zusammenfassung {
        datei: "Wo_steht_der_Patient_zwischen_Krankenkasse,_Arzt_und_Staat_17.06.1988.pdf",
        titel: "",
        text: "Vortrag von U. Davatz an der Jahresdelegiertenversammlung einer Krankenkasse (1988). Sie analysiert das Gesundheitswesen als Krankheitsversorgungssystem, in dem der Patient Leistungen fast gratis einkauft, Ärzte und Zulieferer das Angebot nach eigenem Interesse ausbauen, die Kasse im Nachhinein zahlt und der Staat das Defizit über Steuern deckt. So werden Krankheit und Misswirtschaft belohnt, Gesundheit und Produktivität bestraft, und es kommt zur Kostenexplosion. Als Ausweg schlägt sie direkte Rückkoppelungen zwischen Patient, Arzt und Krankenkasse vor, etwa kritischere Patienten als Konsumenten. Ein solches sich selbst regulierendes System mache rigide staatliche Eingriffe überflüssig und gebe dem Patienten die Verantwortung für seine Gesundheit zurück.",
    },
    Zusammenfassung {
        datei: "Wo_steht_der_Patient_zwischen_Krankenkasse,_Arzt_und_Staat_17.06.1988_2.pdf",
        titel: "Wo steht der Patient zwischen Krankenkasse, Arzt und Staat (Fassung mit Schemata)",
        text: "Zweite Fassung des Vortrags von U. Davatz an der Delegiertenversammlung einer Krankenkasse (1988), in Schreibmaschinensatz und mit zwei Schemata. Das erste zeigt die Teufelsspirale eines stark kontrollierten und doch ausser Kontrolle geratenen Krankheitsversorgungssystems, in dem Bürger, Ärzte und Spitäler, Krankenkassen und Staat den Druck weitergeben und sich gegenseitig zu kontrollieren versuchen. Das zweite zeigt ein autoregulatives System mit direkten Rückmeldungen zwischen Patient, Arzt und Krankenkasse und mehr Selbstkontrolle des Bürgers. Der Text fordert Krankenkassen, Ärzte und Patienten zu mehr Information, Rückmeldung und Eigenverantwortung auf.",
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
        datei: "Wohnheime_Übergangslösung_oder_Dauerghetto_08.09.1988.pdf",
        titel: "Wohnheime: Übergangslösung oder Dauerghetto?",
        text: "Thesen von U. Davatz für eine Arbeitsgruppe am Jubiläumssymposium «10 Jahre Sozialpsychiatrische Universitätsklinik Bern» (1988). Sie kritisiert die therapeutische Wohnsituation als künstliche Lebenslage: Therapie sei wie eine Medizin nur in kleinen Dosen bekömmlich, das hierarchische Gefälle und die dauernde Beobachtung führten zu Dekompensation, chronischer Abwehr oder Hospitalismus. Berufspersonen sollen sich im Wohnheim möglichst natürlich verhalten, sich selbst abgrenzen statt die Patienten zu deuten und das therapeutische Wissen auf sich selbst anwenden. Betreutes Wohnen gehört nur als vorübergehende Lösung in therapeutische Hand; als Dauerlösung sollten es möglichst gesunde Pensionseltern führen.",
    },
    Zusammenfassung {
        datei: "Ziele_und_Organisation_des_sozialpsychiatrischen_Dienstes_10.06.1986.pdf",
        titel: "",
        text: "Stichworte von U. Davatz zu einem Vortrag vor einem Akademikerinnenverband (1986). Menschliches Verhalten lasse sich als krank, unreif oder kriminell betrachten, mit den Folgen ärztliche Behandlung, Nacherziehung oder Strafe; jede Sicht schade dem Patienten, weil sie ihn passiv macht, infantilisiert oder in Machtkampf und Ausweichverhalten treibt. Der Sozialpsychiatrische Dienst will Patienten so wenig krank und so erwachsen wie möglich behandeln, klare Grenzen ohne Strafe setzen, nur wenn nötig Verantwortung übernehmen und sie in Familie, Arbeit und Gemeinde integrieren. Sie beschreibt die Organisation mit Nachtklinik, Tageskliniken und Ambulatorien sowie Konsilien, Supervision privater Institutionen und Prävention an Schulen, mit Familientherapie als Ansatz.",
    },
    Zusammenfassung {
        datei: "Zusammenarbeit_unter_Sozialtherapeutischen_Systemen_bei_30.04.2002.pdf",
        titel: "Zusammenarbeit unter sozialtherapeutischen Systemen bei gefährdeter Mutter-Kind-Beziehung",
        text: "Bei gefährdeter Mutter-Kind-Beziehung streiten Helfersysteme oft um Macht über das Kind, obwohl der Brutpflegeinstinkt eigentlich dessen Wohl leiten sollte. Der Vortrag vor dem Psychiatriestützpunkt Biel skizziert ein systemisches Vorgehen bei fraglicher Kindsplatzierung: die Mutter zuerst stützen, professionelle Helfer nicht mit ihr konkurrieren lassen und, falls nötig, die Fremdplatzierung sorgfältig vorbereiten. Eine Person, am besten jene mit der besten Beziehung zur Mutter, soll die Führung übernehmen; die Hierarchie der Institutionen darf nicht entscheiden. Fehlt eine klare Führung, droht dem Kind laut Vortrag die soziale Verwahrlosung.",
    },
    Zusammenfassung {
        datei: "Zusammenfassung_aller_6_Konzepte_von_M._Bowen_26.07.2010.pdf",
        titel: "",
        text: "Skript von U. Davatz für den Familienkurs I vom 26.7.2010, das die Konzepte von Murray Bowen auf zwei Seiten zusammenfasst. Psychische Krankheit gilt als funktionelle Störung in der Familiendynamik über Generationen, die sie als «soziale Vererbung» bezeichnet und die zuerst über die Fusionsbeziehung zwischen Mutter und Kind läuft; je ängstlicher das Klima, desto enger die Fusion und desto rigider die Normen. Wer sich aus einer zu engen Fusion löst, tut es oft über einen «Cut-off» und verliert dabei Unterstützung; ungelöste Ablösung wird auf den Partner übertragen und führt zum Ehekonflikt, in den das Kind als Dreiecksbeziehung hineingezogen wird. Überschüssige Spannung zeigt sich als psychisches, körperliches oder soziales Symptom oder als Ehekonflikt («Familienprojektionsprozess»). Therapeutisch dient der Differenzierungsprozess von der Herkunftsfamilie: eine eigene Position gegenüber den Eltern beziehen, ohne sie zu belehren, was Entwicklungsenergie für die Person und ihre Kinder freisetzt.",
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
