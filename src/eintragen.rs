// Trägt einen neuen Scan nach: Zusammenfassung alphabetisch in
// `src/zusammenfassungen.rs`, Zeile nach Datum in den Bestand der README,
// und die Zählzeile «N PDFs, M Seiten» aus `scan/` neu berechnet.
//
//   cargo run --release --bin eintragen -- --datei Titel_TT.MM.JJJJ.pdf \
//       --text "Zusammenfassung in drei bis fünf Sätzen." \
//       [--titel "Voller Titel"] [--bestand "Vermerk für die README"]
//   cargo run --release --bin eintragen -- --zaehlen   # nur die Zählzeile
//
// Ohne --bestand bleibt die README unberührt (nur die Zählzeile wird
// nachgeführt). Danach `cargo run --release --bin scanliste`.
//
// Lizenz: GPL-3.0

mod dateiname;

use std::path::Path;

use anyhow::{bail, Context, Result};
use dateiname::zerlegen;

const ZUSAMMENFASSUNGEN: &str = "src/zusammenfassungen.rs";
const README: &str = "README.md";
const SCAN_DIR: &str = "scan";

#[derive(Default)]
struct Optionen {
    datei: String,
    titel: String,
    text: String,
    bestand: Option<String>,
    nur_zaehlen: bool,
}

fn optionen() -> Result<Optionen> {
    let mut o = Optionen::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut wert = || args.next().with_context(|| format!("{a} braucht einen Wert"));
        match a.as_str() {
            "--datei" => o.datei = wert()?.trim_start_matches("scan/").to_string(),
            "--titel" => o.titel = wert()?,
            "--text" => o.text = wert()?,
            "--bestand" => o.bestand = Some(wert()?),
            "--zaehlen" => o.nur_zaehlen = true,
            "-h" | "--help" => {
                println!("eintragen --datei NAME.pdf --text TEXT [--titel TITEL] [--bestand VERMERK]\n\
                     eintragen --zaehlen   # nur die Zählzeile der README nachführen");
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    if o.nur_zaehlen {
        return Ok(o);
    }
    if o.datei.is_empty() || o.text.is_empty() {
        bail!("--datei und --text sind nötig");
    }
    if zerlegen(&o.datei).is_none() {
        bail!("Dateiname ohne Datum: {}", o.datei);
    }
    Ok(o)
}

/// Zeichenkette als Rust-Literal maskieren.
fn literal(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Fügt den Eintrag vor dem ersten alphabetisch späteren ein.
fn zusammenfassung_einfuegen(quelle: &str, datei: &str, titel: &str, text: &str) -> Result<String> {
    let kopf = "    Zusammenfassung {\n        datei: \"";
    if quelle.contains(&format!("{kopf}{}\"", literal(datei))) {
        bail!("{datei} steht schon in {ZUSAMMENFASSUNGEN}");
    }
    let neu = format!(
        "{kopf}{}\",\n        titel: \"{}\",\n        text: \"{}\",\n    }},\n",
        literal(datei),
        literal(titel),
        literal(text)
    );
    let mut stelle = None;
    for (i, _) in quelle.match_indices(kopf) {
        let rest = &quelle[i + kopf.len()..];
        let name = &rest[..rest.find('"').context("Eintrag ohne Ende")?];
        if name > datei {
            stelle = Some(i);
            break;
        }
    }
    let stelle = match stelle {
        Some(i) => i,
        None => quelle.rfind("];").context("Ende der Liste fehlt")?,
    };
    Ok(format!("{}{neu}{}", &quelle[..stelle], &quelle[stelle..]))
}

/// Sortierschlüssel einer Bestandszeile: Datum, dann Fassung.
fn schluessel(datei: &str) -> Option<((u32, u32, u32), u32)> {
    let (_, _, datum, fassung) = zerlegen(datei)?;
    Some((datum, fassung.unwrap_or(1)))
}

/// Setzt die Zeile hinter die letzte Bestandszeile mit gleichem oder
/// früherem Datum.
fn bestand_einfuegen(readme: &str, datei: &str, vermerk: &str) -> Result<String> {
    let neu_schluessel = schluessel(datei).context("Dateiname ohne Datum")?;
    let zeile = format!("| `{datei}` | {vermerk} |");
    let mut zeilen: Vec<&str> = readme.lines().collect();
    if zeilen.iter().any(|z| z.starts_with(&format!("| `{datei}`"))) {
        bail!("{datei} steht schon in der README");
    }
    let mut letzte_davor = None;
    let mut erste = None;
    for (n, z) in zeilen.iter().enumerate() {
        let Some(rest) = z.strip_prefix("| `") else { continue };
        let Some(name) = rest.split('`').next() else { continue };
        let Some(k) = schluessel(name) else { continue };
        erste.get_or_insert(n);
        if k <= neu_schluessel {
            letzte_davor = Some(n);
        }
    }
    let stelle = match (letzte_davor, erste) {
        (Some(n), _) => n + 1,
        (None, Some(n)) => n,
        (None, None) => bail!("kein Bestand in der README gefunden"),
    };
    zeilen.insert(stelle, &zeile);
    let mut aus = zeilen.join("\n");
    if readme.ends_with('\n') {
        aus.push('\n');
    }
    Ok(aus)
}

/// Zählt die PDFs in scan/ und ihre Seiten.
fn zaehlen(ordner: &Path) -> Result<(usize, usize)> {
    let mut pdfs = 0;
    let mut seiten = 0;
    for e in std::fs::read_dir(ordner)? {
        let p = e?.path();
        if p.extension().and_then(|x| x.to_str()) == Some("pdf") {
            let doc = lopdf::Document::load(&p).with_context(|| format!("{}", p.display()))?;
            pdfs += 1;
            seiten += doc.get_pages().len();
        }
    }
    Ok((pdfs, seiten))
}

/// Ersetzt die erste Zählzeile «N PDFs, M Seiten».
fn zaehlzeile(readme: &str, pdfs: usize, seiten: usize) -> Result<String> {
    let re = regex::Regex::new(r"\d+ PDFs, \d+ Seiten").unwrap();
    if !re.is_match(readme) {
        bail!("Zählzeile «N PDFs, M Seiten» fehlt in der README");
    }
    Ok(re.replace(readme, format!("{pdfs} PDFs, {seiten} Seiten").as_str()).into_owned())
}

fn main() -> Result<()> {
    let o = optionen()?;
    if o.nur_zaehlen {
        let (pdfs, seiten) = zaehlen(Path::new(SCAN_DIR))?;
        let readme = zaehlzeile(&std::fs::read_to_string(README)?, pdfs, seiten)?;
        std::fs::write(README, readme)?;
        println!("Bestand {pdfs} PDFs, {seiten} Seiten");
        return Ok(());
    }
    if !Path::new(SCAN_DIR).join(&o.datei).exists() {
        bail!("{SCAN_DIR}/{} gibt es nicht", o.datei);
    }

    let quelle = std::fs::read_to_string(ZUSAMMENFASSUNGEN)?;
    let quelle = zusammenfassung_einfuegen(&quelle, &o.datei, &o.titel, &o.text)?;

    let mut readme = std::fs::read_to_string(README)?;
    if let Some(v) = &o.bestand {
        readme = bestand_einfuegen(&readme, &o.datei, v)?;
    }
    let (pdfs, seiten) = zaehlen(Path::new(SCAN_DIR))?;
    readme = zaehlzeile(&readme, pdfs, seiten)?;

    // Erst schreiben, wenn beides gelungen ist.
    std::fs::write(ZUSAMMENFASSUNGEN, quelle)?;
    std::fs::write(README, readme)?;
    println!("{}: Zusammenfassung eingetragen{}, Bestand {pdfs} PDFs, {seiten} Seiten",
        o.datei, if o.bestand.is_some() { ", README-Zeile gesetzt" } else { "" });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTE: &str = "pub const Z: &[Zusammenfassung] = &[\n    Zusammenfassung {\n        datei: \"A_01.01.1990.pdf\",\n        titel: \"\",\n        text: \"a\",\n    },\n    Zusammenfassung {\n        datei: \"C_01.01.1990.pdf\",\n        titel: \"\",\n        text: \"c\",\n    },\n];\n";

    #[test]
    fn alphabetisch_einfuegen() {
        let s = zusammenfassung_einfuegen(LISTE, "B_02.02.1991.pdf", "", "Er sagte \"ja\".").unwrap();
        let a = s.find("A_01").unwrap();
        let b = s.find("B_02").unwrap();
        let c = s.find("C_01").unwrap();
        assert!(a < b && b < c);
        assert!(s.contains("Er sagte \\\"ja\\\"."));
        let z = zusammenfassung_einfuegen(LISTE, "Z_01.01.1990.pdf", "", "z").unwrap();
        assert!(z.find("Z_01").unwrap() > z.find("C_01").unwrap());
        assert!(zusammenfassung_einfuegen(LISTE, "A_01.01.1990.pdf", "", "x").is_err());
    }

    const BESTAND: &str = "### Bestand\n\n3 PDFs, 7 Seiten, nach Datum.\n\n| Datei | Seiten |\n|---|---|\n| `X_05.03.1986.pdf` | 1 |\n| `Y_12.1986.pdf` | 2 |\n| `Z_10.01.1987.pdf` | 4 |\n\nDanach\n";

    #[test]
    fn nach_datum_einfuegen() {
        let s = bestand_einfuegen(BESTAND, "N_20.11.1986.pdf", "3 (neu)").unwrap();
        let x = s.find("X_05").unwrap();
        let n = s.find("N_20").unwrap();
        let y = s.find("Y_12").unwrap();
        assert!(x < n && n < y);
        let v = bestand_einfuegen(BESTAND, "V_01.01.1980.pdf", "1").unwrap();
        assert!(v.find("V_01").unwrap() < v.find("X_05").unwrap());
        let f = bestand_einfuegen(BESTAND, "X_05.03.1986_2.pdf", "1").unwrap();
        assert!(f.find("X_05.03.1986_2").unwrap() > f.find("X_05.03.1986.pdf").unwrap());
        assert!(f.ends_with("Danach\n"));
    }

    #[test]
    fn zaehlzeile_ersetzen() {
        let s = zaehlzeile(BESTAND, 4, 9).unwrap();
        assert!(s.contains("4 PDFs, 9 Seiten, nach Datum."));
    }
}
