// SPDX-License-Identifier: GPL-3.0-or-later
//
// Scannt einen Vortrag über den automatischen Einzug des Epson GT-2500 und
// legt ein PDF an, das Datum und Titel des Vortrags trägt - im Dateinamen
// und in den Dokumenteigenschaften (Titel, Thema, Stichwörter).
//
// Datum und Titel liest das Programm aus den gescannten Seiten selbst
// (Texterkennung von macOS, Vision-Framework): der Titel ist der Kopf der
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
//   brew install sane-backends
//   cargo run --release --bin scannen
//
// Immer `--batch` scannen: ein Einzelscan (`scanimage -o`) lässt den Einzug
// das nächste Blatt vorladen und ungescannt auswerfen.
//
// Jede Seite kommt als JPEG an und wird unverändert (DCTDecode) ins PDF
// gestellt, eine Seite je Bild, Seitengrösse aus Pixelzahl und Auflösung.
//
// Weitere Schalter: --dpi 300, --farbe (Vorgabe Graustufen),
// --ordner DIR (Vorgabe: scan/, wird angelegt), --geraet <SANE-Gerät>
// (Vorgabe: das erste epson2-Gerät aus `scanimage -L`),
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
        dpi: 300,
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
                    "scannen [--datum JJJJ-MM-TT] [--titel \"Titel\"] [--dpi 300] [--farbe] \
                     [--ordner DIR] [--geraet SANE-Gerät] [--lesen BILD…] [--aus BILD…]"
                );
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    Ok(o)
}

/// Das erste epson2-Gerät aus `scanimage -L`, etwa `epson2:libusb:001:004`.
/// Die USB-Nummern wechseln beim Umstecken, darum nicht fest verdrahten.
fn epson_geraet() -> anyhow::Result<String> {
    let aus = Command::new("scanimage")
        .arg("-L")
        .output()
        .context("scanimage nicht gefunden - brew install sane-backends")?;
    let text = String::from_utf8_lossy(&aus.stdout);
    text.lines()
        .filter_map(|z| z.strip_prefix("device `")?.split_once('\''))
        .map(|(geraet, _)| geraet.to_string())
        .find(|g| g.starts_with("epson2:"))
        .with_context(|| format!("kein Epson-Scanner gefunden - eingeschaltet und eingesteckt?\n{text}"))
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
        let (datum, titel, _) = erkennen(&opt.lesen)?;
        println!("Datum: {}", datum.as_deref().unwrap_or("(keines gefunden)"));
        println!("Titel: {}", titel.as_deref().unwrap_or("(keiner gefunden)"));
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
    let (mut datum, mut titel) = (opt.datum.clone(), opt.titel.clone());
    eprintln!("Texterkennung über {} Seiten …", seiten.len());
    let texte = match erkennen(&seiten) {
        Ok((d, t, texte)) => {
            datum = datum.or(d);
            titel = titel.or(t);
            texte
        }
        Err(e) => {
            eprintln!("Hinweis: Texterkennung fehlgeschlagen, PDF ohne Textebene: {e:#}");
            vec![Vec::new(); seiten.len()]
        }
    };
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

    std::fs::create_dir_all(&opt.ordner).with_context(|| format!("{}", opt.ordner.display()))?;
    // Nie überschreiben: bei gleichem Namen eine Nummer anhängen.
    let mut ziel = opt.ordner.join(dateiname(&datum, &titel, 1));
    let mut n = 2;
    while ziel.exists() {
        ziel = opt.ordner.join(dateiname(&datum, &titel, n));
        n += 1;
    }
    if let Err(e) = pdf_bauen(&seiten, &texte, opt.dpi, &datum, scandatum, &titel, &ziel) {
        eprintln!("Die gescannten Seiten liegen in {}", tmp.display());
        return Err(e);
    }
    std::fs::remove_dir_all(&tmp).ok();
    println!("{} ({} Seiten)", ziel.display(), seiten.len());
    Ok(())
}

/// Eine erkannte Textzeile. `oben` ist der Abstand der Oberkante vom oberen
/// Seitenrand, `hoehe` die Zeilenhöhe, beides als Anteil der Seitenhöhe;
/// `links` und `breite` entsprechend als Anteil der Seitenbreite.
#[derive(Debug, Clone)]
struct Zeile {
    text: String,
    oben: f64,
    hoehe: f64,
    links: f64,
    breite: f64,
}

type Erkennung = (Option<String>, Option<String>, Vec<Vec<Zeile>>);

/// Datum und Titel aus den Seitenbildern.
/// Datum, Titel und die Zeilen jeder Seite.
fn erkennen(seiten: &[PathBuf]) -> anyhow::Result<Erkennung> {
    let mut alle = Vec::new();
    for s in seiten {
        alle.push(ocr::zeilen(s)?);
    }
    let datum = datum_finden(&alle);
    let titel = alle.first().and_then(|z| titel_finden(z));
    Ok((datum, titel, alle))
}

#[cfg(not(target_os = "macos"))]
mod ocr {
    pub fn zeilen(_bild: &std::path::Path) -> anyhow::Result<Vec<super::Zeile>> {
        anyhow::bail!("Texterkennung gibt es nur unter macOS - --datum und --titel angeben")
    }
}

#[cfg(target_os = "macos")]
mod ocr {
    use anyhow::Context;
    use objc2::rc::Retained;
    use objc2::AllocAnyThread;
    use objc2_foundation::{NSArray, NSDictionary, NSString, NSURL};
    use objc2_vision::{
        VNImageRequestHandler, VNRecognizeTextRequest, VNRequest, VNRequestTextRecognitionLevel,
    };

    pub fn zeilen(bild: &std::path::Path) -> anyhow::Result<Vec<super::Zeile>> {
        let pfad = bild.canonicalize().with_context(|| format!("{}", bild.display()))?;
        let url = NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy()));
        let handler = unsafe {
            VNImageRequestHandler::initWithURL_options(
                VNImageRequestHandler::alloc(),
                &url,
                &NSDictionary::new(),
            )
        };
        let anfrage = VNRecognizeTextRequest::new();
        anfrage.setRecognitionLevel(VNRequestTextRecognitionLevel::Accurate);
        anfrage.setRecognitionLanguages(&NSArray::from_retained_slice(&[NSString::from_str("de-DE")]));
        anfrage.setUsesLanguageCorrection(true);
        let als_request: Retained<VNRequest> = Retained::into_super(Retained::into_super(anfrage.clone()));
        handler
            .performRequests_error(&NSArray::from_retained_slice(&[als_request]))
            .map_err(|e| anyhow::anyhow!("{}", e.localizedDescription()))
            .with_context(|| format!("Texterkennung {}", bild.display()))?;

        let mut zeilen = Vec::new();
        for beob in anfrage.results().into_iter().flatten() {
            let Some(kandidat) = beob.topCandidates(1).firstObject() else { continue };
            // Vision misst von unten links, normiert auf 0..1
            let r = unsafe { beob.boundingBox() };
            zeilen.push(super::Zeile {
                text: kandidat.string().to_string(),
                oben: 1.0 - (r.origin.y + r.size.height),
                hoehe: r.size.height,
                links: r.origin.x,
                breite: r.size.width,
            });
        }
        zeilen.sort_by(|a, b| a.oben.total_cmp(&b.oben));
        Ok(zeilen)
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
    t.contains("davatz") || t.contains("ganglion") || t.contains("www.") || t.contains('@')
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
            let luecke = z.oben - (v.oben + v.hoehe) > 0.7 * v.hoehe;
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

/// Dateiname aus Datum und Titel. Nur der erste Titelblock (vor « – »),
/// höchstens `MAX_TITEL_IM_NAMEN` Zeichen, an einer Wortgrenze gekürzt; was
/// im Finder stört, wird ersetzt. `nr` ab 2 unterscheidet gleiche Namen.
fn dateiname(datum: &str, titel: &str, nr: u32) -> String {
    let titel = titel.split(" – ").next().unwrap_or(titel);
    let titel: String = titel
        .chars()
        .map(|c| match c {
            '/' | ':' | '\\' => '-',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    let mut kurz = String::new();
    for wort in titel.split_whitespace() {
        let laenge = kurz.chars().count() + 1 + wort.chars().count();
        if !kurz.is_empty() && laenge > MAX_TITEL_IM_NAMEN {
            break;
        }
        if !kurz.is_empty() {
            kurz.push(' ');
        }
        kurz.push_str(wort);
    }
    let kurz: String = kurz.chars().take(MAX_TITEL_IM_NAMEN).collect();
    let kurz = kurz.trim_end_matches([',', ';', '-', ' ']);
    match nr {
        0 | 1 => format!("{datum} {kurz}.pdf"),
        n => format!("{datum} {kurz} ({n}).pdf"),
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
    for z in zeilen {
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
            r"(?i)\b(?:https?://[^\s<>]+|www\.[^\s<>]+|[a-z0-9][a-z0-9.-]*\.(?:ch|com|de|org|net|li|at)\b[^\s<>]*|[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,})",
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
                format!("https://{roh}")
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
        let z = |t: &str| Zeile { text: t.into(), oben: 0.1, hoehe: 0.01, links: 0.1, breite: 0.5 };
        let ziele = |t: &str| -> Vec<String> {
            links_finden(&[z(t)], 595.0, 842.0).into_iter().map(|(u, _)| u).collect()
        };
        assert_eq!(ziele("U. Davatz, www.ganglion.ch"), vec!["https://www.ganglion.ch"]);
        assert_eq!(ziele("siehe http://schizo.li."), vec!["http://schizo.li"]);
        assert_eq!(ziele("Mail: info@ganglion.ch"), vec!["mailto:info@ganglion.ch"]);
        assert!(ziele("Dr. med. U. Davatz").is_empty());
    }

    #[test]
    fn name() {
        assert_eq!(
            dateiname("2003-06-01", "Cannabis und  Schizophrenie: ein Überblick", 1),
            "2003-06-01 Cannabis und Schizophrenie- ein Überblick.pdf"
        );
        assert_eq!(
            dateiname(
                "ohne Datum",
                "Text für Werbung Familientherapie – Was ist Familientherapie und wie unterscheidet sie sich",
                2
            ),
            "ohne Datum Text für Werbung Familientherapie (2).pdf"
        );
        let lang = dateiname("2014-03-05", &"Wort ".repeat(30), 1);
        assert!(lang.chars().count() <= "2014-03-05 .pdf".len() + MAX_TITEL_IM_NAMEN);
    }
}
