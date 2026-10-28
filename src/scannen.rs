// SPDX-License-Identifier: GPL-3.0-or-later
//
// Scannt einen Vortrag über den automatischen Einzug des Epson GT-2500 und
// legt ein PDF an, das Datum und Titel des Vortrags trägt - im Dateinamen
// und in den Dokumenteigenschaften (Titel, Thema, Stichwörter).
//
// Datum und Titel liest das Programm aus den gescannten Seiten selbst
// (Texterkennung mit Tesseract, siehe `mod ocr`): der Titel ist der Kopf der
// ersten Seite bis zur Autorenzeile, das Datum das erste im Kopf der ersten
// Seite, sonst das letzte im Dokument - dort steht meist «Ort, Datum».
// `--datum` und `--titel` überschreiben die Erkennung. Was erkannt wurde,
// steht auf stderr; geprüft werden muss es trotzdem.
//
// Angesprochen wird der Scanner über SANE (`scanimage`, Backend epson2, dort
// als ES-H300 geführt - das japanische Schwestermodell). Epsons eigene Wege
// taugen dafür nicht: Epson Scan 2 kennt zwar ein Argument `-SCAN`, aber
// kein dokumentiertes Format dafür, und der Image-Capture-Treiber
// «EPSON Scanner.app» ist nur für Intel gebaut und startet auf Apple
// Silicon ohne Rosetta nicht.
//
//   brew install sane-backends tesseract tesseract-lang
//   cargo run --release --bin scannen
//
// Immer `--batch` scannen: ein Einzelscan (`scanimage -o`) lässt den Einzug
// das nächste Blatt vorladen und ungescannt auswerfen.
//
// Jede Seite kommt als JPEG an und wird unverändert (DCTDecode) ins PDF
// gestellt, eine Seite je Bild, Seitengrösse aus Pixelzahl und Auflösung.
//
// Weitere Schalter: --dpi 150 (300 für kleine Schrift oder Notizen),
// --farbe (Vorgabe Graustufen),
// --ordner DIR (Vorgabe: scan/, wird angelegt), --geraet <SANE-Gerät>
// (Vorgabe: der GT-2500 aus `scanimage -L`, nicht ein Epson im Netz),
// --lesen BILD… (nur Erkennung zeigen, nichts scannen),
// --aus BILD… (PDF aus schon gescannten JPEGs, nichts scannen).

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context};

fn main() {
    if let Err(e) = lauf() {
        eprintln!("Fehler: {e:#}");
        std::process::exit(1);
    }
}

struct Optionen {
    datum: Option<String>,
    titel: Option<String>,
    dpi: u32,
    farbe: bool,
    ordner: PathBuf,
    geraet: Option<String>,
    lesen: Vec<PathBuf>,
    aus: Vec<PathBuf>,
}

fn optionen() -> anyhow::Result<Optionen> {
    let mut o = Optionen {
        datum: None,
        titel: None,
        dpi: 150,
        farbe: false,
        ordner: PathBuf::from("scan"),
        geraet: None,
        lesen: Vec::new(),
        aus: Vec::new(),
    };
    let mut args = std::env::args().skip(1).peekable();
    while let Some(a) = args.next() {
        let mut wert = || args.next().with_context(|| format!("{a} braucht einen Wert"));
        match a.as_str() {
            "--datum" => {
                let w = wert()?;
                o.datum = Some(
                    datum_iso(&w)
                        .with_context(|| format!("Datum {w:?} - erwartet JJJJ-MM-TT oder TT.MM.JJJJ"))?,
                );
            }
            "--titel" => o.titel = Some(wert()?.trim().to_string()),
            "--dpi" => o.dpi = wert()?.parse().context("--dpi")?,
            "--ordner" => o.ordner = PathBuf::from(wert()?),
            "--geraet" => o.geraet = Some(wert()?),
            "--farbe" => o.farbe = true,
            "--lesen" | "--aus" => {
                let liste = if a == "--lesen" { &mut o.lesen } else { &mut o.aus };
                while let Some(p) = args.next_if(|p| !p.starts_with("--")) {
                    liste.push(PathBuf::from(p));
                }
            }
            "-h" | "--help" => {
                println!(
                    "scannen [--datum JJJJ-MM-TT] [--titel \"Titel\"] [--dpi 150] [--farbe] \
                     [--ordner DIR] [--geraet SANE-Gerät] [--lesen BILD…] [--aus BILD…]"
                );
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    Ok(o)
}

/// Der GT-2500 aus `scanimage -L`, etwa `epson2:libusb:001:004`. Die
/// USB-Nummern wechseln beim Umstecken, darum nicht fest verdrahten. Im Netz
/// meldet sich zeitweise ein zweites Epson-Gerät (`epson2:net:…`, ein
/// Multifunktionsdrucker ohne diesen Einzug) - und manchmal zuerst. Darum
/// zählt das Modell (SANE nennt den GT-2500 «ES-H300»), dann USB vor Netz.
fn epson_geraet() -> anyhow::Result<String> {
    let aus = Command::new("scanimage")
        .arg("-L")
        .output()
        .context("scanimage nicht gefunden - brew install sane-backends")?;
    let text = String::from_utf8_lossy(&aus.stdout);
    let geraete: Vec<(&str, &str)> = text
        .lines()
        .filter_map(|z| z.strip_prefix("device `")?.split_once('\''))
        .filter(|(g, _)| g.starts_with("epson2:"))
        .collect();
    let modell = |(_, rest): &&(&str, &str)| rest.contains("ES-H300") || rest.contains("GT-2500");
    geraete
        .iter()
        .find(modell)
        .or_else(|| geraete.iter().find(|(g, _)| g.contains(":libusb:")))
        .map(|(g, _)| g.to_string())
        .with_context(|| format!("GT-2500 nicht gefunden - eingeschaltet und eingesteckt?\n{text}"))
}

/// Alle Seiten aus dem Einzug als JPEG nach `tmp`.
fn scannen(opt: &Optionen, tmp: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let geraet = match &opt.geraet {
        Some(g) => g.clone(),
        None => epson_geraet()?,
    };
    eprintln!("Scanne über den Einzug ({geraet}, {} dpi) …", opt.dpi);
    // Ohne Papier meldet scanimage «Document feeder out of documents» und
    // legt keine Datei an - das fangen wir unten über die leere Liste ab.
    let status = Command::new("scanimage")
        .args(["-d", &geraet, "--source", "Automatic Document Feeder"])
        .args(["--adf-mode", "Simplex"])
        .args(["--mode", if opt.farbe { "Color" } else { "Gray" }])
        .args(["--resolution", &opt.dpi.to_string()])
        .args(["-x", "210", "-y", "297", "--format=jpeg"])
        .arg(format!("--batch={}/seite-%03d.jpg", tmp.display()))
        .status()
        .context("scanimage starten")?;

    let mut seiten: Vec<PathBuf> = std::fs::read_dir(tmp)?
        .filter_map(|e| Some(e.ok()?.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jpg"))
        .collect();
    seiten.sort();
    if seiten.is_empty() {
        bail!("keine Seite eingezogen - liegt Papier im Einzug, oder meldet der Scanner einen Papierstau? (scanimage: {status})");
    }
    if !status.success() {
        eprintln!("Hinweis: scanimage endete mit {status}, {} Seiten sind da.", seiten.len());
    }
    Ok(seiten)
}

fn lauf() -> anyhow::Result<()> {
    let opt = optionen()?;
    if !opt.lesen.is_empty() {
        let texte = erkennen(&opt.lesen, opt.dpi)?;
        // SCAN_ZEILEN=1 zeigt jede erkannte Zeile mit ihrem Kasten
        if std::env::var_os("SCAN_ZEILEN").is_some() {
            for (n, zeilen) in texte.iter().enumerate() {
                for z in zeilen {
                    eprintln!(
                        "S.{} oben={:.3} hoehe={:.3} links={:.3} breite={:.3} {:?}",
                        n + 1, z.oben, z.hoehe, z.links, z.breite, z.text
                    );
                }
            }
        }
        for teil in vortraege_trennen(&texte) {
            let texte = &texte[teil.clone()];
            println!("Seiten {}-{}", teil.start + 1, teil.end);
            println!("  Datum: {}", datum_finden(texte).as_deref().unwrap_or("(keines gefunden)"));
            let titel = texte.first().and_then(|z| titel_finden(z));
            println!("  Titel: {}", titel.as_deref().unwrap_or("(keiner gefunden)"));
        }
        return Ok(());
    }

    let tmp = std::env::temp_dir().join(format!("adhs-scan-{}", std::process::id()));
    std::fs::create_dir_all(&tmp)?;
    let seiten = match if opt.aus.is_empty() { scannen(&opt, &tmp) } else { Ok(opt.aus.clone()) } {
        Ok(s) => s,
        Err(e) => {
            std::fs::remove_dir_all(&tmp).ok();
            return Err(e);
        }
    };

    // Die Texterkennung läuft immer: sie liefert nicht nur Datum und Titel,
    // sondern auch die unsichtbare Textebene und die Links im PDF.
    eprintln!("Texterkennung über {} Seiten …", seiten.len());
    let texte = match erkennen(&seiten, opt.dpi) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Hinweis: Texterkennung fehlgeschlagen, PDF ohne Textebene: {e:#}");
            vec![Vec::new(); seiten.len()]
        }
    };
    // Eine Seite ohne jeden Text ist fast immer eine Rückseite: das Blatt lag
    // verkehrt herum im Einzug.
    for (i, zeilen) in texte.iter().enumerate() {
        if zeilen.iter().all(|z| z.text.trim().is_empty()) {
            eprintln!("Achtung: Scan-Seite {} ist leer - lag das Blatt verkehrt herum im Einzug?", i + 1);
        }
    }
    std::fs::create_dir_all(&opt.ordner).with_context(|| format!("{}", opt.ordner.display()))?;

    // Mailadressen, Strassenadressen und Telefonnummern aus den Bildern
    // entfernen - aus dem Text sind sie schon draussen. Ein weisser Kasten im
    // PDF genügte nicht: das Bild darunter liesse sich herausziehen.
    let mut seiten = seiten;
    for (i, zeilen) in texte.iter().enumerate() {
        let kaesten: Vec<[f64; 4]> = zeilen.iter().flat_map(|z| z.schwaerzen.iter().copied()).collect();
        if kaesten.is_empty() {
            continue;
        }
        let ziel = tmp.join(format!("geschwaerzt-{:03}.jpg", i + 1));
        bild_schwaerzen(&seiten[i], &kaesten, &ziel)?;
        eprintln!("Seite {}: {} Stelle(n) entfernt (Mail, Adresse, Telefon)", i + 1, kaesten.len());
        seiten[i] = ziel;
    }

    // Mehrere Vorträge in einem Stapel: je Vortrag ein PDF.
    let teile = vortraege_trennen(&texte);
    if teile.len() > 1 {
        eprintln!("{} Vorträge im Stapel erkannt.", teile.len());
    }
    for teil in teile {
        let ziel = vortrag_ablegen(&opt, &seiten[teil.clone()], &texte[teil.clone()]);
        match ziel {
            Ok(ziel) => println!("{} ({} Seiten)", ziel.display(), teil.len()),
            Err(e) => {
                eprintln!("Die gescannten Seiten liegen in {}", tmp.display());
                return Err(e);
            }
        }
    }
    std::fs::remove_dir_all(&tmp).ok();
    Ok(())
}

/// Ein Vortrag: Datum und Titel bestimmen, PDF schreiben, Pfad zurück.
fn vortrag_ablegen(opt: &Optionen, seiten: &[PathBuf], texte: &[Vec<Zeile>]) -> anyhow::Result<PathBuf> {
    let datum = opt.datum.clone().or_else(|| datum_finden(texte));
    let titel = opt.titel.clone().or_else(|| texte.first().and_then(|z| titel_finden(z)));
    // Ohne Datum im Text gilt der Scantag; im PDF heisst es dann «gescannt
    // am», damit niemand ihn für das Vortragsdatum hält.
    let scandatum = datum.is_none();
    let datum = datum.unwrap_or_else(|| {
        let heute = heute();
        eprintln!("Hinweis: kein Datum im Text - nehme den Scantag {heute}; --datum überschreibt.");
        heute
    });
    let titel = titel.unwrap_or_else(|| {
        eprintln!("Hinweis: kein Titel gefunden - mit --titel nachtragen.");
        "Scan".into()
    });
    eprintln!("Datum: {datum}\nTitel: {titel}");

    // Erkannte Titel bestehen aus mehreren Blöcken: Datum, Veranstalter oder
    // Anlass zuoberst, der eigentliche Titel direkt über der Autorenzeile.
    // In den Namen kommt nur dieser letzte Block, ins PDF der ganze Kopf.
    let name_titel = match &opt.titel {
        Some(t) => t.as_str(),
        None => titel.rsplit(" – ").next().unwrap_or(&titel),
    };
    // Nie überschreiben: bei gleichem Namen eine Nummer anhängen.
    let mut ziel = opt.ordner.join(dateiname(&datum, name_titel, 1));
    let mut n = 2;
    while ziel.exists() {
        ziel = opt.ordner.join(dateiname(&datum, name_titel, n));
        n += 1;
    }
    pdf_bauen(seiten, texte, opt.dpi, &datum, scandatum, &titel, &ziel)?;
    Ok(ziel)
}

/// Die Seitenzahl unten auf der Seite («3», «- 3 -», «Seite 3», «3/7»).
fn seitenzahl(zeilen: &[Zeile]) -> Option<u32> {
    use std::sync::LazyLock;
    static NR: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"(?i)^[-–\s]*(?:seite\s*)?(\d{1,3})(?:\s*(?:/|von)\s*\d{1,3})?[-–\s]*$").unwrap()
    });
    zeilen
        .iter()
        .rev()
        .filter(|z| z.oben > 0.85)
        .find_map(|z| NR.captures(z.text.trim())?[1].parse().ok())
}

/// Zeile ohne alles ausser Buchstaben, zum Vergleich von Kopfzeilen über
/// Lesefehler bei Satzzeichen und Leerstellen hinweg.
fn buchstaben(text: &str) -> String {
    text.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect()
}

/// Ob eine Seite einen Vortrag beginnt, nach dem Kopf: oben steht «Vortrag
/// vom …» oder die Autorenzeile mit Adresse («U. Davatz, www.ganglion.ch»,
/// auch auf zwei Zeilen). Die Unterschrift am Textende («Dr. med. U.
/// Davatz») trägt keine Adresse und zählt nicht. Eine Kopfzeile, die schon
/// auf der Seite davor stand, ist ein laufender Kopf und zählt ebenfalls
/// nicht - taucht dagegen ein neuer laufender Kopf auf, beginnt etwas Neues.
fn beginnt_vortrag(zeilen: &[Zeile], vorher: &[Zeile]) -> bool {
    let kopf_vorher: Vec<String> =
        vorher.iter().filter(|z| z.oben < 0.12).map(|z| buchstaben(&z.text)).collect();
    let oben: Vec<&Zeile> = zeilen.iter().filter(|z| z.oben < 0.35).collect();
    let laufend = |z: &Zeile| z.oben < 0.12 && kopf_vorher.contains(&buchstaben(&z.text));
    let eigen: Vec<String> = oben.iter().filter(|z| !laufend(z)).map(|z| z.text.to_lowercase()).collect();

    let vortrag = oben.iter().any(|z| {
        let t = z.text.to_lowercase();
        (t.starts_with("vortrag") || t.starts_with("votrag")) && !daten_in(&z.text).is_empty()
    });
    let autor = eigen.iter().any(|t| t.contains("davatz"))
        && eigen.iter().any(|t| t.contains("ganglion") || (t.contains("www.") || t.contains("www,")));
    // Neuer laufender Kopf: oben eine Zeile mit Adresse, die vorher fehlte,
    // obwohl die Seite davor einen Kopf hatte oder keinen.
    let neuer_kopf = oben.iter().any(|z| z.oben < 0.12 && !laufend(z) && ist_autorenzeile(&z.text))
        && !kopf_vorher.is_empty();
    vortrag || autor || neuer_kopf
}

/// Teilt einen Stapel in Vorträge. Die Seitenzahl unten entscheidet: 1
/// beginnt einen Vortrag, jede andere setzt ihn fort. Ohne Seitenzahl gilt
/// der Kopf (`beginnt_vortrag`). Lücken in der Zählung werden gemeldet - nach
/// einem Papierstau fehlt sonst still eine Seite.
fn vortraege_trennen(texte: &[Vec<Zeile>]) -> Vec<std::ops::Range<usize>> {
    let nummern: Vec<Option<u32>> = texte.iter().map(|z| seitenzahl(z)).collect();
    let mut starts = vec![0];
    for i in 1..texte.len() {
        let neu = match nummern[i] {
            Some(1) => true,
            Some(_) => false,
            None => beginnt_vortrag(&texte[i], &texte[i - 1]),
        };
        if neu {
            starts.push(i);
        } else if let (Some(a), Some(b)) = (nummern[i - 1], nummern[i]) {
            if b != a + 1 {
                eprintln!("Hinweis: Scan-Seite {} trägt Seitenzahl {b}, die davor {a} - fehlt eine Seite?", i + 1);
            }
        }
    }
    starts.push(texte.len());
    starts.windows(2).map(|w| w[0]..w[1]).filter(|r| !r.is_empty()).collect()
}

/// Eine erkannte Textzeile. `oben` ist der Abstand der Oberkante vom oberen
/// Seitenrand, `hoehe` die Zeilenhöhe, beides als Anteil der Seitenhöhe;
/// `links` und `breite` entsprechend als Anteil der Seitenbreite.
/// `schwaerzen` sind Kästen (links, oben, breite, hoehe, gleich normiert)
/// mit Mailadressen, Strassenadressen oder Telefonnummern: sie werden im
/// Bild übermalt und stehen in `text` schon nicht mehr.
#[derive(Debug, Clone, Default)]
struct Zeile {
    text: String,
    oben: f64,
    hoehe: f64,
    links: f64,
    breite: f64,
    schwaerzen: Vec<[f64; 4]>,
    /// Die Wörter der Zeile mit ihrer Lage (links, breite); leer heisst:
    /// die Zeile wird als Ganzes gesetzt.
    woerter: Vec<Wort>,
}

#[derive(Debug, Clone, Default)]
struct Wort {
    text: String,
    links: f64,
    breite: f64,
}

/// Stellen, die nicht ins öffentliche PDF gehören: Mailadressen,
/// Strassenadressen mit Hausnummer, Telefonnummern. Byte-Bereiche in `text`.
fn heikle_stellen(text: &str) -> Vec<std::ops::Range<usize>> {
    use std::sync::LazyLock;
    static HEIKEL: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(concat!(
            r"(?i)[a-z0-9._%+-]+\s?@\s?[a-z0-9.-]+\.[a-z]{2,}",
            r"|\b[A-ZÄÖÜ][\wäöüé-]*(?:strasse|straße|str\.|gasse|weg|platz|rain|halde|allee)\s*\d+\s?[a-z]?\b",
            r"|(?:\+41|\b0)\s?\(?\d{2}\)?[\s/]?\d{3}[\s.]?\d{2}[\s.]?\d{2}\b",
        ))
        .unwrap()
    });
    HEIKEL.find_iter(text).map(|m| m.range()).collect()
}

/// Übermalt die Kästen weiss und schreibt das Bild als JPEG nach `ziel`.
fn bild_schwaerzen(quelle: &Path, kaesten: &[[f64; 4]], ziel: &Path) -> anyhow::Result<()> {
    let mut bild = image::open(quelle).with_context(|| format!("{}", quelle.display()))?;
    let (b, h) = (bild.width() as f64, bild.height() as f64);
    for &[links, oben, breite, hoehe] in kaesten {
        // Etwas Rand, damit keine angeschnittenen Buchstaben stehen bleiben
        let x0 = ((links - 0.004) * b).max(0.0) as u32;
        let x1 = ((links + breite + 0.004) * b).min(b) as u32;
        let y0 = ((oben - 0.15 * hoehe) * h).max(0.0) as u32;
        let y1 = ((oben + 1.05 * hoehe) * h).min(h) as u32;
        match &mut bild {
            image::DynamicImage::ImageLuma8(g) => {
                for y in y0..y1 {
                    for x in x0..x1 {
                        g.put_pixel(x, y, image::Luma([255]));
                    }
                }
            }
            andere => {
                let mut rgb = andere.to_rgb8();
                for y in y0..y1 {
                    for x in x0..x1 {
                        rgb.put_pixel(x, y, image::Rgb([255, 255, 255]));
                    }
                }
                *andere = image::DynamicImage::ImageRgb8(rgb);
            }
        }
    }
    let datei = std::fs::File::create(ziel).with_context(|| format!("{}", ziel.display()))?;
    let mut kodierer =
        image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(datei), 85);
    kodierer.encode_image(&bild)?;
    Ok(())
}


/// Datum und Titel aus den Seitenbildern.
/// Die erkannten Zeilen jeder Seite.
fn erkennen(seiten: &[PathBuf], dpi: u32) -> anyhow::Result<Vec<Vec<Zeile>>> {
    seiten.iter().map(|s| ocr::zeilen(s, dpi)).collect()
}

/// Texterkennung mit Tesseract (`brew install tesseract tesseract-lang`).
///
/// Bis Oktober 2026 lief hier Apples Vision. Aus einem Rust-Programm heraus
/// liess es auf manchen Seiten ganze Zeilen aus - 15 statt 23 auf derselben
/// Seite, die ein Swift-Programm vollständig las, bei gleichen Einstellungen
/// und Revision 3. Fehlende Zeilen fehlen in der Textebene, und «alles
/// markieren» verpasst Text. Tesseract liest alle Zeilen und liefert dazu die
/// Lage jedes Wortes.
mod ocr {
    use anyhow::Context;

    /// Zeilen einer Seite aus der TSV-Ausgabe von Tesseract.
    pub fn zeilen(bild: &std::path::Path, dpi: u32) -> anyhow::Result<Vec<super::Zeile>> {
        let aus = std::process::Command::new("tesseract")
            .arg(bild)
            .args(["stdout", "-l", "deu", "--dpi", &dpi.to_string(), "--psm", "3", "tsv"])
            .output()
            .context("tesseract nicht gefunden - brew install tesseract tesseract-lang")?;
        if !aus.status.success() {
            anyhow::bail!(
                "tesseract {}: {}",
                bild.display(),
                String::from_utf8_lossy(&aus.stderr).trim()
            );
        }
        Ok(tsv_lesen(&String::from_utf8_lossy(&aus.stdout)))
    }

    /// TSV-Spalten: level page block par line word left top width height conf text.
    /// Level 1 ist die Seite (liefert die Bildgrösse), Level 5 ein Wort.
    pub fn tsv_lesen(tsv: &str) -> Vec<super::Zeile> {
        let (mut bb, mut bh) = (1.0f64, 1.0f64);
        // Wörter je Zeile sammeln, Schlüssel (block, par, line)
        let mut gruppen: Vec<((u32, u32, u32), Vec<(String, f64, f64, f64, f64)>)> = Vec::new();
        for z in tsv.lines().skip(1) {
            let f: Vec<&str> = z.split('\t').collect();
            if f.len() < 12 {
                continue;
            }
            let zahl = |i: usize| f[i].parse::<f64>().unwrap_or(0.0);
            match f[0] {
                "1" => {
                    bb = zahl(8).max(1.0);
                    bh = zahl(9).max(1.0);
                }
                "5" => {
                    let text = f[11].trim();
                    if text.is_empty() {
                        continue;
                    }
                    let schluessel = (zahl(2) as u32, zahl(3) as u32, zahl(4) as u32);
                    let wort = (text.to_string(), zahl(6), zahl(7), zahl(8), zahl(9));
                    match gruppen.last_mut() {
                        Some((k, w)) if *k == schluessel => w.push(wort),
                        _ => gruppen.push((schluessel, vec![wort])),
                    }
                }
                _ => {}
            }
        }

        let mut zeilen = Vec::new();
        for (_, mut woerter) in gruppen {
            // Kontaktdaten: betroffene Wörter fallen ganz heraus, ihr Kasten
            // wird im Bild übermalt.
            let text: String = woerter.iter().map(|w| w.0.as_str()).collect::<Vec<_>>().join(" ");
            let stellen = super::heikle_stellen(&text);
            let mut schwaerzen = Vec::new();
            if !stellen.is_empty() {
                let mut ab = 0;
                let mut behalten = Vec::new();
                for w in woerter {
                    let bereich = ab..ab + w.0.len();
                    ab = bereich.end + 1;
                    if stellen.iter().any(|st| st.start < bereich.end && bereich.start < st.end) {
                        schwaerzen.push([w.1 / bb, w.2 / bh, w.3 / bb, w.4 / bh]);
                    } else {
                        behalten.push(w);
                    }
                }
                woerter = behalten;
                // Übrig gebliebene Trennstriche am Rand
                while woerter.last().is_some_and(|w| w.0 == "-" || w.0 == "–") {
                    woerter.pop();
                }
            }
            let links = woerter.iter().map(|w| w.1).fold(f64::MAX, f64::min);
            let rechts = woerter.iter().map(|w| w.1 + w.3).fold(0.0, f64::max);
            let oben = woerter.iter().map(|w| w.2).fold(f64::MAX, f64::min);
            let unten = woerter.iter().map(|w| w.2 + w.4).fold(0.0, f64::max);
            let zeile = super::Zeile {
                text: woerter.iter().map(|w| w.0.as_str()).collect::<Vec<_>>().join(" "),
                oben: if woerter.is_empty() { 0.0 } else { oben / bh },
                hoehe: if woerter.is_empty() { 0.0 } else { (unten - oben) / bh },
                links: if woerter.is_empty() { 0.0 } else { links / bb },
                breite: if woerter.is_empty() { 0.0 } else { (rechts - links) / bb },
                schwaerzen,
                woerter: woerter
                    .iter()
                    .map(|w| super::Wort { text: w.0.clone(), links: w.1 / bb, breite: w.3 / bb })
                    .collect(),
            };
            if !zeile.text.is_empty() || !zeile.schwaerzen.is_empty() {
                zeilen.push(zeile);
            }
        }
        zeilen.sort_by(|a, b| a.oben.total_cmp(&b.oben));
        zeilen
    }
}

const MONATE: [&str; 12] = [
    "januar", "februar", "märz", "april", "mai", "juni", "juli", "august", "september",
    "oktober", "november", "dezember",
];

/// Alle Daten in einer Zeile, als `JJJJ-MM-TT` (oder `JJJJ-MM`, wenn nur
/// Monat und Jahr dastehen). Zweistellige Jahre: bis 30 → 20xx.
fn daten_in(text: &str) -> Vec<String> {
    use std::sync::LazyLock;
    static ZAHL: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"\b(\d{1,2})\.\s?(\d{1,2})\.\s?(\d{4}|\d{2})\b").unwrap()
    });
    static WORT: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"(?i)\b(?:(\d{1,2})\.\s*)?(januar|jan\.|februar|feb\.|märz|maerz|april|mai|juni|juli|august|aug\.|september|sept?\.|oktober|okt\.|november|nov\.|dezember|dez\.)\s+(\d{4})\b",
        )
        .unwrap()
    });
    let mut daten = Vec::new();
    for c in ZAHL.captures_iter(text) {
        let mut j: u32 = c[3].parse().unwrap();
        if c[3].len() == 2 {
            j += if j <= 30 { 2000 } else { 1900 };
        }
        if let Some(d) = datum_iso(&format!("{j}-{}-{}", &c[2], &c[1])) {
            daten.push(d);
        }
    }
    for c in WORT.captures_iter(text) {
        let wort = c[2].to_lowercase().replace("maerz", "märz");
        let Some(m) = MONATE.iter().position(|n| n.starts_with(wort.trim_end_matches('.'))) else {
            continue;
        };
        let j: u32 = c[3].parse().unwrap();
        match c.get(1) {
            Some(t) => daten.extend(datum_iso(&format!("{j}-{}-{}", m + 1, t.as_str()))),
            None if (1900..=2100).contains(&j) => daten.push(format!("{j:04}-{:02}", m + 1)),
            None => {}
        }
    }
    daten
}

/// Das Datum des Vortrags: im Kopf der ersten Seite (oberes Drittel), sonst
/// das letzte Datum im Dokument.
fn datum_finden(seiten: &[Vec<Zeile>]) -> Option<String> {
    let kopf = seiten
        .first()?
        .iter()
        .filter(|z| z.oben < 0.33)
        .find_map(|z| daten_in(&z.text).into_iter().next());
    kopf.or_else(|| {
        seiten.iter().flatten().flat_map(|z| daten_in(&z.text)).last()
    })
}

/// Zeilen, die zum Kopf gehören, aber nicht zum Titel: Autor, Adresse, Datum.
fn ist_autorenzeile(text: &str) -> bool {
    let t = text.to_lowercase();
    t.contains("davatz") || t.contains("ganglion") || (t.contains("www.") || t.contains("www,")) || t.contains('@')
}

/// Der Titel: die Zeilen oben auf der ersten Seite bis zur Autorenzeile.
/// Fehlt sie, der erste Block bis zur ersten grösseren Lücke. Blöcke werden
/// mit « – » verbunden, Trennstriche am Zeilenende aufgelöst.
fn titel_finden(zeilen: &[Zeile]) -> Option<String> {
    let kopf: Vec<&Zeile> = zeilen.iter().filter(|z| z.oben < 0.4).take(8).collect();
    let ende = kopf.iter().position(|z| ist_autorenzeile(&z.text));
    let mut zeilen: Vec<&Zeile> = match ende {
        Some(0) | None => {
            // erster Block nach einer eventuellen Autorenzeile
            let start = ende.map_or(0, |_| 1);
            let mut block = vec![*kopf.get(start)?];
            for z in &kopf[start + 1..] {
                let vorige = block.last().unwrap();
                if z.oben - (vorige.oben + vorige.hoehe) > 1.5 * vorige.hoehe {
                    break;
                }
                block.push(z);
            }
            block
        }
        Some(e) => kopf[..e].to_vec(),
    };
    zeilen.retain(|z| daten_in(&z.text).is_empty() || z.text.len() > 40);

    let mut titel = String::new();
    let mut vorige: Option<&Zeile> = None;
    for z in zeilen {
        let text = z.text.trim();
        if let Some(v) = vorige {
            // Eine Zeile, die klein beginnt («zur Prävention beitragen»),
            // setzt die vorige fort, auch über eine Lücke hinweg: Tesseract
            // misst die Zeilen enger als Vision, Titelzeilen mit normalem
            // Zeilenabstand sähen sonst wie getrennte Blöcke aus.
            let klein = text.starts_with(|c: char| c.is_lowercase());
            let luecke = !klein && z.oben - (v.oben + v.hoehe) > 0.7 * v.hoehe;
            if luecke {
                titel.push_str(" – ");
            } else if titel.ends_with('-') && text.starts_with(|c: char| c.is_lowercase()) {
                titel.pop();
            } else {
                titel.push(' ');
            }
        }
        titel.push_str(text);
        vorige = Some(z);
    }
    let titel = titel.split_whitespace().collect::<Vec<_>>().join(" ");
    (!titel.is_empty()).then_some(titel)
}

/// `JJJJ-MM-TT` als `TT.MM.JJJJ`, `JJJJ-MM` als `MM.JJJJ`; alles andere
/// bleibt, wie es ist.
fn datum_ch(datum: &str) -> String {
    match datum.split('-').collect::<Vec<_>>()[..] {
        [j, m, t] if datum_iso(datum).is_some() => format!("{t}.{m}.{j}"),
        [j, m] if datum_iso(&format!("{datum}-01")).is_some() => format!("{m}.{j}"),
        _ => datum.to_string(),
    }
}

/// Das heutige Datum als `JJJJ-MM-TT`, in der Ortszeit des Rechners.
fn heute() -> String {
    Command::new("date")
        .arg("+%F")
        .output()
        .ok()
        .and_then(|a| datum_iso(String::from_utf8_lossy(&a.stdout).trim()))
        .unwrap_or_else(|| "ohne Datum".into())
}

/// Wandelt `JJJJ-MM-TT` oder `TT.MM.JJJJ` in `JJJJ-MM-TT`.
fn datum_iso(s: &str) -> Option<String> {
    let (j, m, t) = if let Some((t, rest)) = s.split_once('.') {
        let (m, j) = rest.split_once('.')?;
        (j, m, t)
    } else {
        let mut it = s.splitn(3, '-');
        (it.next()?, it.next()?, it.next()?)
    };
    let (j, m, t): (u32, u32, u32) = (j.parse().ok()?, m.parse().ok()?, t.parse().ok()?);
    if !(1900..=2100).contains(&j) || !(1..=12).contains(&m) || !(1..=31).contains(&t) {
        return None;
    }
    Some(format!("{j:04}-{m:02}-{t:02}"))
}

/// Längster Titelteil im Dateinamen; der volle Titel steht in den
/// Dokumenteigenschaften.
const MAX_TITEL_IM_NAMEN: usize = 60;

/// Dateiname aus Titel und Datum, etwa `Alkohol_und_Familie_20.11.2003.pdf`:
/// Titel zuerst, höchstens `MAX_TITEL_IM_NAMEN` Zeichen, an einer Wortgrenze
/// gekürzt, Unterstriche statt Leerschlägen, Datum am Schluss. Was in
/// Dateinamen stört (/ : \\ ? * " < > |), fällt weg oder wird zum Strich.
/// `nr` ab 2 unterscheidet gleiche Namen (`…_20.11.2003_2.pdf`).
fn dateiname(datum: &str, titel: &str, nr: u32) -> String {
    let titel: String = titel
        .chars()
        .filter(|c| !matches!(c, '?' | '*' | '"' | '<' | '>' | '|'))
        .map(|c| match c {
            '/' | ':' | '\\' => '-',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    let mut kurz = String::new();
    for wort in titel.split_whitespace().filter(|w| *w != "-" && *w != "–") {
        let laenge = kurz.chars().count() + 1 + wort.chars().count();
        if !kurz.is_empty() && laenge > MAX_TITEL_IM_NAMEN {
            break;
        }
        if !kurz.is_empty() {
            kurz.push('_');
        }
        kurz.push_str(wort);
    }
    let kurz: String = kurz.chars().take(MAX_TITEL_IM_NAMEN).collect();
    let kurz = kurz.trim_end_matches([',', ';', '-', '_', '.']);
    let datum = datum_ch(datum).replace(' ', "_");
    match nr {
        0 | 1 => format!("{kurz}_{datum}.pdf"),
        n => format!("{kurz}_{datum}_{n}.pdf"),
    }
}

/// Breite, Höhe und Farbkanäle aus dem SOF-Segment eines JPEG.
fn jpeg_mass(daten: &[u8]) -> Option<(u32, u32, u8)> {
    if daten.get(..2)? != [0xFF, 0xD8] {
        return None;
    }
    let mut i = 2;
    while i + 4 <= daten.len() {
        if daten[i] != 0xFF {
            return None;
        }
        let marker = daten[i + 1];
        if marker == 0xFF {
            i += 1;
            continue;
        }
        let laenge = u16::from_be_bytes([daten[i + 2], daten[i + 3]]) as usize;
        // SOF0..SOF15 ausser DHT (C4), JPG (C8) und DAC (CC)
        if (0xC0..=0xCF).contains(&marker) && ![0xC4, 0xC8, 0xCC].contains(&marker) {
            let s = daten.get(i + 4..i + 4 + 6)?;
            let hoehe = u16::from_be_bytes([s[1], s[2]]) as u32;
            let breite = u16::from_be_bytes([s[3], s[4]]) as u32;
            return Some((breite, hoehe, s[5]));
        }
        i += 2 + laenge;
    }
    None
}

/// PDF-Textzeichenkette: ASCII direkt, sonst UTF-16BE mit BOM.
fn pdf_text(s: &str) -> lopdf::Object {
    if s.is_ascii() {
        return lopdf::Object::string_literal(s);
    }
    let mut b = vec![0xFE, 0xFF];
    for u in s.encode_utf16() {
        b.extend_from_slice(&u.to_be_bytes());
    }
    lopdf::Object::String(b, lopdf::StringFormat::Hexadecimal)
}

/// Breite eines Zeichens in Helvetica, in Tausendsteln der Schriftgrösse.
/// Akzente und Umlaute laufen so breit wie ihr Grundbuchstabe.
fn helvetica_breite(c: char) -> f64 {
    let basis = match c {
        'ä' | 'à' | 'á' | 'â' => 'a',
        'ö' | 'ò' | 'ó' | 'ô' => 'o',
        'ü' | 'ù' | 'ú' | 'û' => 'u',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'ï' | 'î' | 'í' | 'ì' => 'i',
        'Ä' => 'A',
        'Ö' => 'O',
        'Ü' => 'U',
        'É' | 'È' => 'E',
        'ç' => 'c',
        c => c,
    };
    (match basis {
        'i' | 'j' | 'l' | '\'' => 222,
        ' ' | 'f' | 't' | 'I' | '.' | ',' | ':' | ';' | '/' | '!' | '[' | ']' => 278,
        'r' | '-' | '(' | ')' | '«' | '»' => 333,
        'c' | 'k' | 's' | 'v' | 'x' | 'y' | 'z' | 'J' | '"' => 500,
        'F' | 'T' | 'Z' | 'ß' => 611,
        'A' | 'B' | 'E' | 'K' | 'P' | 'S' | 'V' | 'X' | 'Y' | '&' => 667,
        'C' | 'D' | 'H' | 'N' | 'R' | 'U' | 'w' => 722,
        'G' | 'O' | 'Q' => 778,
        'm' | 'M' => 833,
        'W' => 944,
        '@' => 1015,
        'L' => 556,
        _ => 556,
    }) as f64
}

/// Text in WinAnsi (Windows-1252), Klammern und Backslash maskiert.
fn winansi(text: &str) -> Vec<u8> {
    let mut b = Vec::new();
    for c in text.chars() {
        let byte = match c {
            '(' | ')' | '\\' => {
                b.push(b'\\');
                c as u8
            }
            c if (c as u32) < 0x80 && !c.is_control() => c as u8,
            c if (0xA0..=0xFF).contains(&(c as u32)) => c as u32 as u8,
            '€' => 0x80,
            '‚' => 0x82,
            '„' => 0x84,
            '…' => 0x85,
            '‘' => 0x91,
            '’' => 0x92,
            '“' => 0x93,
            '”' => 0x94,
            '•' => 0x95,
            '–' => 0x96,
            '—' => 0x97,
            _ => b'?',
        };
        b.push(byte);
    }
    b
}

/// Unsichtbarer Text (Rendermodus 3) genau über den erkannten Zeilen, damit
/// sich das PDF markieren, kopieren und durchsuchen lässt. Jede Zeile wird
/// über die horizontale Skalierung (Tz) auf die Breite ihres Kastens
/// gezogen, damit die Markierung mit dem Bild übereinstimmt.
fn textebene(zeilen: &[Zeile], w: f64, hh: f64) -> Vec<u8> {
    let mut s = Vec::new();
    if zeilen.is_empty() {
        return s;
    }
    s.extend_from_slice(b"BT 3 Tr\n");
    for z in zeilen.iter().filter(|z| !z.woerter.is_empty()) {
        // Wort für Wort, jedes auf seine eigene Breite gezogen; ein
        // Leerzeichen dahinter, damit beim Kopieren Wortabstände entstehen.
        let groesse = z.hoehe * hh / 1.15;
        let grundlinie = (1.0 - z.oben - z.hoehe) * hh + 0.22 * groesse;
        let anzahl = z.woerter.len();
        for (i, wort) in z.woerter.iter().enumerate() {
            let einheiten: f64 = wort.text.chars().map(helvetica_breite).sum();
            if einheiten <= 0.0 || groesse <= 0.0 {
                continue;
            }
            let tz = 100.0 * wort.breite * w / (einheiten / 1000.0 * groesse);
            let x = wort.links * w;
            s.extend(
                format!("/F1 {groesse:.2} Tf {tz:.1} Tz 1 0 0 1 {x:.2} {grundlinie:.2} Tm (").into_bytes(),
            );
            s.extend(winansi(&wort.text));
            if i + 1 < anzahl {
                s.push(b' ');
            }
            s.extend_from_slice(b") Tj\n");
        }
    }
    for z in zeilen.iter().filter(|z| z.woerter.is_empty()) {
        let text = z.text.trim();
        let einheiten: f64 = text.chars().map(helvetica_breite).sum();
        if text.is_empty() || einheiten <= 0.0 {
            continue;
        }
        // Der Kasten umfasst Ober- und Unterlängen, gut 1.15 Schriftgrössen.
        let groesse = z.hoehe * hh / 1.15;
        let x = z.links * w;
        let grundlinie = (1.0 - z.oben - z.hoehe) * hh + 0.22 * groesse;
        let tz = 100.0 * z.breite * w / (einheiten / 1000.0 * groesse);
        s.extend(
            format!("/F1 {groesse:.2} Tf {tz:.1} Tz 1 0 0 1 {x:.2} {grundlinie:.2} Tm (").into_bytes(),
        );
        s.extend(winansi(text));
        s.extend_from_slice(b") Tj\n");
    }
    s.extend_from_slice(b"ET\n");
    s
}

/// Adressen in den erkannten Zeilen, mit ihrer Fläche auf der Seite
/// (x1, y1, x2, y2 in Punkten). Die Lage innerhalb der Zeile wird über die
/// Zeichenbreiten bemessen, wie bei der Textebene.
fn links_finden(zeilen: &[Zeile], w: f64, hh: f64) -> Vec<(String, [f32; 4])> {
    use std::sync::LazyLock;
    static URL: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"(?i)\b(?:https?://[^\s<>]+|www[.,][^\s<>]+|[a-z0-9][a-z0-9.-]*\.(?:ch|com|de|org|net|li|at)\b[^\s<>]*|[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,})",
        )
        .unwrap()
    });
    let mut links = Vec::new();
    for z in zeilen {
        let text = z.text.trim();
        let gesamt: f64 = text.chars().map(helvetica_breite).sum();
        if gesamt <= 0.0 {
            continue;
        }
        for m in URL.find_iter(text) {
            let roh = m.as_str().trim_end_matches(['.', ',', ';', ':', ')', '»', '"']);
            // E-Mail-Adressen: das Muster für Domains darf sie nicht zerschneiden
            if roh.contains('@') && !roh.contains("://") {
                if !m.as_str().starts_with(roh) || roh.starts_with('@') {
                    continue;
                }
            }
            let ziel = if roh.contains('@') && !roh.contains("://") {
                format!("mailto:{roh}")
            } else if roh.to_lowercase().starts_with("http") {
                roh.to_string()
            } else {
                // Tesseract liest den Punkt nach «www» gern als Komma
                format!("https://{}", roh.replacen("www,", "www.", 1))
            };
            let vor: f64 = text[..m.start()].chars().map(helvetica_breite).sum();
            let lang: f64 = roh.chars().map(helvetica_breite).sum();
            let x1 = (z.links + z.breite * vor / gesamt) * w;
            let x2 = (z.links + z.breite * (vor + lang) / gesamt) * w;
            let y1 = (1.0 - z.oben - z.hoehe) * hh;
            let y2 = (1.0 - z.oben) * hh;
            links.push((ziel, [x1 as f32, y1 as f32, x2 as f32, y2 as f32]));
        }
    }
    links
}

/// Setzt die gescannten JPEG-Seiten zu einem PDF zusammen.
fn pdf_bauen(
    seiten: &[PathBuf],
    texte: &[Vec<Zeile>],
    dpi: u32,
    datum: &str,
    scandatum: bool,
    titel: &str,
    ziel: &Path,
) -> anyhow::Result<()> {
    use lopdf::{dictionary, Document, Object, Stream};

    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    // Helvetica gehört zu den 14 Standardschriften und muss nicht eingebettet
    // werden; die Textebene ist ohnehin unsichtbar.
    let schrift_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
        "Encoding" => "WinAnsiEncoding",
    });
    let mut kids = Vec::new();

    for (n, pfad) in seiten.iter().enumerate() {
        let daten = std::fs::read(pfad).with_context(|| format!("{}", pfad.display()))?;
        let (b, h, kanaele) = jpeg_mass(&daten)
            .with_context(|| format!("kein lesbares JPEG: {}", pfad.display()))?;
        let farbraum = match kanaele {
            1 => "DeviceGray",
            4 => "DeviceCMYK",
            _ => "DeviceRGB",
        };
        let mut bild = Stream::new(
            dictionary! {
                "Type" => "XObject",
                "Subtype" => "Image",
                "Width" => b as i64,
                "Height" => h as i64,
                "ColorSpace" => farbraum,
                "BitsPerComponent" => 8,
                "Filter" => "DCTDecode",
            },
            daten,
        );
        bild.allows_compression = false;
        let bild_id = doc.add_object(bild);

        let w = b as f32 * 72.0 / dpi as f32;
        let hh = h as f32 * 72.0 / dpi as f32;
        let zeilen = texte.get(n).map(Vec::as_slice).unwrap_or(&[]);
        let mut inhalt = format!("q {w:.2} 0 0 {hh:.2} 0 0 cm /Im0 Do Q\n").into_bytes();
        inhalt.extend(textebene(zeilen, w as f64, hh as f64));
        let inhalt_id = doc.add_object(Stream::new(dictionary! {}, inhalt));

        let mut annots = Vec::new();
        for (url, [x1, y1, x2, y2]) in links_finden(zeilen, w as f64, hh as f64) {
            annots.push(Object::Reference(doc.add_object(dictionary! {
                "Type" => "Annot",
                "Subtype" => "Link",
                "Rect" => vec![x1.into(), y1.into(), x2.into(), y2.into()],
                "Border" => vec![0.into(), 0.into(), 0.into()],
                "A" => dictionary! { "S" => "URI", "URI" => Object::string_literal(url) },
            })));
        }
        let mut seite = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), Object::Real(w), Object::Real(hh)],
            "Resources" => dictionary! {
                "XObject" => dictionary! { "Im0" => bild_id },
                "Font" => dictionary! { "F1" => schrift_id },
            },
            "Contents" => inhalt_id,
        };
        if !annots.is_empty() {
            seite.set("Annots", annots);
        }
        let seite_id = doc.add_object(seite);
        kids.push(seite_id.into());
        eprintln!("  Seite {} – {b}×{h} px", n + 1);
    }

    let anzahl = kids.len() as i64;
    doc.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => kids, "Count" => anzahl }),
    );
    let katalog_id = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    doc.trailer.set("Root", katalog_id);

    let art = if scandatum { "gescannt am" } else { "Vortrag vom" };
    let vom = match (datum.get(..4), datum.get(5..7), datum.get(8..10)) {
        (Some(j), Some(m), Some(t)) if datum_iso(datum).is_some() => format!("{art} {t}.{m}.{j}"),
        (Some(j), Some(m), None) if datum_iso(&format!("{datum}-01")).is_some() => {
            format!("{art} {m}.{j}")
        }
        _ => "Vortrag".to_string(),
    };
    let info_id = doc.add_object(dictionary! {
        "Title" => pdf_text(titel),
        "Subject" => pdf_text(&vom),
        "Keywords" => pdf_text(&format!("{datum}; {titel}")),
        "Creator" => pdf_text("adhs-expert scannen (Epson GT-2500, Einzug)"),
    });
    doc.trailer.set("Info", info_id);

    doc.save(ziel).with_context(|| format!("{}", ziel.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datum() {
        assert_eq!(datum_iso("2014-03-05").as_deref(), Some("2014-03-05"));
        assert_eq!(datum_iso("5.3.2014").as_deref(), Some("2014-03-05"));
        assert_eq!(datum_iso("2014-13-05"), None);
        assert_eq!(datum_iso("gestern"), None);
    }

    #[test]
    fn daten_im_text() {
        assert_eq!(daten_in("Zürich, 5. März 2014"), vec!["2014-03-05"]);
        assert_eq!(daten_in("Basel, 12.11.03"), vec!["2003-11-12"]);
        assert_eq!(daten_in("im Oktober 2021 gehalten"), vec!["2021-10"]);
        assert!(daten_in("Seite 3 von 12").is_empty());
    }

    #[test]
    fn links() {
        let z = |t: &str| Zeile { text: t.into(), oben: 0.1, hoehe: 0.01, links: 0.1, breite: 0.5, ..Default::default() };
        let ziele = |t: &str| -> Vec<String> {
            links_finden(&[z(t)], 595.0, 842.0).into_iter().map(|(u, _)| u).collect()
        };
        assert_eq!(ziele("U. Davatz, www.ganglion.ch"), vec!["https://www.ganglion.ch"]);
        assert_eq!(ziele("siehe http://schizo.li."), vec!["http://schizo.li"]);
        assert_eq!(ziele("Mail: info@ganglion.ch"), vec!["mailto:info@ganglion.ch"]);
        assert!(ziele("Dr. med. U. Davatz").is_empty());
    }

    #[test]
    fn trennen() {
        let z = |t: &str, oben: f64| Zeile { text: t.into(), oben, hoehe: 0.01, links: 0.1, breite: 0.5, ..Default::default() };
        let texte = vec![
            vec![z("Vortrag vom 2.12.2003", 0.05), z("U. Davatz, www.ganglion.ch", 0.15)],
            vec![z("Fliesstext", 0.1), z("Dr. med. U. Davatz", 0.25)],
            vec![z("Votrag vom 20.11.2003", 0.05), z("Alkohol und Familie", 0.1)],
            vec![z("Fliesstext", 0.1)],
        ];
        assert_eq!(vortraege_trennen(&texte), vec![0..2, 2..4]);
    }

    #[test]
    fn trennen_mit_laufendem_kopf_und_seitenzahlen() {
        let z = |t: &str, oben: f64| Zeile { text: t.into(), oben, hoehe: 0.01, links: 0.1, breite: 0.5, ..Default::default() };
        let kopf = "Ganglion Frau Dr. med. Ursula Davatz - www.ganglion.ch";
        let texte = vec![
            vec![z(kopf, 0.03), z("Alkohol und Familie", 0.1), z("1", 0.95)],
            vec![z(&kopf.replace(" - ", "-"), 0.03), z("Fliesstext", 0.1), z("2", 0.95)],
            vec![z(kopf, 0.03), z("Fliesstext", 0.1)],
            vec![z("Familiendynamik", 0.05), z("Dr. med. U. Davatz", 0.1), z("www.ganglion.ch", 0.12)],
            vec![z("Fliesstext", 0.1), z("- 2 -", 0.95)],
        ];
        assert_eq!(vortraege_trennen(&texte), vec![0..3, 3..5]);
        assert_eq!(seitenzahl(&[z("Seite 3 von 7", 0.96)]), Some(3));
        assert_eq!(seitenzahl(&[z("2003", 0.5)]), None);
    }

    /// Eine Zeile als Tesseract-TSV, ein Wort je 100 px breit.
    fn tsv(woerter: &[&str]) -> String {
        let mut t = String::from("level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\twidth\theight\tconf\ttext\n");
        t.push_str("1\t1\t0\t0\t0\t0\t0\t0\t2000\t3000\t-1\t\n");
        for (i, w) in woerter.iter().enumerate() {
            t.push_str(&format!("5\t1\t1\t1\t1\t{}\t{}\t100\t90\t30\t95\t{w}\n", i + 1, 100 + i * 100));
        }
        t
    }

    #[test]
    fn heikles_entfernen() {
        let z = ocr::tsv_lesen(&tsv(&[
            "Ganglion", "Frau", "Dr.", "med.", "Ursula", "Davatz", "-", "www.ganglion.ch", "-",
            "ursula.davatz@ganglion.ch",
        ]));
        assert_eq!(z[0].text, "Ganglion Frau Dr. med. Ursula Davatz - www.ganglion.ch");
        assert_eq!(z[0].schwaerzen.len(), 1);
        assert!((z[0].schwaerzen[0][0] - 1000.0 / 2000.0).abs() < 1e-9);
        let z = ocr::tsv_lesen(&tsv(&["der", "Mäderstrasse", "13", "in", "Baden"]));
        assert_eq!(z[0].text, "der in Baden");
        assert_eq!(z[0].schwaerzen.len(), 2);
        assert_eq!(heikle_stellen("Tel. 056 222 33 44").len(), 1);
        assert!(heikle_stellen("Vortrag vom 20.11.2003, Seite 3 von 7").is_empty());
        assert!(heikle_stellen("die Strasse und der Weg").is_empty());
    }

    #[test]
    fn titel_ueber_mehrere_zeilen() {
        let z = |t: &str, oben: f64| Zeile { text: t.into(), oben, hoehe: 0.014, ..Default::default() };
        let zeilen = vec![
            z("Artikel über Prävention im Clic", 0.093),
            z("über das Thema", 0.122),
            z("Was und wieviel kann die Mütter- und Väterberatung", 0.151),
            z("zur Prävention beitragen", 0.180),
            z("U. Davatz, www.ganglion.ch", 0.226),
        ];
        let titel = titel_finden(&zeilen).unwrap();
        assert_eq!(
            titel.rsplit(" – ").next().unwrap(),
            "Was und wieviel kann die Mütter- und Väterberatung zur Prävention beitragen"
        );
    }

    #[test]
    fn name() {
        assert_eq!(
            dateiname("2003-06-01", "Cannabis und  Schizophrenie: ein Überblick", 1),
            "Cannabis_und_Schizophrenie-_ein_Überblick_01.06.2003.pdf"
        );
        assert_eq!(
            dateiname("2003-12-02", "Jugendliche und Sucht Freiheit oder Sackgasse auf dem Weg der Suchenden", 2),
            "Jugendliche_und_Sucht_Freiheit_oder_Sackgasse_auf_dem_Weg_02.12.2003_2.pdf"
        );
        let lang = dateiname("2014-03-05", &"Wort ".repeat(30), 1);
        assert!(lang.chars().count() <= "_05.03.2014.pdf".len() + MAX_TITEL_IM_NAMEN);
        assert_eq!(dateiname("2021-10", "Titel", 1), "Titel_10.2021.pdf");
        assert_eq!(
            dateiname("2003-11-11", "Das trotzende Kind - Bin ich zu lieb oder bin ich zu streng?", 1),
            "Das_trotzende_Kind_Bin_ich_zu_lieb_oder_bin_ich_zu_streng_11.11.2003.pdf"
        );
    }
}
