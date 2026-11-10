// Druckt die erste Seite eines PDFs als PCL-Rasterbild (300 dpi, schwarz-
// weiss) auf einen Drucker ohne PDF-Verständnis, etwa einen HP LaserJet
// 2100 am Port 9100. Ein PDF direkt an so einen Drucker geschickt, wird als
// Text gedruckt: Seite um Seite Zeichensalat.
//
//   cargo run --release --bin pcldruck -- datei.pdf --an HOST:9100
//   cargo run --release --bin pcldruck -- datei.pdf --datei seite.pcl   # nur schreiben
//
// Gerendert wird mit `pdftoppm -mono` (Poppler). Querformat-Seiten werden
// um 90° gedreht und hochkant gedruckt. Der Drucker beginnt erst rund
// 6 mm vom linken Blattrand; um so viel wird das Bild links beschnitten,
// damit es auf dem Blatt an derselben Stelle steht wie im PDF.
//
// Lizenz: GPL-3.0

use std::io::Write;
use std::process::Command;

use anyhow::{bail, Context, Result};

const DPI: usize = 300;
/// Unbedruckbarer linker Rand des LaserJet bei A4 hochkant (1/4 Zoll).
const RAND_LINKS: usize = 75;

struct Bild {
    breite: usize,
    hoehe: usize,
    /// true = schwarz
    punkte: Vec<bool>,
}

/// Liest ein PBM (P4) ein.
fn pbm_lesen(daten: &[u8]) -> Result<Bild> {
    let mut felder = Vec::new();
    let mut i = 0;
    while felder.len() < 3 {
        while daten[i].is_ascii_whitespace() {
            i += 1;
        }
        if daten[i] == b'#' {
            while daten[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        let start = i;
        while !daten[i].is_ascii_whitespace() {
            i += 1;
        }
        felder.push(std::str::from_utf8(&daten[start..i])?.to_string());
    }
    if felder[0] != "P4" {
        bail!("kein PBM P4");
    }
    let breite: usize = felder[1].parse()?;
    let hoehe: usize = felder[2].parse()?;
    let roh = &daten[i + 1..];
    let zeile = breite.div_ceil(8);
    if roh.len() < zeile * hoehe {
        bail!("PBM zu kurz");
    }
    let mut punkte = vec![false; breite * hoehe];
    for y in 0..hoehe {
        for x in 0..breite {
            punkte[y * breite + x] = roh[y * zeile + x / 8] & (0x80 >> (x % 8)) != 0;
        }
    }
    Ok(Bild { breite, hoehe, punkte })
}

/// Dreht um 90° im Uhrzeigersinn.
fn drehen(b: &Bild) -> Bild {
    let (breite, hoehe) = (b.hoehe, b.breite);
    let mut punkte = vec![false; breite * hoehe];
    for y in 0..b.hoehe {
        for x in 0..b.breite {
            punkte[x * breite + (b.hoehe - 1 - y)] = b.punkte[y * b.breite + x];
        }
    }
    Bild { breite, hoehe, punkte }
}

/// Eine Seite PCL 5: A4 hochkant, Rasterbild ab der linken oberen Ecke.
fn pcl(b: &Bild, links_weg: usize) -> Vec<u8> {
    let esc = 0x1b_u8;
    let mut aus = Vec::new();
    let befehl = |aus: &mut Vec<u8>, s: &str| {
        aus.push(esc);
        aus.extend_from_slice(s.as_bytes());
    };
    befehl(&mut aus, "E"); // zurücksetzen
    befehl(&mut aus, "&l26A"); // A4
    befehl(&mut aus, "&l0O"); // hochkant
    befehl(&mut aus, "&l0E"); // kein oberer Rand
    befehl(&mut aus, "*p0x0Y");
    befehl(&mut aus, &format!("*t{DPI}R"));
    let breite = b.breite - links_weg;
    befehl(&mut aus, &format!("*r{breite}S"));
    befehl(&mut aus, "*r0A");
    let bytes = breite.div_ceil(8);
    for y in 0..b.hoehe {
        let mut zeile = vec![0u8; bytes];
        for x in 0..breite {
            if b.punkte[y * b.breite + x + links_weg] {
                zeile[x / 8] |= 0x80 >> (x % 8);
            }
        }
        let n = zeile.iter().rposition(|&v| v != 0).map_or(0, |p| p + 1);
        befehl(&mut aus, &format!("*b{n}W"));
        aus.extend_from_slice(&zeile[..n]);
    }
    befehl(&mut aus, "*rB");
    befehl(&mut aus, "E"); // Seite ausgeben
    aus
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut pdf = None;
    let mut an = None;
    let mut datei = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--an" => an = Some(args.next().context("--an braucht HOST:PORT")?),
            "--datei" => datei = Some(args.next().context("--datei braucht einen Pfad")?),
            "-h" | "--help" => {
                println!("pcldruck DATEI.pdf (--an HOST:9100 | --datei SEITE.pcl)");
                return Ok(());
            }
            _ if pdf.is_none() => pdf = Some(a),
            _ => bail!("unbekanntes Argument {a:?}"),
        }
    }
    let pdf = pdf.context("PDF fehlt")?;
    if an.is_none() && datei.is_none() {
        bail!("--an oder --datei fehlt");
    }

    // Nur die erste Seite – nie mehr als ein Blatt.
    let aus = Command::new("pdftoppm")
        .args(["-mono", "-r", &DPI.to_string(), "-f", "1", "-l", "1", &pdf])
        .output()
        .context("pdftoppm")?;
    if !aus.status.success() {
        bail!("pdftoppm: {}", String::from_utf8_lossy(&aus.stderr));
    }
    let mut bild = pbm_lesen(&aus.stdout)?;
    if bild.breite > bild.hoehe {
        bild = drehen(&bild);
    }
    let daten = pcl(&bild, RAND_LINKS.min(bild.breite));

    if let Some(p) = &datei {
        std::fs::write(p, &daten).with_context(|| p.clone())?;
        println!("{p}: 1 Seite PCL, {} B", daten.len());
    }
    if let Some(ziel) = &an {
        let mut s = std::net::TcpStream::connect(ziel).with_context(|| ziel.clone())?;
        s.write_all(&daten)?;
        s.flush()?;
        println!("an {ziel}: 1 Seite PCL, {} B", daten.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pbm_und_drehen() {
        // 3 × 2: obere Zeile schwarz-weiss-schwarz
        let roh = b"P4\n3 2\n\xa0\x00";
        let b = pbm_lesen(roh).unwrap();
        assert_eq!(b.punkte, vec![true, false, true, false, false, false]);
        let d = drehen(&b);
        assert_eq!((d.breite, d.hoehe), (2, 3));
        // im Uhrzeigersinn: obere Zeile wird rechte Spalte
        assert!(d.punkte[1] && !d.punkte[3] && d.punkte[5]);
    }

    #[test]
    fn pcl_eine_seite() {
        let b = Bild { breite: 16, hoehe: 2, punkte: vec![true; 32] };
        let p = pcl(&b, 0);
        assert!(p.starts_with(b"\x1bE"));
        assert!(p.ends_with(b"\x1b*rB\x1bE"));
        // genau ein Seitenvorschub am Schluss
        assert_eq!(p.windows(2).filter(|w| w == b"\x1bE").count(), 2);
    }
}
