// Link-Annotationen nachträglich über ein genpdf-PDF legen.
//
// genpdf 0.2 kennt keine Hyperlinks. Die Link-Zeilen sind darum die einzigen
// Zeilen des Dokuments in `LINK_FONT_SIZE`; nach dem Rendern legt
// `add_links` über jede solche Zeile eine Link-Annotation. Gemeinsam genutzt
// von `uebersicht` und `scanliste`.
//
// Lizenz: GPL-3.0

use std::path::Path;

use anyhow::Result;

pub const LINK_FONT_SIZE: u8 = 8;
pub const A4_WIDTH_PT: f64 = 595.276;
pub const MARGIN_PT: f64 = 22.0 * 72.0 / 25.4;
const AVG_ADVANCE_EM: f64 = 0.55;

/// Ein Link: Ziel und Zahl der angezeigten Zeichen (für die Breite der
/// Klickfläche - die Anzeige darf vom Ziel abweichen).
pub struct Link {
    pub url: String,
    pub zeichen: usize,
}

/// Legt über jede in LINK_FONT_SIZE gesetzte Textzeile eine Link-Annotation,
/// in der Reihenfolge von `links`. Gibt die Zahl der gesetzten Links zurück.
/// Schreibt dabei auch `titel` neu ins Info-Verzeichnis (siehe `utf16`).
///
/// Der Inhaltsstrom wird Seite für Seite in Zeichenreihenfolge durchlaufen;
/// printpdf schreibt je Zeile `BT / TL / Td x y / Tf /F n / TJ [...]`, sodass
/// das letzte `Td` vor einem `TJ` die Grundlinie der Zeile angibt.
pub fn add_links(pdf: &Path, links: &[Link], titel: &str) -> Result<usize> {
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
            let Some(link) = links.get(gesetzt) else { break };
            gesetzt += 1;

            let breite = link.zeichen as f64 * LINK_FONT_SIZE as f64 * AVG_ADVANCE_EM;
            let rechts = (x + breite + 2.0).min(A4_WIDTH_PT - MARGIN_PT);

            let mut action = Dictionary::new();
            action.set("S", Object::Name(b"URI".to_vec()));
            action.set(
                "URI",
                Object::String(link.url.as_bytes().to_vec(), StringFormat::Literal),
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

    if let Ok(info) = doc.trailer.get(b"Info").and_then(Object::as_reference) {
        if let Ok(info) = doc.get_object_mut(info).and_then(Object::as_dict_mut) {
            info.set("Title", utf16(titel));
        }
    }

    doc.save(pdf)?;
    Ok(gesetzt)
}

/// printpdf schreibt den Titel als rohes UTF-8 in eine Literal-Zeichenkette;
/// Leser deuten das als PDFDocEncoding und zeigen «VortrÃ¤ge». Richtig ist
/// UTF-16BE mit Byte-Order-Mark.
fn utf16(s: &str) -> lopdf::Object {
    let mut b = vec![0xFE, 0xFF];
    for e in s.encode_utf16() {
        b.extend_from_slice(&e.to_be_bytes());
    }
    lopdf::Object::String(b, lopdf::StringFormat::Hexadecimal)
}
