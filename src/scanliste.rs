// Übersicht aller gescannten Vorträge in scan/: je PDF Titel, Datum,
// Seitenzahl, eine Zusammenfassung in drei bis fünf Sätzen und ein Link auf
// die Datei im GitHub-Repository.
//
// Die Zusammenfassungen stehen in `src/zusammenfassungen.rs`. Fehlt dort ein
// PDF aus scan/ - oder führt die Liste eines, das es nicht mehr gibt -, bricht
// das Programm ab: nach jedem neuen Scan also den Eintrag nachtragen.
//
//   cargo run --release --bin scanliste
//   cargo run --release --bin scanliste -- --out /pfad/zum.pdf
//
// Schriftverzeichnis überschreibbar via $FONT_DIR (Vorgabe: ./fonts).
//
// Lizenz: GPL-3.0

mod pdflinks;
mod zusammenfassungen;

use std::collections::BTreeSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Result};
use genpdf::elements::{Break, PageBreak, Paragraph};
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Element};

use pdflinks::{add_links, Link, LINK_FONT_SIZE};
use zusammenfassungen::ZUSAMMENFASSUNGEN;

const DEFAULT_FONT_DIR: &str = "fonts";
const DEFAULT_OUT: &str = "scan_uebersicht.pdf";
const SCAN_DIR: &str = "scan";
const TITEL: &str = "adhs.expert - Gescannte Vorträge und Kursunterlagen";
const GITHUB: &str = "https://github.com/zdavatz/adhs-expert/blob/main/scan/";

const INK: Color = Color::Rgb(0x1b, 0x1b, 0x1d);
const ACCENT: Color = Color::Rgb(0x8a, 0x2f, 0x4f);
const SLATE: Color = Color::Rgb(0x3a, 0x3d, 0x44);
const MUTED: Color = Color::Rgb(0x8a, 0x8d, 0x94);
const LINK: Color = Color::Rgb(0x2c, 0x5a, 0x8a);

// Die Link-Zeile darf nie umbrechen - zwei 8-pt-Zeilen verschöben jeden
// folgenden Link. Längere Dateinamen werden in der Mitte gekürzt.
const MAX_LINK_CHARS: usize = 88;

const A4_HOEHE_MM: f64 = 297.0;
const A4_BREITE_MM: f64 = 210.0;
const RAND_MM: f64 = 22.0;
// Muss mit `doc.set_line_spacing()` übereinstimmen, sonst laufen die
// selbstgesetzten Umbrüche aus dem Takt (siehe uebersicht.rs).
const ZEILENABSTAND: f64 = 1.35;
const RESERVE_MM: f64 = 8.0;

// ---------------------------------------------------------------------------
// Bestand
// ---------------------------------------------------------------------------

struct Scan {
    datei: String,
    titel: String,
    /// Anzeigeform: `TT.MM.JJJJ` oder `MM.JJJJ`
    datum: String,
    /// (Jahr, Monat, Tag) zum Sortieren
    schluessel: (u32, u32, u32),
    /// Zweite Fassung (`_2`) und weitere
    fassung: Option<u32>,
    seiten: usize,
    text: &'static str,
}

/// Zerlegt `Titel_TT.MM.JJJJ[_N].pdf` oder `Titel_MM.JJJJ[_N].pdf`.
fn zerlegen(datei: &str) -> Option<(String, String, (u32, u32, u32), Option<u32>)> {
    let stamm = datei.strip_suffix(".pdf")?;
    let mut teile: Vec<&str> = stamm.split('_').collect();
    let mut fassung = None;
    if teile.len() > 2 {
        if let Ok(n) = teile[teile.len() - 1].parse::<u32>() {
            if n < 100 {
                fassung = Some(n);
                teile.pop();
            }
        }
    }
    let datum = teile.pop()?.to_string();
    let zahlen: Vec<u32> = datum.split('.').map(|z| z.parse().ok()).collect::<Option<_>>()?;
    let schluessel = match zahlen[..] {
        [t, m, j] => (j, m, t),
        [m, j] => (j, m, 0),
        _ => return None,
    };
    let titel = teile.join(" ");
    if titel.is_empty() {
        return None;
    }
    Some((titel, datum, schluessel, fassung))
}

fn bestand() -> Result<Vec<Scan>> {
    let mut dateien = BTreeSet::new();
    for e in std::fs::read_dir(SCAN_DIR)? {
        let name = e?.file_name().to_string_lossy().into_owned();
        if name.ends_with(".pdf") {
            dateien.insert(name);
        }
    }
    let bekannt: BTreeSet<String> = ZUSAMMENFASSUNGEN.iter().map(|z| z.datei.to_string()).collect();
    let fehlt: Vec<&String> = dateien.difference(&bekannt).collect();
    let weg: Vec<&String> = bekannt.difference(&dateien).collect();
    if !fehlt.is_empty() || !weg.is_empty() {
        bail!(
            "src/zusammenfassungen.rs passt nicht zu {SCAN_DIR}/:\n  ohne Zusammenfassung: {fehlt:?}\n  \
             nicht mehr vorhanden: {weg:?}"
        );
    }

    let mut scans = Vec::new();
    for z in ZUSAMMENFASSUNGEN {
        let (aus_namen, datum, schluessel, fassung) =
            zerlegen(z.datei).ok_or_else(|| anyhow!("Dateiname ohne Datum: {}", z.datei))?;
        let pfad = Path::new(SCAN_DIR).join(z.datei);
        let seiten = lopdf::Document::load(&pfad)
            .map_err(|e| anyhow!("{}: {e}", pfad.display()))?
            .get_pages()
            .len();
        // Der Dateiname kürzt den Titel auf 60 Zeichen und lässt «?» und «:» weg.
        let titel = if z.titel.is_empty() { aus_namen } else { z.titel.to_string() };
        scans.push(Scan { datei: z.datei.into(), titel, datum, schluessel, fassung, seiten, text: z.text });
    }
    scans.sort_by(|a, b| (a.schluessel, &a.titel, a.fassung).cmp(&(b.schluessel, &b.titel, b.fassung)));
    Ok(scans)
}

/// Prozentkodierung für den Pfadteil der GitHub-Adresse.
fn kodieren(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn link_text(datei: &str) -> String {
    let s = format!("github.com/zdavatz/adhs-expert/scan/{datei}");
    let z: Vec<char> = s.chars().collect();
    if z.len() <= MAX_LINK_CHARS {
        return s;
    }
    let vorn = (MAX_LINK_CHARS - 1) / 2;
    let hinten = MAX_LINK_CHARS - 1 - vorn;
    format!("{}…{}", z[..vorn].iter().collect::<String>(), z[z.len() - hinten..].iter().collect::<String>())
}

fn kopfzeile(s: &Scan, nr: usize) -> String {
    match s.fassung {
        Some(n) => format!("{nr}.  {} ({}. Fassung)", s.titel, n),
        None => format!("{nr}.  {}", s.titel),
    }
}

fn metazeile(s: &Scan) -> String {
    let seiten = if s.seiten == 1 { "1 Seite".to_string() } else { format!("{} Seiten", s.seiten) };
    format!("{}   ·   {seiten}", s.datum)
}

fn heute() -> String {
    const MONATE: [&str; 12] = [
        "Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September",
        "Oktober", "November", "Dezember",
    ];
    let aus = Command::new("date").arg("+%Y %m %d").output().ok();
    let s = aus.map(|a| String::from_utf8_lossy(&a.stdout).into_owned()).unwrap_or_default();
    let z: Vec<usize> = s.split_whitespace().filter_map(|t| t.parse().ok()).collect();
    match z[..] {
        [j, m, t] if (1..=12).contains(&m) => format!("Stand: {t}. {} {j}", MONATE[m - 1]),
        _ => String::new(),
    }
}

// ---------------------------------------------------------------------------
// Seitenumbruch von Hand (wie in uebersicht.rs: genpdf kennt kein
// "keep together", ein Eintrag soll nicht über zwei Seiten laufen)
// ---------------------------------------------------------------------------

fn st(groesse: u8) -> Style {
    Style::new().with_font_size(groesse).with_line_spacing(ZEILENABSTAND)
}

fn titel_st() -> Style {
    st(10).bold().with_color(INK)
}

fn text_st() -> Style {
    st(9).with_color(SLATE)
}

fn meta_st() -> Style {
    st(7).with_color(MUTED)
}

/// Zahl der Zeilen, die `text` im gegebenen Stil auf `breite` mm belegt.
/// Bildet die gierige Wortumbruch-Logik von `Paragraph` nach.
fn zeilenzahl(fc: &genpdf::fonts::FontCache, style: Style, text: &str, breite: f64) -> usize {
    let leer = f64::from(style.str_width(fc, " "));
    let mut zeilen = 1usize;
    let mut belegt = 0.0f64;
    for wort in text.split_whitespace() {
        let w = f64::from(style.str_width(fc, wort));
        if belegt > 0.0 && belegt + leer + w > breite {
            zeilen += 1;
            belegt = w;
        } else {
            belegt += if belegt > 0.0 { leer + w } else { w };
        }
    }
    zeilen
}

/// Höhe jedes Eintrags in mm und nutzbare Höhe einer Seite.
fn messen(doc: &genpdf::Document, scans: &[Scan]) -> (Vec<f64>, f64) {
    let fc = doc.font_cache();
    let breite = A4_BREITE_MM - 2.0 * RAND_MM;
    // genpdf setzt jede Zeile mindestens in der Zeilenhöhe der Grundschrift
    // (10 pt) - auch die kleineren 9-, 8- und 7-pt-Zeilen. Nachgemessen mit
    // `pdftotext -bbox`: Zeilenabstand 5,55 mm statt der 5,0 für 9 pt.
    let grund = f64::from(st(10).line_height(fc));
    let h = |s: Style| f64::from(s.line_height(fc)).max(grund);
    let abstand = h(st(10)) * 0.7;
    let hoehen: Vec<f64> = scans
        .iter()
        .enumerate()
        .map(|(i, s)| {
            zeilenzahl(fc, titel_st(), &kopfzeile(s, i + 1), breite) as f64 * h(titel_st())
                + h(meta_st())
                + zeilenzahl(fc, text_st(), s.text, breite) as f64 * h(text_st())
                + h(st(LINK_FONT_SIZE))
                + abstand
        })
        .collect();
    // Kopfzeile (7 pt + 6 mm) abziehen und eine Reserve freihalten, damit
    // unser Umbruch sicher vor dem von genpdf kommt - fällt er zusammen,
    // entsteht eine leere Seite. Anders als in uebersicht.rs nicht den
    // grössten Eintrag: die Einträge sind hier gross (bis 70 mm), die Seiten
    // blieben sonst halb leer. Die Messung stimmt auf den Millimeter.
    let nutzbar = A4_HOEHE_MM - 2.0 * RAND_MM - (h(st(7)) + 6.0) - RESERVE_MM;
    if env::var("MASS_DEBUG").is_ok() {
        eprintln!("nutzbar={nutzbar:.1} erste={:?}", &hoehen[..3]);
    }
    (hoehen, nutzbar)
}

// ---------------------------------------------------------------------------
// Satz
// ---------------------------------------------------------------------------

fn zeile(text: impl Into<String>, style: Style) -> Paragraph {
    let mut p = Paragraph::default();
    p.push_styled(text.into(), style);
    p
}

fn push_titelseite(doc: &mut genpdf::Document, scans: &[Scan]) {
    let seiten: usize = scans.iter().map(|s| s.seiten).sum();
    doc.push(Break::new(3.0));
    doc.push(
        zeile("adhs.expert", Style::new().with_color(ACCENT).with_font_size(11).bold())
            .aligned(Alignment::Center),
    );
    doc.push(Break::new(0.6));
    // Der Absatz selbst muss den 22-pt-Stil tragen (`styled`): sonst bemisst
    // genpdf die Zeilenhöhe nach der Grundschrift, und die zwei Zeilen des
    // Titels überlappen.
    let gross = Style::new().with_color(INK).with_font_size(22).bold();
    doc.push(zeile("Gescannte Vorträge und Kursunterlagen", gross).aligned(Alignment::Center).styled(gross));
    doc.push(Break::new(0.6));
    doc.push(
        zeile(
            "Dr. med. Ursula Davatz – mit einer kurzen Zusammenfassung je Dokument",
            Style::new().with_color(SLATE).with_font_size(11),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(2.0));
    doc.push(
        zeile(
            format!("{} PDFs   ·   {} Seiten", scans.len(), seiten),
            Style::new().with_color(INK).with_font_size(11).bold(),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(0.5));
    doc.push(zeile(heute(), Style::new().with_color(MUTED).with_font_size(9)).aligned(Alignment::Center));

    doc.push(Break::new(2.5));
    doc.push(zeile("Zu dieser Sammlung", Style::new().with_color(ACCENT).with_font_size(13).bold()));
    doc.push(Break::new(0.6));
    for absatz in [
        "Die Vorlagen sind Vortragsskripte, Handouts und Kursunterlagen aus den \
         Familientherapie-Kursen, Weiterbildungen und Vorträgen von Ursula Davatz, \
         dazu einzelne Arbeiten von Kursteilnehmern und Ko-Referenten. Sie lagen nur \
         auf Papier vor und wurden mit einem Einzugsscanner digitalisiert.",
        "Jedes PDF trägt neben dem Bild eine unsichtbare Textebene aus der \
         Texterkennung (Tesseract) und ist damit durchsuchbar. Mail-, Strassen- und \
         Telefonangaben wurden entfernt; in Fallbeispielen und persönlichen Berichten \
         sind Namen und Daten durch erfundene ersetzt.",
        "Die Liste ist nach Vortragsdatum geordnet. Nennt eine Vorlage kein Datum, \
         gilt der Tag des Scans; bei blosser Jahresangabe das Jahr mit Tag und Monat \
         vom Scan. Die Zusammenfassungen fassen den Inhalt in wenigen Sätzen; jede \
         Adresse darunter ist anklickbar und führt zur Datei auf GitHub.",
    ] {
        doc.push(zeile(absatz, Style::new().with_color(INK).with_font_size(10)));
        doc.push(Break::new(0.5));
    }
}

fn push_eintrag(doc: &mut genpdf::Document, s: &Scan, nr: usize, letzter: bool) {
    doc.push(zeile(kopfzeile(s, nr), titel_st()));
    doc.push(zeile(metazeile(s), meta_st()));
    doc.push(zeile(s.text, text_st()));
    doc.push(zeile(link_text(&s.datei), st(LINK_FONT_SIZE).with_color(LINK)));
    // Nach dem letzten Eintrag kein Abstand - er erzwänge eine leere Schlussseite.
    if !letzter {
        doc.push(Break::new(0.7));
    }
}

fn push_liste(doc: &mut genpdf::Document, scans: &[Scan], hoehen: &[f64], nutzbar: f64) {
    let mut rest = 0.0;
    for (i, s) in scans.iter().enumerate() {
        if hoehen[i] > rest {
            doc.push(PageBreak::new());
            rest = nutzbar;
        }
        push_eintrag(doc, s, i + 1, i + 1 == scans.len());
        rest -= hoehen[i];
    }
}

// ---------------------------------------------------------------------------
// Ausgabe
// ---------------------------------------------------------------------------

fn load_font_family(font_dir: &str) -> Result<genpdf::fonts::FontFamily<genpdf::fonts::FontData>> {
    let load = |file: &str| -> Result<genpdf::fonts::FontData> {
        let path = Path::new(font_dir).join(file);
        let data = std::fs::read(&path).map_err(|e| anyhow!("Schrift {}: {}", path.display(), e))?;
        genpdf::fonts::FontData::new(data, None).map_err(|e| anyhow!("Schrift {}: {}", file, e))
    };
    Ok(genpdf::fonts::FontFamily {
        regular: load("DejaVuSans.ttf")?,
        bold: load("DejaVuSans-Bold.ttf")?,
        italic: load("DejaVuSans-Oblique.ttf")?,
        bold_italic: load("DejaVuSans-BoldOblique.ttf")?,
    })
}

fn render(out: &Path, font_dir: &str) -> Result<()> {
    let scans = bestand()?;
    let family = load_font_family(font_dir)?;
    let mut doc = genpdf::Document::new(family);
    doc.set_title(TITEL);
    doc.set_minimal_conformance();
    doc.set_font_size(10);
    doc.set_line_spacing(ZEILENABSTAND);

    let mut deco = genpdf::SimplePageDecorator::new();
    deco.set_margins(RAND_MM as i32);
    deco.set_header(move |page| {
        let mut p = Paragraph::default();
        if page > 1 {
            p.push_styled(
                format!("adhs.expert - Gescannte Vorträge und Kursunterlagen          {page}"),
                Style::new().with_color(MUTED).with_font_size(7),
            );
        }
        p.aligned(Alignment::Right).padded(genpdf::Margins::trbl(0, 0, 6, 0))
    });
    doc.set_page_decorator(deco);

    let (hoehen, nutzbar) = messen(&doc, &scans);
    push_titelseite(&mut doc, &scans);
    push_liste(&mut doc, &scans, &hoehen, nutzbar);

    doc.render_to_file(out).map_err(|e| anyhow!("PDF schreiben {}: {}", out.display(), e))?;

    let links: Vec<Link> = scans
        .iter()
        .map(|s| Link { url: format!("{GITHUB}{}", kodieren(&s.datei)), zeichen: link_text(&s.datei).chars().count() })
        .collect();
    let gesetzt = add_links(out, &links, TITEL)?;
    if gesetzt != links.len() {
        bail!(
            "Link-Overlay: {gesetzt} Link-Zeilen im Satz gefunden, aber {} erwartet - \
             die Zuordnung wäre verschoben",
            links.len()
        );
    }
    println!("  {} Scans, {gesetzt} Links gesetzt", scans.len());
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_OUT));
    let font_dir = env::var("FONT_DIR").unwrap_or_else(|_| DEFAULT_FONT_DIR.to_string());

    render(&out, &font_dir)?;
    let bytes = std::fs::metadata(&out)?.len();
    println!("→ {} ({} B)", out.display(), bytes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dateinamen() {
        let (t, d, k, f) = zerlegen("Ablösung_von_der_Autorität_28.10.1991.pdf").unwrap();
        assert_eq!((t.as_str(), d.as_str(), k, f), ("Ablösung von der Autorität", "28.10.1991", (1991, 10, 28), None));
        let (_, d, k, f) = zerlegen("Familie_12.03.1990_2.pdf").unwrap();
        assert_eq!((d.as_str(), k, f), ("12.03.1990", (1990, 3, 12), Some(2)));
        let (_, d, k, _) = zerlegen("Ehe_12.1991.pdf").unwrap();
        assert_eq!((d.as_str(), k), ("12.1991", (1991, 12, 0)));
        assert!(zerlegen("ohne_datum.pdf").is_none());
    }

    #[test]
    fn adresse() {
        assert_eq!(kodieren("Ablösung, a.pdf"), "Abl%C3%B6sung%2C%20a.pdf");
    }
}
