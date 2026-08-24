// Übersicht der Beiträge von adhs.expert, deren PDF-Text jetzt vollständig
// im Beitrag selbst steht.
//
// Bis August 2026 bestanden viele Beiträge nur aus einem Audio-Player und
// einem nackten Link auf ein PDF - für Google praktisch leere Seiten. Der
// Text der PDFs wurde 1:1 in die Beiträge übernommen. Fünf PDFs waren
// Scans ohne Textebene und mussten zuerst per OCR erkannt werden; diese
// Fälle sind hier eigens ausgewiesen.
//
// Pure Rust, kein Chrome: das PDF entsteht mit `genpdf` (das über printpdf
// schreibt) und bettet die DejaVu-Sans-Familie ein - dieselbe Pipeline wie
// in ~/.software/fundaziun-davaz.
//
//   cargo run --release --bin uebersicht
//   cargo run --release --bin uebersicht -- --out /pfad/zum.pdf
//
// Schriftverzeichnis überschreibbar via $FONT_DIR (Vorgabe: ./fonts).

mod eintraege;

use std::env;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use genpdf::elements::{Break, PageBreak, Paragraph};
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Element};

use eintraege::{Eintrag, EINTRAEGE};

const DEFAULT_FONT_DIR: &str = "fonts";
const DEFAULT_OUT: &str = "adhs_expert_beitraege_mit_volltext.pdf";
const STAND: &str = "Stand: 24. August 2026";

const INK: Color = Color::Rgb(0x1b, 0x1b, 0x1d);
const ACCENT: Color = Color::Rgb(0x8a, 0x2f, 0x4f);
const SLATE: Color = Color::Rgb(0x3a, 0x3d, 0x44);
const MUTED: Color = Color::Rgb(0x8a, 0x8d, 0x94);
const LINK: Color = Color::Rgb(0x2c, 0x5a, 0x8a);

// genpdf 0.2 kennt keine Hyperlinks. Die URL-Zeilen sind die einzigen Zeilen
// des Dokuments in dieser Schriftgrösse; nach dem Rendern legt `add_links`
// über jede solche Zeile eine Link-Annotation. Sobald irgendwo sonst im Satz
// diese Grösse auftaucht, verschiebt sich die Zuordnung - deshalb laufen
// Kopfzeile und Beiwerk bewusst auf 7 pt und die Metazeilen auf 9 pt.
const LINK_FONT_SIZE: u8 = 8;
const A4_WIDTH_PT: f64 = 595.276;
const MARGIN_PT: f64 = 22.0 * 72.0 / 25.4;
const AVG_ADVANCE_EM: f64 = 0.55;
const MAX_LINK_CHARS: usize = 88;

// ---------------------------------------------------------------------------
// Kennzahlen
// ---------------------------------------------------------------------------

fn anzahl_ocr() -> usize {
    EINTRAEGE.iter().filter(|e| e.ocr).count()
}

fn woerter_gesamt() -> u64 {
    EINTRAEGE.iter().map(|e| e.woerter as u64).sum()
}

fn tausender(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push('\u{2009}');
        }
        out.push(c);
    }
    out
}

/// Anzeigeform einer URL: Schema und `www.` weg, danach bei Bedarf in der
/// Mitte gekürzt. Die Klickfläche wird nach dieser Länge bemessen, nicht nach
/// der vollen Adresse - verlinkt wird immer das Original.
fn link_text(url: &str) -> String {
    let s = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let s = s.strip_prefix("www.").unwrap_or(s);
    if s.chars().count() <= MAX_LINK_CHARS {
        return s.to_string();
    }
    let (host, rest) = match s.find('/') {
        Some(i) => s.split_at(i),
        None => (s, ""),
    };
    let platz = MAX_LINK_CHARS.saturating_sub(host.chars().count() + 3);
    let rest: Vec<char> = rest.chars().collect();
    if rest.len() <= platz {
        return s.to_string();
    }
    let vorn = platz / 2;
    let hinten = platz - vorn;
    let a: String = rest[..vorn].iter().collect();
    let b: String = rest[rest.len() - hinten..].iter().collect();
    format!("{host}{a}…{b}")
}

// ---------------------------------------------------------------------------
// Satz
// ---------------------------------------------------------------------------

fn zeile(text: impl Into<String>, style: Style) -> Paragraph {
    let mut p = Paragraph::default();
    p.push_styled(text.into(), style);
    p
}

fn push_cover(doc: &mut genpdf::Document) {
    doc.push(Break::new(3.0));
    doc.push(
        zeile(
            "adhs.expert",
            Style::new().with_color(ACCENT).with_font_size(11).bold(),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(0.6));
    doc.push(
        zeile(
            "Beiträge mit vollständigem Text",
            Style::new().with_color(INK).with_font_size(22).bold(),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(0.6));
    doc.push(
        zeile(
            "Der Text der verlinkten PDFs steht jetzt im Beitrag selbst",
            Style::new().with_color(SLATE).with_font_size(11),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(2.0));
    doc.push(
        zeile(
            format!(
                "{} Beiträge   ·   {} Wörter   ·   davon {} per OCR",
                EINTRAEGE.len(),
                tausender(woerter_gesamt()),
                anzahl_ocr()
            ),
            Style::new().with_color(INK).with_font_size(11).bold(),
        )
        .aligned(Alignment::Center),
    );
    doc.push(Break::new(0.5));
    doc.push(
        zeile(STAND, Style::new().with_color(MUTED).with_font_size(9))
            .aligned(Alignment::Center),
    );
}

fn push_methode(doc: &mut genpdf::Document) {
    doc.push(Break::new(2.5));
    doc.push(zeile(
        "Was gemacht wurde",
        Style::new().with_color(ACCENT).with_font_size(13).bold(),
    ));
    doc.push(Break::new(0.6));
    for absatz in [
        "Viele Beiträge bestanden nur aus einem Audio-Player und einem nackten \
         Link auf ein PDF. Für Suchmaschinen waren das praktisch leere Seiten - \
         der gesamte Inhalt lag im PDF, nicht im Beitrag.",
        "Der Text wurde mit pdftotext aus den PDFs gelesen und 1:1 in den Beitrag \
         übernommen, ohne Umformulierung und ohne Kürzung. Audio-Player und \
         PDF-Link bleiben erhalten; der Text steht darunter unter der Überschrift \
         \u{201E}Transkript\u{201C}.",
        "Lange Adressen brechen im PDF-Satz mitten in der URL um. Diese Umbrüche \
         sind Layout und nicht Text: sie wurden wieder zusammengefügt, danach sind \
         die Adressen im Beitrag als Links anklickbar.",
    ] {
        doc.push(zeile(
            absatz,
            Style::new().with_color(INK).with_font_size(10),
        ));
        doc.push(Break::new(0.5));
    }
}

fn push_ocr_abschnitt(doc: &mut genpdf::Document) {
    let ocr: Vec<&Eintrag> = EINTRAEGE.iter().filter(|e| e.ocr).collect();
    if ocr.is_empty() {
        return;
    }
    doc.push(Break::new(1.2));
    doc.push(zeile(
        format!("Mit OCR erkannt ({})", ocr.len()),
        Style::new().with_color(ACCENT).with_font_size(13).bold(),
    ));
    doc.push(Break::new(0.6));
    doc.push(zeile(
        "Bei diesen Beiträgen war das verlinkte PDF ein Scan ohne Textebene - \
         eingescannte Flyer, Zeitungsausschnitte, Interviews. pdftotext fand dort \
         nichts. Der Text wurde deshalb mit Tesseract 4 (Sprache Deutsch) aus dem \
         Bild erkannt und dann eingesetzt. Diese Beiträge tragen im Beitrag die \
         Überschrift \u{201E}Transkript (OCR)\u{201C}.",
        Style::new().with_color(INK).with_font_size(10),
    ));
    doc.push(Break::new(0.5));
    doc.push(zeile(
        "OCR liest nicht fehlerfrei. Bei grafisch gesetzten Vorlagen - besonders \
         bei Plakaten mit Schmuckschrift - stehen im erkannten Text Verlesungen. \
         Der Fliesstext ist durchweg brauchbar, die Kopfgrafiken sind es nicht \
         immer. In der Liste unten sind diese Beiträge mit OCR gekennzeichnet.",
        Style::new().with_color(SLATE).with_font_size(10),
    ));
    doc.push(Break::new(0.8));
    for e in &ocr {
        push_eintrag(doc, e, None);
    }
}

/// Ein Listeneintrag. Die URL steht als einzige Zeile in LINK_FONT_SIZE -
/// daran hängt die Link-Annotation.
fn push_eintrag(doc: &mut genpdf::Document, e: &Eintrag, nr: Option<usize>) {
    let titel = match nr {
        Some(n) => format!("{n}.  {}", e.titel),
        None => e.titel.to_string(),
    };
    doc.push(zeile(
        titel,
        Style::new().with_color(INK).with_font_size(10).bold(),
    ));

    let mut meta = Paragraph::default();
    meta.push_styled(
        format!("{}   ·   {} Wörter", e.datum, tausender(e.woerter as u64)),
        Style::new().with_color(MUTED).with_font_size(9),
    );
    if e.ocr {
        meta.push_styled(
            "   ·   OCR",
            Style::new().with_color(ACCENT).with_font_size(9).bold(),
        );
    }
    doc.push(meta);

    doc.push(zeile(
        link_text(e.url),
        Style::new()
            .with_color(LINK)
            .with_font_size(LINK_FONT_SIZE),
    ));
    doc.push(Break::new(0.55));
}

fn push_liste(doc: &mut genpdf::Document) {
    doc.push(PageBreak::new());
    doc.push(zeile(
        format!("Alle Beiträge ({})", EINTRAEGE.len()),
        Style::new().with_color(ACCENT).with_font_size(13).bold(),
    ));
    doc.push(Break::new(0.4));
    doc.push(zeile(
        "Neueste zuerst. Jede Adresse ist anklickbar.",
        Style::new().with_color(MUTED).with_font_size(9),
    ));
    doc.push(Break::new(0.8));
    for (i, e) in EINTRAEGE.iter().enumerate() {
        push_eintrag(doc, e, Some(i + 1));
    }
}

/// Reihenfolge der URLs im Satz: erst der OCR-Abschnitt, dann die volle Liste.
fn urls() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = EINTRAEGE.iter().filter(|e| e.ocr).map(|e| e.url).collect();
    v.extend(EINTRAEGE.iter().map(|e| e.url));
    v
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

/// Legt über jede in LINK_FONT_SIZE gesetzte Textzeile eine Link-Annotation.
///
/// Der Inhaltsstrom wird Seite für Seite in Zeichenreihenfolge durchlaufen;
/// printpdf schreibt je Zeile `BT / TL / Td x y / Tf /F n / TJ [...]`, sodass
/// das letzte `Td` vor einem `TJ` die Grundlinie der Zeile angibt.
fn add_links(pdf: &Path, urls: &[&str]) -> Result<usize> {
    use lopdf::{Dictionary, Document, Object, StringFormat};

    let mut doc = Document::load(pdf)?;
    let seiten: Vec<(u32, lopdf::ObjectId)> = doc.get_pages().into_iter().collect();

    let num = |o: &Object| -> Option<f64> {
        match o {
            Object::Real(r) => Some(*r as f64),
            Object::Integer(i) => Some(*i as f64),
            _ => None,
        }
    };

    let mut gesetzt = 0usize;
    for (_, page_id) in seiten {
        let content = doc.get_and_decode_page_content(page_id)?;

        let mut pos = (0.0f64, 0.0f64);
        let mut size = 0.0f64;
        let mut origins: Vec<(f64, f64)> = Vec::new();
        for op in &content.operations {
            match op.operator.as_str() {
                "Td" | "TD" if op.operands.len() >= 2 => {
                    if let (Some(x), Some(y)) = (num(&op.operands[0]), num(&op.operands[1])) {
                        pos = (x, y);
                    }
                }
                "Tm" if op.operands.len() >= 6 => {
                    if let (Some(x), Some(y)) = (num(&op.operands[4]), num(&op.operands[5])) {
                        pos = (x, y);
                    }
                }
                "Tf" if op.operands.len() >= 2 => {
                    if let Some(s) = num(&op.operands[1]) {
                        size = s;
                    }
                }
                "Tj" | "TJ" => {
                    if (size - LINK_FONT_SIZE as f64).abs() < 0.01 && origins.last() != Some(&pos) {
                        origins.push(pos);
                    }
                }
                _ => {}
            }
        }
        if origins.is_empty() {
            continue;
        }

        let mut annots: Vec<Object> = Vec::new();
        for (x, y) in &origins {
            let Some(url) = urls.get(gesetzt) else { break };
            gesetzt += 1;

            let breite =
                (link_text(url).chars().count() as f64) * LINK_FONT_SIZE as f64 * AVG_ADVANCE_EM;
            let rechts = (x + breite + 2.0).min(A4_WIDTH_PT - MARGIN_PT);

            let mut action = Dictionary::new();
            action.set("S", Object::Name(b"URI".to_vec()));
            action.set(
                "URI",
                Object::String(url.as_bytes().to_vec(), StringFormat::Literal),
            );

            let mut annot = Dictionary::new();
            annot.set("Type", Object::Name(b"Annot".to_vec()));
            annot.set("Subtype", Object::Name(b"Link".to_vec()));
            annot.set(
                "Rect",
                Object::Array(vec![
                    Object::Real((*x - 2.0) as f32),
                    Object::Real((*y - 2.0) as f32),
                    Object::Real(rechts as f32),
                    Object::Real((*y + LINK_FONT_SIZE as f64 + 2.0) as f32),
                ]),
            );
            annot.set("Border", Object::Array(vec![0.into(), 0.into(), 0.into()]));
            annot.set("A", Object::Dictionary(action));
            annots.push(Object::Dictionary(annot));
        }

        if let Ok(page) = doc.get_object_mut(page_id).and_then(|o| o.as_dict_mut()) {
            page.set("Annots", Object::Array(annots));
        }
    }

    doc.save(pdf)?;
    Ok(gesetzt)
}

fn render(out: &Path, font_dir: &str) -> Result<()> {
    let family = load_font_family(font_dir)?;
    let mut doc = genpdf::Document::new(family);
    doc.set_title("adhs.expert - Beiträge mit vollständigem Text");
    doc.set_minimal_conformance();
    doc.set_font_size(10);
    doc.set_line_spacing(1.35);

    let mut deco = genpdf::SimplePageDecorator::new();
    deco.set_margins(22);
    deco.set_header(move |page| {
        let mut p = Paragraph::default();
        if page > 1 {
            p.push_styled(
                format!("adhs.expert - Beiträge mit vollständigem Text          {page}"),
                Style::new().with_color(MUTED).with_font_size(7),
            );
        }
        p.aligned(Alignment::Right)
            .padded(genpdf::Margins::trbl(0, 0, 6, 0))
    });
    doc.set_page_decorator(deco);

    push_cover(&mut doc);
    push_methode(&mut doc);
    push_ocr_abschnitt(&mut doc);
    push_liste(&mut doc);

    doc.render_to_file(out)
        .map_err(|e| anyhow!("PDF schreiben {}: {}", out.display(), e))?;

    let urls = urls();
    let gesetzt = add_links(out, &urls)?;
    if gesetzt != urls.len() {
        return Err(anyhow!(
            "Link-Overlay: {} URL-Zeilen im Satz gefunden, aber {} URLs erwartet - \
             die Zuordnung wäre verschoben",
            gesetzt,
            urls.len()
        ));
    }
    println!("  {gesetzt} Links gesetzt");
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
