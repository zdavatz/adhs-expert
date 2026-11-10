// Rückenschilder für Bundesordner: «Vorträge Dr. med. Ursula Davatz
// JJJJ – JJJJ», nebeneinander auf ein A4-Blatt im Querformat, mit
// gestrichelter Schnittlinie zum Ausschneiden.
//
//   cargo run --release --bin etiketten -- --jahre 1983-1988 \
//       --jahre 1987-1994 --jahre 2013-2018@25 [--breite 50] [--hoehe 145] \
//       [--out etiketten.pdf]
//
// Vorgabe 50 × 145 mm: die Einstecktasche der grauen Ordner (gemessen);
// der schmale orange Ordner hat 25 × 145 mm.
// «@MM» hinter den Jahren setzt die Breite eines einzelnen Schilds. Schmale
// Schilder (unter 40 mm) bekommen den Text längs, von unten nach oben
// lesbar, in zwei Zeilen: Name und Jahre.
// Schrift: DejaVu Sans Bold aus $FONT_DIR (Vorgabe ./fonts), eingebettet.
// Drucken in Originalgrösse:
//   cargo run --release --bin pcldruck -- etiketten.pdf --an DRUCKER:9100
// (nicht `lp` an eine Raw-Queue: der Drucker druckt sonst den PDF-Quelltext).
//
// Lizenz: GPL-3.0

use std::io::BufWriter;

use anyhow::{bail, Context, Result};
use printpdf::{CurTransMat, IndirectFontRef, Line, LineDashPattern, Mm, PdfDocument, PdfLayerReference, Point};
use rusttype::{Font, Scale};

const A4_BREIT: f64 = 297.0;
const A4_HOCH: f64 = 210.0;
const PT_JE_MM: f64 = 72.0 / 25.4;

struct Schild {
    von: String,
    bis: String,
    breite: Option<f64>,
}

struct Optionen {
    jahre: Vec<Schild>,
    breite: f64,
    hoehe: f64,
    out: String,
}

fn optionen() -> Result<Optionen> {
    let mut o = Optionen { jahre: vec![], breite: 50.0, hoehe: 145.0, out: "etiketten.pdf".into() };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut wert = || args.next().with_context(|| format!("{a} braucht einen Wert"));
        match a.as_str() {
            "--jahre" => {
                let w = wert()?;
                let (jahre, breite) = match w.split_once('@') {
                    Some((j, b)) => (j, Some(b.trim().parse::<f64>().with_context(|| format!("--jahre {w}: Breite"))?)),
                    None => (w.as_str(), None),
                };
                let (von, bis) = jahre.split_once('-').with_context(|| format!("--jahre {w}: JJJJ-JJJJ[@MM]"))?;
                o.jahre.push(Schild { von: von.trim().into(), bis: bis.trim().into(), breite });
            }
            "--breite" => o.breite = wert()?.parse()?,
            "--hoehe" => o.hoehe = wert()?.parse()?,
            "--out" => o.out = wert()?,
            "-h" | "--help" => {
                println!("etiketten --jahre JJJJ-JJJJ [--jahre …] [--breite MM] [--hoehe MM] [--out DATEI]");
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    if o.jahre.is_empty() {
        bail!("--jahre fehlt");
    }
    if gesamtbreite(&o) > A4_BREIT - 10.0 || o.hoehe > A4_HOCH - 10.0 {
        bail!("{} Schilder zu {} × {} mm passen nicht auf ein A4-Blatt", o.jahre.len(), o.breite, o.hoehe);
    }
    Ok(o)
}

const ABSTAND: f64 = 6.0;

fn gesamtbreite(o: &Optionen) -> f64 {
    let n = o.jahre.len() as f64;
    o.jahre.iter().map(|s| s.breite.unwrap_or(o.breite)).sum::<f64>() + (n - 1.0) * ABSTAND
}

/// Breite einer Zeile in mm bei gegebener Schriftgrösse (pt).
fn breite_mm(font: &Font, text: &str, pt: f64) -> f64 {
    // rusttype skaliert auf die Pixelhöhe (Ober- bis Unterlänge), nicht
    // aufs Geviert - darum die Skala auf das Geviert umrechnen.
    let v = font.v_metrics_unscaled();
    let geviert = (v.ascent - v.descent) / font.units_per_em() as f32;
    let s = Scale::uniform(pt as f32 * geviert);
    let pt_breit: f32 = text.chars().map(|c| font.glyph(c).scaled(s).h_metrics().advance_width).sum();
    pt_breit as f64 / PT_JE_MM
}

/// Grösste Schrift bis `max_pt`, bei der alle Zeilen in `platz` mm passen.
fn passende_groesse(font: &Font, zeilen: &[&str], platz: f64, max_pt: f64) -> f64 {
    let breiteste = zeilen.iter().map(|z| breite_mm(font, z, 1.0)).fold(0.0, f64::max);
    (platz / breiteste).min(max_pt)
}

fn rechteck(x: f64, y: f64, b: f64, h: f64) -> Line {
    let p = |x: f64, y: f64| (Point::new(Mm(x), Mm(y)), false);
    Line {
        points: vec![p(x, y), p(x + b, y), p(x + b, y + h), p(x, y + h)],
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    }
}

/// Schmales Schild: zwei Zeilen längs, von unten nach oben zu lesen.
#[allow(clippy::too_many_arguments)]
fn laengs(
    ebene: &PdfLayerReference,
    schrift: &IndirectFontRef,
    font: &Font,
    schild: &Schild,
    x: f64,
    y0: f64,
    breite: f64,
    hoehe: f64,
) {
    let zeilen = [
        "Vorträge Dr. med. Ursula Davatz".to_string(),
        format!("{} – {}", schild.von, schild.bis),
    ];
    let refs: Vec<&str> = zeilen.iter().map(String::as_str).collect();
    // Länge: 12 mm Rand oben und unten; Breite: zwei Zeilen zu 1,3 Grössen.
    let pt = passende_groesse(font, &refs, hoehe - 24.0, 30.0)
        .min((breite - 6.0) * PT_JE_MM / 2.6);
    let zeile = pt / PT_JE_MM * 1.3;
    let links = x + (breite - 2.0 * zeile) / 2.0;
    for (i, z) in zeilen.iter().enumerate() {
        let lang = breite_mm(font, z, pt);
        // Gedreht liegt die Oberlänge links der Grundlinie.
        let grundlinie = links + zeile * (i as f64) + zeile * 0.78;
        ebene.save_graphics_state();
        ebene.set_ctm(CurTransMat::Translate(Mm(grundlinie), Mm(y0 + (hoehe - lang) / 2.0)));
        ebene.set_ctm(CurTransMat::Rotate(90.0));
        ebene.use_text(z.as_str(), pt, Mm(0.0), Mm(0.0), schrift);
        ebene.restore_graphics_state();
    }
}

fn main() -> Result<()> {
    let o = optionen()?;
    let fontdir = std::env::var("FONT_DIR").unwrap_or_else(|_| "./fonts".into());
    let pfad = format!("{fontdir}/DejaVuSans-Bold.ttf");
    let daten = std::fs::read(&pfad).with_context(|| pfad.clone())?;
    let font = Font::from_bytes(daten.clone()).context("Schrift nicht lesbar")?;

    let (doc, seite, ebene) = PdfDocument::new("Ordnerrücken", Mm(A4_BREIT), Mm(A4_HOCH), "Ebene 1");
    let schrift = doc.add_external_font(std::io::Cursor::new(daten)).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    let ebene = doc.get_page(seite).get_layer(ebene);

    let mut x = (A4_BREIT - gesamtbreite(&o)) / 2.0;
    let y0 = (A4_HOCH - o.hoehe) / 2.0;
    let kopf = ["Vorträge", "Dr. med.", "Ursula", "Davatz"];

    for schild in &o.jahre {
        let breite = schild.breite.unwrap_or(o.breite);

        // Schnittlinie, gestrichelt
        ebene.set_outline_thickness(0.5);
        ebene.set_line_dash_pattern(LineDashPattern::new(0, Some(6), Some(4), None, None, None, None));
        ebene.add_shape(rechteck(x, y0, breite, o.hoehe));
        ebene.set_line_dash_pattern(LineDashPattern::new(0, None, None, None, None, None, None));

        if breite < 40.0 {
            laengs(&ebene, &schrift, &font, schild, x, y0, breite, o.hoehe);
            x += breite + ABSTAND;
            continue;
        }
        let platz = breite - 8.0;
        let mittig = |text: &str, pt: f64, y: f64| {
            let b = breite_mm(&font, text, pt);
            ebene.use_text(text, pt, Mm(x + (breite - b) / 2.0), Mm(y), &schrift);
        };

        // Kopf: vier Zeilen ab 25 mm unter der Oberkante
        let kopf_pt = passende_groesse(&font, &kopf, platz, 26.0);
        let zeile = kopf_pt / PT_JE_MM * 1.35;
        let mut y = y0 + o.hoehe - 25.0;
        for z in kopf {
            mittig(z, kopf_pt, y);
            y -= zeile;
        }

        // Jahre gross, untereinander
        let jahre = [schild.von.as_str(), "–", schild.bis.as_str()];
        let jahr_pt = passende_groesse(&font, &jahre, platz, 40.0);
        let jzeile = jahr_pt / PT_JE_MM * 1.25;
        let mut y = y0 + o.hoehe * 0.42;
        for z in jahre {
            mittig(z, jahr_pt, y);
            y -= jzeile;
        }
        x += breite + ABSTAND;
    }

    let datei = std::fs::File::create(&o.out).with_context(|| o.out.clone())?;
    doc.save(&mut BufWriter::new(datei)).map_err(|e| anyhow::anyhow!("{e:?}"))?;
    println!("{}: {} Schilder, {} mm hoch", o.out, o.jahre.len(), o.hoehe);
    Ok(())
}
