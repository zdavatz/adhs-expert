// Bringt die Scans aus `scan/` auf adhs.expert: PDF in die Mediathek, Beitrag
// mit Zusammenfassung, Kategorien und Tags, Eintrag in `src/medien.rs`.
//
//   cargo run --release --bin webscan -- --scan Titel_TT.MM.JJJJ.pdf
//   cargo run --release --bin webscan -- --alle
//   cargo run --release --bin webscan -- --verzeichnis
//   cargo run --release --bin webscan -- --alle --trocken
//
// Zugang: `~/.config/adhs-expert/credentials` mit `WP_SITE`, `WP_USER` und
// `WP_APP_PASSWORD` (Anwendungspasswort, nicht das Kontopasswort). Pfad über
// `$ADHS_WP_CREDENTIALS` überschreibbar. Die Datei gehört nicht ins
// Repository - es ist öffentlich.
//
// **Die Doppel-Upload-Sperre ist der Kern dieses Programms.** Im November 2026
// liefen zwei Uploader parallel und legten 429 Dubletten in der Mediathek an,
// die einzeln wieder weg mussten; einen Papierkorb gibt es für Medien nicht.
// Darum gilt: steht eine Datei in `src/medien.rs`, wird sie nie wieder
// hochgeladen, und `--alle` arbeitet strikt der Reihe nach in einem Prozess.
//
// Lizenz: GPL-3.0

mod dateiname;
mod medien;
mod zusammenfassungen;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};

/// Beiträge erscheinen unter diesem Benutzer, nicht unter dem Konto, mit dem
/// das Anwendungspasswort ausgestellt ist (Wunsch des Benutzers).
const AUTOR: u64 = 15_361_161; // udavatz
const KAT_VORTRAG: u64 = 199_287;
const KAT_ARCHIV: u64 = 716_265_248;

const SCAN_DIR: &str = "scan";
const MEDIEN_RS: &str = "src/medien.rs";
const VERZEICHNIS_PRAEFIX: &str = "Vorträge_Dr.med._Ursula_Davatz_von_";
const VERZEICHNIS_SLUG: &str = "uebersicht-aller-gescannten-vortraege-und-kursunterlagen-als-pdf";

/// Marke im Beitrag, damit ein zweiter Lauf den Block findet statt ihn zu doppeln.
const MARKE: &str = "<!-- scan-pdf -->";

const MAX_TAGS: usize = 8;
const MAX_SACHKATEGORIEN: usize = 4;
/// Nur eingespielte Begriffe als Treffer zulassen - sonst zieht der Lauf
/// Einmal-Tags herein, die niemand sucht.
const TAG_MIN_ZAHL: u64 = 5;
const KAT_MIN_ZAHL: u64 = 3;
const MIN_LAENGE: usize = 4;

/// Zu unspezifisch, um aus dem Text erkannt zu werden.
const SPERRE: &[&str] = &[
    "umgang", "umfeld", "mutter", "vater", "kind", "frau", "mann", "zeit", "rolle", "arbeit",
    "leben", "menschen", "thema", "teil", "vortrag", "vortraege", "system", "uncategorized",
    "zusammenfassung", "podcast", "youtube", "buch", "book", "archiv",
];
/// Zusätzlich für Kategorien: unspezifisch oder englisch doppelt.
const SPERRE_KAT: &[&str] = &[
    "kampf", "mensch", "individuum", "herausforderung", "recht", "therapie", "zusammenarbeit",
    "lika", "adhs20+", "aarau", "aargau", "adhd", "add", "schizophrenia", "pubertierend",
];
/// Taugt nicht als Tag aus dem Titel.
const STOPP: &[&str] = &[
    "eine", "einer", "eines", "einem", "einen", "oder", "und", "der", "die", "das", "den", "dem",
    "des", "bei", "mit", "von", "vom", "fuer", "ihre", "ihren", "ihrem", "sich", "diese", "dieser",
    "dieses", "nicht", "sind", "wird", "werden", "kann", "koennen", "lassen", "was", "warum",
    "welche", "aus", "auf", "als", "ist", "sowie", "zwischen", "handschrift", "reinschrift",
    "abschrift", "kurs", "ihrer", "unserer", "unsere",
];

const MONATE: [&str; 12] = [
    "Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober",
    "November", "Dezember",
];

// ---------------------------------------------------------------- Optionen

#[derive(Default)]
struct Optionen {
    scans: Vec<String>,
    alle: bool,
    verzeichnis: bool,
    nur_hochladen: bool,
    trocken: bool,
}

fn optionen() -> Result<Optionen> {
    let mut o = Optionen::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut wert = || args.next().with_context(|| format!("{a} braucht einen Wert"));
        match a.as_str() {
            "--scan" => o.scans.push(wert()?),
            "--alle" => o.alle = true,
            "--verzeichnis" => o.verzeichnis = true,
            "--nur-hochladen" => o.nur_hochladen = true,
            "--trocken" => o.trocken = true,
            "-h" | "--help" => {
                println!(
                    "webscan (--scan DATEI … | --alle | --verzeichnis) [--nur-hochladen] [--trocken]\n\
                     \n\
                     --scan DATEI      ein Scan aus scan/ (mehrfach möglich)\n\
                     --alle            alle Scans, die noch nicht in src/medien.rs stehen\n\
                     --verzeichnis     das Übersichts-PDF neu hochladen und den Link\n\
                     \x20                 im erklärenden Beitrag nachziehen\n\
                     --nur-hochladen   PDF hochladen und medien.rs ergänzen, keinen Beitrag anlegen\n\
                     --trocken         nur zeigen, was geschähe"
                );
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    if !o.alle && !o.verzeichnis && o.scans.is_empty() {
        bail!("nichts zu tun - --scan, --alle oder --verzeichnis angeben (--help)");
    }
    Ok(o)
}

// ---------------------------------------------------------------- Zugang

struct Zugang {
    site: String,
    auth: String,
}

impl Zugang {
    fn laden() -> Result<Self> {
        let pfad = match std::env::var("ADHS_WP_CREDENTIALS") {
            Ok(p) => PathBuf::from(p),
            Err(_) => {
                let heim = std::env::var("HOME").context("$HOME fehlt")?;
                PathBuf::from(heim).join(".config/adhs-expert/credentials")
            }
        };
        let inhalt = std::fs::read_to_string(&pfad)
            .with_context(|| format!("{} nicht lesbar", pfad.display()))?;
        let feld = |name: &str| -> Result<String> {
            for zeile in inhalt.lines() {
                let zeile = zeile.trim();
                if zeile.starts_with('#') {
                    continue;
                }
                if let Some((k, v)) = zeile.split_once('=') {
                    if k.trim() == name {
                        return Ok(v.trim().trim_matches('"').trim_matches('\'').to_string());
                    }
                }
            }
            bail!("{name} fehlt in {}", pfad.display())
        };
        let site = feld("WP_SITE")?.trim_end_matches('/').to_string();
        let auth = STANDARD.encode(format!("{}:{}", feld("WP_USER")?, feld("WP_APP_PASSWORD")?));
        Ok(Zugang { site, auth })
    }

    fn kopf(&self) -> String {
        format!("Basic {}", self.auth)
    }

    fn hole(&self, pfad: &str) -> Result<Value> {
        mit_wiederholung(&format!("GET {pfad}"), || {
            ureq::get(&format!("{}{pfad}", self.site))
                .set("Authorization", &self.kopf())
                .set("User-Agent", "adhs-expert/webscan")
                .call()
        })
    }

    fn schreibe(&self, pfad: &str, daten: &Value) -> Result<Value> {
        mit_wiederholung(&format!("POST {pfad}"), || {
            ureq::post(&format!("{}{pfad}", self.site))
                .set("Authorization", &self.kopf())
                .set("User-Agent", "adhs-expert/webscan")
                .send_json(daten.clone())
        })
    }

    /// Lädt ein PDF in die Mediathek und gibt (ID, Adresse) zurück.
    fn hochladen(&self, pfad: &Path, name: &str) -> Result<(u64, String)> {
        let daten = std::fs::read(pfad).with_context(|| format!("{} nicht lesbar", pfad.display()))?;
        let j = mit_wiederholung(&format!("Upload {name}"), || {
            ureq::post(&format!("{}/wp-json/wp/v2/media", self.site))
                .set("Authorization", &self.kopf())
                .set("User-Agent", "adhs-expert/webscan")
                .set("Content-Type", "application/pdf")
                .set("Content-Disposition", &format!("attachment; filename=\"{name}\""))
                .send_bytes(&daten)
        })?;
        let id = j["id"].as_u64().context("Upload ohne id")?;
        let url = j["source_url"].as_str().context("Upload ohne source_url")?.to_string();
        Ok((id, url))
    }
}

/// WordPress.com drosselt mit 429. Abbrechen wäre falsch - warten und noch
/// einmal, mit sich verdoppelnder Pause.
fn mit_wiederholung<F>(was: &str, mut ruf: F) -> Result<Value>
where
    F: FnMut() -> Result<ureq::Response, ureq::Error>,
{
    let mut warte = 15u64;
    for versuch in 0..6 {
        match ruf() {
            Ok(r) => return Ok(r.into_json()?),
            Err(ureq::Error::Status(code, r)) => {
                let text = r.into_string().unwrap_or_default();
                let wiederholbar = matches!(code, 429 | 502 | 503 | 504);
                if wiederholbar && versuch < 5 {
                    eprintln!("   … {was}: HTTP {code}, warte {warte} s");
                    std::thread::sleep(Duration::from_secs(warte));
                    warte = (warte * 2).min(180);
                    continue;
                }
                bail!("{was}: HTTP {code}: {}", text.chars().take(300).collect::<String>());
            }
            Err(e) => return Err(e).context(was.to_string()),
        }
    }
    unreachable!()
}

// ---------------------------------------------------------------- Namen, Text

/// Dateiname für die Mediathek: WordPress nimmt nur ASCII.
fn medienname(datei: &str) -> String {
    let stamm = datei.strip_suffix(".pdf").unwrap_or(datei);
    let mut out = String::new();
    for c in stamm.chars() {
        match c {
            'ä' => out.push_str("ae"),
            'ö' => out.push_str("oe"),
            'ü' => out.push_str("ue"),
            'Ä' => out.push_str("Ae"),
            'Ö' => out.push_str("Oe"),
            'Ü' => out.push_str("Ue"),
            'ß' => out.push_str("ss"),
            'é' | 'è' | 'ê' => out.push('e'),
            'à' | 'â' => out.push('a'),
            'ç' => out.push('c'),
            c if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' => out.push(c),
            _ => out.push('_'),
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    format!("{}.pdf", out.trim_matches('_'))
}

/// Kleinschreibung, Umlaute aufgelöst, alles andere zu Leerzeichen; vorn und
/// hinten ein Leerzeichen, damit `contains(" wort ")` auf Wortgrenzen prüft.
fn normalisieren(s: &str) -> String {
    let mut t = String::from(" ");
    for c in s.to_lowercase().replace("(h)", "h").chars() {
        match c {
            'ä' => t.push_str("ae"),
            'ö' => t.push_str("oe"),
            'ü' => t.push_str("ue"),
            'ß' => t.push_str("ss"),
            c if c.is_ascii_alphanumeric() => t.push(c),
            _ => t.push(' '),
        }
    }
    t.push(' ');
    while t.contains("  ") {
        t = t.replace("  ", " ");
    }
    t
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn datum_lesbar(datum: &str) -> Result<String> {
    let z: Vec<u32> = datum.split('.').map(|x| x.parse().ok()).collect::<Option<_>>()
        .with_context(|| format!("Datum {datum:?} nicht lesbar"))?;
    match z[..] {
        [t, m, j] => Ok(format!("{t}. {} {j}", MONATE[(m as usize) - 1])),
        [m, j] => Ok(format!("{} {j}", MONATE[(m as usize) - 1])),
        _ => bail!("Datum {datum:?} nicht lesbar"),
    }
}

fn datum_iso(datum: &str) -> Result<String> {
    let z: Vec<u32> = datum.split('.').map(|x| x.parse().ok()).collect::<Option<_>>()
        .with_context(|| format!("Datum {datum:?} nicht lesbar"))?;
    match z[..] {
        [t, m, j] => Ok(format!("{j:04}-{m:02}-{t:02}T09:00:00")),
        [m, j] => Ok(format!("{j:04}-{m:02}-01T09:00:00")),
        _ => bail!("Datum {datum:?} nicht lesbar"),
    }
}

fn jahr_von(datum: &str) -> Result<String> {
    datum.split('.').next_back().map(str::to_string).context("Jahr fehlt im Datum")
}

/// Der Block mit dem PDF-Link. **Fett, nie `<h2>`** - eine Überschrift wäre
/// grösser als der Beitragstitel, was der Benutzer nicht will.
fn pdf_block(urls: &[String]) -> String {
    let links = urls
        .iter()
        .map(|u| {
            let name = u.rsplit('/').next().unwrap_or(u);
            format!("<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>", html_escape(u), html_escape(name))
        })
        .collect::<Vec<_>>()
        .join(" und ");
    let satz = if urls.len() == 1 {
        "Die gescannte Vorlage liegt als durchsuchbares PDF vor: "
    } else {
        "Die gescannten Vorlagen liegen als durchsuchbare PDF vor: "
    };
    format!("{MARKE}\n<p><strong>Unterlagen als PDF</strong></p>\n<p>{satz}{links}.</p>\n")
}

fn beitrag_inhalt(datum: &str, text: &str, url: &str) -> Result<String> {
    Ok(format!(
        "<p><em>{}</em></p>\n<p>{}</p>\n\n{}",
        html_escape(&datum_lesbar(datum)?),
        html_escape(text),
        pdf_block(std::slice::from_ref(&url.to_string()))
    ))
}

// ---------------------------------------------------------------- Taxonomie

struct Begriff {
    id: u64,
    name: String,
    norm: String,
}

struct Taxonomie {
    kategorien: Vec<Begriff>,
    tags: Vec<Begriff>,
    tags_alle: Vec<Begriff>,
    /// Name -> ID über alle Kategorien, auch die mit Zähler 0 (Jahre).
    alle_kategorien: Vec<(String, u64)>,
}

impl Taxonomie {
    fn laden(z: &Zugang) -> Result<Self> {
        let kat_roh = liste(z, "categories")?;
        let tag_roh = liste(z, "tags")?;
        let gesperrt = |n: &str, auch_kat: bool| {
            let k = normalisieren(n).trim().to_string();
            SPERRE.contains(&k.as_str())
                || (auch_kat && SPERRE_KAT.contains(&k.as_str()))
                // Jahreszahlen kommen aus dem Datum, nie aus dem Text: eine
                // Zusammenfassung nennt oft Jahre («ab 1986 in Königsfelden»)
                // und würde sonst die falsche Jahres-Kategorie setzen.
                || (auch_kat && n.len() == 4 && n.chars().all(|c| c.is_ascii_digit()))
        };
        let filtern = |roh: &[(String, u64, u64)], min: u64, auch_kat: bool| -> Vec<Begriff> {
            roh.iter()
                .filter(|(n, _, z)| *z >= min && n.chars().count() >= MIN_LAENGE && !gesperrt(n, auch_kat))
                .map(|(n, id, _)| Begriff { id: *id, name: n.clone(), norm: normalisieren(n) })
                .collect()
        };
        Ok(Taxonomie {
            kategorien: filtern(&kat_roh, KAT_MIN_ZAHL, true),
            tags: filtern(&tag_roh, TAG_MIN_ZAHL, false),
            tags_alle: filtern(&tag_roh, 1, false),
            alle_kategorien: kat_roh.into_iter().map(|(n, id, _)| (n, id)).collect(),
        })
    }

    /// Jahres-Kategorie finden oder anlegen.
    fn jahr(&mut self, z: &Zugang, jahr: &str, trocken: bool) -> Result<u64> {
        if let Some((_, id)) = self.alle_kategorien.iter().find(|(n, _)| n == jahr) {
            return Ok(*id);
        }
        if trocken {
            println!("    (Kategorie {jahr} würde angelegt)");
            return Ok(0);
        }
        let r = z.schreibe("/wp-json/wp/v2/categories", &json!({ "name": jahr }))?;
        let id = neue_id(&r).with_context(|| format!("Kategorie {jahr} anlegen"))?;
        self.alle_kategorien.push((jahr.to_string(), id));
        println!("    Kategorie {jahr} angelegt (id {id})");
        Ok(id)
    }

    fn tag_id(&mut self, z: &Zugang, name: &str, trocken: bool) -> Result<u64> {
        let n = normalisieren(name);
        if let Some(b) = self.tags_alle.iter().find(|b| b.norm == n) {
            return Ok(b.id);
        }
        if trocken {
            println!("    (Tag «{name}» würde angelegt)");
            return Ok(0);
        }
        let r = z.schreibe("/wp-json/wp/v2/tags", &json!({ "name": name }))?;
        let id = neue_id(&r).with_context(|| format!("Tag {name:?} anlegen"))?;
        self.tags_alle.push(Begriff { id, name: name.to_string(), norm: n });
        println!("    Tag «{name}» angelegt (id {id})");
        Ok(id)
    }
}

/// `term_exists` ist kein Fehler: die ID steht dann unter `data.term_id`.
fn neue_id(r: &Value) -> Option<u64> {
    r["id"].as_u64().or_else(|| r["data"]["term_id"].as_u64())
}

fn liste(z: &Zugang, art: &str) -> Result<Vec<(String, u64, u64)>> {
    let mut out = Vec::new();
    for seite in 1..=30 {
        let j = z.hole(&format!(
            "/wp-json/wp/v2/{art}?per_page=100&page={seite}&_fields=id,name,count"
        ))?;
        let a = j.as_array().context("Liste ist kein Array")?;
        for t in a {
            let name = t["name"].as_str().unwrap_or("").trim().to_string();
            match (t["id"].as_u64(), t["count"].as_u64()) {
                (Some(id), Some(zahl)) if !name.is_empty() => out.push((name, id, zahl)),
                _ => {}
            }
        }
        if a.len() < 100 {
            break;
        }
    }
    Ok(out)
}

/// Treffer im Text, lange und häufige zuerst; enthaltene Begriffe fallen weg
/// («Familie» neben «Familientherapie»).
fn treffer(text: &str, begriffe: &[Begriff], hoechstens: usize) -> Vec<(u64, String)> {
    let t = normalisieren(text);
    let mut gefunden: Vec<&Begriff> = begriffe
        .iter()
        .filter(|b| t.contains(b.norm.as_str()))
        .collect();
    gefunden.sort_by(|a, b| b.name.chars().count().cmp(&a.name.chars().count()));
    let mut out: Vec<(u64, String)> = Vec::new();
    for b in gefunden {
        if out.iter().any(|(_, n)| {
            let gewaehlt = normalisieren(n);
            gewaehlt != b.norm && gewaehlt.contains(b.norm.trim())
        }) {
            continue;
        }
        out.push((b.id, b.name.clone()));
        if out.len() >= hoechstens {
            break;
        }
    }
    out
}

/// Hauptwörter aus dem Titel, als letzte Reserve für den Mindestbestand an Tags.
fn titelwoerter(titel: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for wort in titel.split(|c: char| !(c.is_alphanumeric() || c == '-')) {
        let w = wort.trim_matches('-');
        if w.chars().count() < 5 || !w.chars().next().is_some_and(char::is_uppercase) {
            continue;
        }
        let k = normalisieren(w).trim().to_string();
        if STOPP.contains(&k.as_str()) || SPERRE.contains(&k.as_str()) {
            continue;
        }
        if !out.iter().any(|x| x == w) {
            out.push(w.to_string());
        }
    }
    out.sort_by_key(|w| std::cmp::Reverse(w.chars().count()));
    out
}

struct Vergabe {
    kategorien: Vec<u64>,
    tags: Vec<u64>,
    kat_namen: Vec<String>,
    tag_namen: Vec<String>,
}

/// Mindestens drei Kategorien (Vortrag, Archiv, Publikationsjahr) und
/// mindestens zwei Tags - Vorgabe des Benutzers.
fn vergeben(
    z: &Zugang,
    taxo: &mut Taxonomie,
    titel: &str,
    text: &str,
    datum: &str,
    trocken: bool,
) -> Result<Vergabe> {
    let volltext = format!("{titel} {text}");
    let jahr = jahr_von(datum)?;
    let jahr_id = taxo.jahr(z, &jahr, trocken)?;

    let mut kategorien = vec![KAT_VORTRAG, KAT_ARCHIV, jahr_id];
    let mut kat_namen = vec!["Vortrag".to_string(), "Archiv".to_string(), jahr];
    for (id, name) in treffer(&volltext, &taxo.kategorien, MAX_SACHKATEGORIEN) {
        if !kategorien.contains(&id) {
            kategorien.push(id);
            kat_namen.push(name);
        }
    }

    let mut tags: Vec<u64> = Vec::new();
    let mut tag_namen: Vec<String> = Vec::new();
    for (id, name) in treffer(&volltext, &taxo.tags, MAX_TAGS) {
        tags.push(id);
        tag_namen.push(name);
    }
    if tags.len() < 2 {
        // zweiter Anlauf im ganzen Bestand, bevor etwas erfunden wird
        for (id, name) in treffer(&volltext, &taxo.tags_alle, 4) {
            if tags.len() >= 2 {
                break;
            }
            if !tags.contains(&id) {
                tags.push(id);
                tag_namen.push(name);
            }
        }
    }
    for wort in titelwoerter(titel) {
        if tags.len() >= 2 {
            break;
        }
        let id = taxo.tag_id(z, &wort, trocken)?;
        if id != 0 && !tags.contains(&id) {
            tags.push(id);
            tag_namen.push(wort);
        } else if trocken {
            tag_namen.push(wort);
        }
    }
    if !trocken && tags.len() < 2 {
        bail!("für «{titel}» kamen nur {} Tags zusammen - von Hand nachhelfen", tags.len());
    }
    Ok(Vergabe { kategorien, tags, kat_namen, tag_namen })
}

// ---------------------------------------------------------------- medien.rs

/// Baut den neuen Inhalt von `medien.rs`: `(datei, url)` an der alphabetisch
/// richtigen Stelle, Kopf und Schluss unverändert.
fn medien_tabelle(inhalt: &str, neu: &[(String, String)]) -> Result<String> {
    let (kopf, rest) = inhalt
        .split_once("pub const MEDIEN: &[(&str, &str)] = &[\n")
        .context("MEDIEN-Tabelle in medien.rs nicht gefunden")?;
    let (tabelle, schluss) = rest.rsplit_once("];").context("Ende der MEDIEN-Tabelle fehlt")?;

    let mut zeilen: Vec<String> =
        tabelle.lines().filter(|z| !z.trim().is_empty()).map(str::to_string).collect();
    for (datei, url) in neu {
        if datei.contains('"') || url.contains('"') {
            bail!("Anführungszeichen in {datei:?} oder {url:?}");
        }
        if !url.starts_with("https://adhs.expert/") {
            bail!("{url} liegt nicht auf adhs.expert");
        }
        zeilen.push(format!("    (\"{datei}\", \"{url}\"),"));
    }
    zeilen.sort_by_key(|z| z.trim_start().to_string());
    zeilen.dedup();
    Ok(format!(
        "{kopf}pub const MEDIEN: &[(&str, &str)] = &[\n{}\n];{schluss}",
        zeilen.join("\n")
    ))
}

fn medien_ergaenzen(neu: &[(String, String)]) -> Result<()> {
    if neu.is_empty() {
        return Ok(());
    }
    let inhalt = std::fs::read_to_string(MEDIEN_RS).with_context(|| format!("{MEDIEN_RS} fehlt"))?;
    let neu_inhalt = medien_tabelle(&inhalt, neu)?;
    std::fs::write(MEDIEN_RS, neu_inhalt).with_context(|| format!("{MEDIEN_RS} schreiben"))?;
    println!("  {MEDIEN_RS}: {} Zuordnung(en) ergänzt", neu.len());
    Ok(())
}

// ---------------------------------------------------------------- Arbeit

fn zusammenfassung(datei: &str) -> Option<&'static zusammenfassungen::Zusammenfassung> {
    zusammenfassungen::ZUSAMMENFASSUNGEN.iter().find(|z| z.datei == datei)
}

/// Gibt es den Beitrag schon? Verglichen wird über Titel und Datum - der Slug
/// weicht ab, weil WordPress Umlaute auflöst und an Doppel `-2` hängt.
fn beitrag_vorhanden(z: &Zugang, titel: &str, datum: &str) -> Result<Option<(u64, String)>> {
    let iso = datum_iso(datum)?;
    let tag = &iso[..10];
    let j = z.hole(&format!(
        "/wp-json/wp/v2/posts?per_page=100&after={tag}T00:00:00&before={tag}T23:59:59\
         &status=publish,draft&_fields=id,link,title"
    ))?;
    let gesucht = normalisieren(titel);
    for p in j.as_array().map(Vec::as_slice).unwrap_or_default() {
        let t = p["title"]["rendered"].as_str().unwrap_or("");
        if normalisieren(t) == gesucht {
            return Ok(Some((
                p["id"].as_u64().unwrap_or(0),
                p["link"].as_str().unwrap_or("").to_string(),
            )));
        }
    }
    Ok(None)
}

fn einen_scan(z: &Zugang, taxo: &mut Taxonomie, datei: &str, o: &Optionen) -> Result<Option<(String, String)>> {
    let pfad = Path::new(SCAN_DIR).join(datei);
    if !pfad.is_file() {
        bail!("{} gibt es nicht", pfad.display());
    }
    let zf = zusammenfassung(datei)
        .with_context(|| format!("keine Zusammenfassung für {datei} - erst `eintragen` laufen lassen"))?;
    let (titel_aus_name, datum, _, _) =
        dateiname::zerlegen(datei).with_context(|| format!("Dateiname {datei:?} passt nicht"))?;
    let titel = if zf.titel.is_empty() { titel_aus_name } else { zf.titel.to_string() };

    // Doppel-Upload-Sperre: steht die Datei schon in medien.rs, bleibt sie dort.
    if let Some(url) = medien::url(datei) {
        println!("  {datei}\n    liegt schon in der Mediathek: {url}");
        if o.nur_hochladen {
            return Ok(None);
        }
        if let Some((id, link)) = beitrag_vorhanden(z, &titel, &datum)? {
            println!("    Beitrag besteht schon (id {id}): {link}");
            return Ok(None);
        }
        let v = vergeben(z, taxo, &titel, zf.text, &datum, o.trocken)?;
        beitrag_anlegen(z, &titel, &datum, zf.text, url, &v, o.trocken)?;
        return Ok(None);
    }

    let name = medienname(datei);
    println!("  {datei}\n    → {name}");
    if o.trocken {
        let v = vergeben(z, taxo, &titel, zf.text, &datum, true)?;
        println!("    Titel     : {titel}");
        println!("    Kategorien: {}", v.kat_namen.join(", "));
        println!("    Tags      : {}", v.tag_namen.join(", "));
        return Ok(None);
    }

    let (id, url) = z.hochladen(&pfad, &name)?;
    println!("    hochgeladen (Medien-ID {id}): {url}");
    if !o.nur_hochladen {
        let v = vergeben(z, taxo, &titel, zf.text, &datum, false)?;
        beitrag_anlegen(z, &titel, &datum, zf.text, &url, &v, false)?;
    }
    Ok(Some((datei.to_string(), url)))
}

fn beitrag_anlegen(
    z: &Zugang,
    titel: &str,
    datum: &str,
    text: &str,
    url: &str,
    v: &Vergabe,
    trocken: bool,
) -> Result<()> {
    if trocken {
        println!("    (Beitrag würde angelegt: {titel})");
        return Ok(());
    }
    let r = z.schreibe(
        "/wp-json/wp/v2/posts",
        &json!({
            "title": titel,
            "date": datum_iso(datum)?,
            "content": beitrag_inhalt(datum, text, url)?,
            "status": "publish",
            "author": AUTOR,
            "categories": v.kategorien,
            "tags": v.tags,
        }),
    )?;
    println!(
        "    Beitrag {} angelegt: {}\n    Kategorien: {}\n    Tags      : {}",
        r["id"].as_u64().unwrap_or(0),
        r["link"].as_str().unwrap_or("?"),
        v.kat_namen.join(", "),
        v.tag_namen.join(", ")
    );
    Ok(())
}

/// Das Übersichts-PDF neu hochladen und den Link im erklärenden Beitrag
/// nachziehen. Ohne diesen Schritt zeigt der Beitrag nach einem
/// `scanliste`-Lauf eine veraltete Fassung.
fn verzeichnis(z: &Zugang, trocken: bool) -> Result<()> {
    let mut treffer: Vec<PathBuf> = std::fs::read_dir(".")?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(VERZEICHNIS_PRAEFIX) && n.ends_with(".pdf"))
        })
        .collect();
    treffer.sort();
    let pfad = match treffer.len() {
        0 => bail!("kein {VERZEICHNIS_PRAEFIX}….pdf gefunden - erst `scanliste` laufen lassen"),
        1 => treffer.remove(0),
        n => bail!("{n} Verzeichnis-PDF gefunden - alte entfernen"),
    };
    let name = medienname(pfad.file_name().and_then(|n| n.to_str()).unwrap_or_default());
    println!("Verzeichnis: {} → {name}", pfad.display());

    let j = z.hole(&format!(
        "/wp-json/wp/v2/posts?slug={VERZEICHNIS_SLUG}&context=edit&_fields=id,link,content"
    ))?;
    let beitrag = j
        .as_array()
        .and_then(|a| a.first())
        .with_context(|| format!("Beitrag {VERZEICHNIS_SLUG} nicht gefunden"))?;
    let id = beitrag["id"].as_u64().context("Beitrag ohne id")?;
    let roh = beitrag["content"]["raw"].as_str().context("Beitrag ohne Rohinhalt")?;

    let vorige = vorige_adresse(roh);
    match &vorige {
        Some(v) => println!("  bisher verlinkt: {v}"),
        None => println!("  bisher ist im Beitrag kein Verzeichnis verlinkt"),
    }
    if trocken {
        println!("  (würde hochgeladen und der Link in Beitrag {id} nachgezogen)");
        return Ok(());
    }
    let (mid, url) = z.hochladen(&pfad, &name)?;
    println!("  hochgeladen (Medien-ID {mid}): {url}");

    let seiten = pdf_seiten(&pfad).unwrap_or(0);
    let mb = std::fs::metadata(&pfad)?.len() as f64 / 1_048_576.0;
    let neu = format!(
        "<p><strong>Verzeichnis als PDF</strong></p>\n\
         <p><a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a> ({seiten} Seiten, {mb:.1} MB).</p>\n",
        html_escape(&url),
        html_escape(url.rsplit('/').next().unwrap_or(&url))
    );
    let inhalt = match roh.find("<p><strong>Verzeichnis als PDF</strong></p>") {
        Some(i) => format!("{}{neu}", &roh[..i]),
        None => format!("{}\n\n{neu}", roh.trim_end()),
    };
    let r = z.schreibe(&format!("/wp-json/wp/v2/posts/{id}"), &json!({ "content": inhalt }))?;
    println!("  Link nachgezogen: {}", r["link"].as_str().unwrap_or("?"));

    // WordPress überschreibt keine Mediendatei, es legt eine zweite an. Die
    // vorige Fassung ist jetzt unverlinkt und sollte weg, sonst sammeln sich
    // mit jedem `scanliste`-Lauf alte Verzeichnisse in der Mediathek.
    if let Some(alt) = vorige {
        if alt != url {
            println!("  Die vorige Fassung ist jetzt unverlinkt und kann weg:\n    {alt}");
        }
    }
    Ok(())
}

/// Die im Beitrag verlinkte Verzeichnis-Adresse, falls es eine gibt.
fn vorige_adresse(roh: &str) -> Option<String> {
    let ab = roh.find("<p><strong>Verzeichnis als PDF</strong></p>")?;
    let rest = &roh[ab..];
    let start = rest.find("href=\"")? + 6;
    let laenge = rest[start..].find('"')?;
    Some(rest[start..start + laenge].to_string())
}

fn pdf_seiten(pfad: &Path) -> Option<usize> {
    lopdf::Document::load(pfad).ok().map(|d| d.get_pages().len())
}

fn main() -> Result<()> {
    let o = optionen()?;
    let z = Zugang::laden()?;
    let mut taxo = Taxonomie::laden(&z)?;
    println!(
        "Bestand: {} Kategorien, {} Tags brauchbar (von {} insgesamt)",
        taxo.kategorien.len(),
        taxo.tags.len(),
        taxo.tags_alle.len()
    );

    if o.verzeichnis {
        verzeichnis(&z, o.trocken)?;
    }

    let mut offen: Vec<String> = o.scans.clone();
    if o.alle {
        let mut vorhanden: BTreeSet<String> = BTreeSet::new();
        for e in std::fs::read_dir(SCAN_DIR).with_context(|| format!("{SCAN_DIR} fehlt"))? {
            let n = e?.file_name().to_string_lossy().to_string();
            if n.ends_with(".pdf") && medien::url(&n).is_none() {
                vorhanden.insert(n);
            }
        }
        offen.extend(vorhanden);
    }
    offen.dedup();
    if offen.is_empty() && !o.verzeichnis {
        println!("nichts offen - alle Scans stehen in {MEDIEN_RS}");
        return Ok(());
    }
    if !offen.is_empty() {
        println!("{} Scan(s) zu bearbeiten", offen.len());
    }

    // Strikt der Reihe nach in einem Prozess: parallele Läufe haben einmal
    // 429 Dubletten in der Mediathek erzeugt.
    let mut neu: Vec<(String, String)> = Vec::new();
    for datei in &offen {
        match einen_scan(&z, &mut taxo, datei, &o) {
            Ok(Some(paar)) => neu.push(paar),
            Ok(None) => {}
            Err(e) => {
                eprintln!("  FEHLER bei {datei}: {e:#}");
                medien_ergaenzen(&neu)?;
                return Err(e);
            }
        }
    }
    medien_ergaenzen(&neu)?;
    if !neu.is_empty() {
        println!(
            "\nNoch zu tun: `cargo run --release --bin scanliste`, dann\n\
             `cargo run --release --bin webscan -- --verzeichnis`, dann committen."
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medienname_wird_ascii() {
        assert_eq!(
            medienname("Ablösung_von_der_Autorität_28.10.1991.pdf"),
            "Abloesung_von_der_Autoritaet_28.10.1991.pdf"
        );
        assert_eq!(
            medienname("AD(H)S,_Weiterbildung_Schule_am_Wasser_13.06.2024.pdf"),
            "AD_H_S_Weiterbildung_Schule_am_Wasser_13.06.2024.pdf"
        );
    }

    #[test]
    fn normalisieren_setzt_wortgrenzen() {
        let t = normalisieren("Die Familientherapie bei Stieffamilien");
        assert!(t.contains(" familientherapie "));
        // «Familie» steckt in «Stieffamilien», darf aber nicht als Wort treffen
        assert!(!t.contains(" familie "));
    }

    #[test]
    fn adhs_schreibweisen_treffen() {
        let t = normalisieren("AD(H)S und die Arbeitswelt");
        assert!(t.contains(" adhs "));
    }

    #[test]
    fn enthaltene_begriffe_fallen_weg() {
        let b = vec![
            Begriff { id: 1, name: "Familie".into(), norm: normalisieren("Familie") },
            Begriff { id: 2, name: "Familientherapie".into(), norm: normalisieren("Familientherapie") },
        ];
        let t = treffer("Familie und Familientherapie", &b, 8);
        assert_eq!(t.len(), 1);
        assert_eq!(t[0].1, "Familientherapie");
    }

    #[test]
    fn titelwoerter_nur_hauptwoerter() {
        let w = titelwoerter("Aussterbende Familien – welches therapeutische Vorgehen bei der Beratung");
        assert!(w.contains(&"Aussterbende".to_string()));
        assert!(w.contains(&"Familien".to_string()));
        assert!(!w.iter().any(|x| x == "therapeutische")); // klein geschrieben
    }

    #[test]
    fn datum_umrechnen() {
        assert_eq!(datum_iso("28.10.1991").unwrap(), "1991-10-28T09:00:00");
        assert_eq!(datum_iso("12.1991").unwrap(), "1991-12-01T09:00:00");
        assert_eq!(datum_lesbar("28.10.1991").unwrap(), "28. Oktober 1991");
        assert_eq!(datum_lesbar("12.1991").unwrap(), "Dezember 1991");
        assert_eq!(jahr_von("28.10.1991").unwrap(), "1991");
    }

    #[test]
    fn block_ist_fett_und_markiert() {
        let b = pdf_block(&["https://adhs.expert/wp-content/uploads/2026/11/X.pdf".into()]);
        assert!(b.starts_with(MARKE));
        assert!(b.contains("<strong>Unterlagen als PDF</strong>"));
        assert!(!b.contains("<h2"), "nie eine Überschrift - sie wäre zu gross");
        assert!(!b.contains("github"));
    }

    const PROBE: &str = "// Kopf\n\npub const MEDIEN: &[(&str, &str)] = &[\n\
        \x20   (\"Alpha_01.01.1990.pdf\", \"https://adhs.expert/wp-content/uploads/a.pdf\"),\n\
        \x20   (\"Zeta_01.01.1990.pdf\", \"https://adhs.expert/wp-content/uploads/z.pdf\"),\n];\n";

    #[test]
    fn medien_tabelle_sortiert_ein() {
        let neu = vec![(
            "Mitte_02.02.1991.pdf".to_string(),
            "https://adhs.expert/wp-content/uploads/m.pdf".to_string(),
        )];
        let out = medien_tabelle(PROBE, &neu).unwrap();
        assert!(out.starts_with("// Kopf"), "Kopf bleibt stehen");
        let zeilen: Vec<&str> = out.lines().filter(|z| z.trim_start().starts_with('(')).collect();
        assert_eq!(zeilen.len(), 3);
        assert!(zeilen[1].contains("Mitte_02.02.1991.pdf"), "alphabetisch einsortiert: {zeilen:?}");
        assert!(out.ends_with("];\n"));
    }

    #[test]
    fn medien_tabelle_bleibt_gleich_bei_wiederholung() {
        let neu = vec![(
            "Alpha_01.01.1990.pdf".to_string(),
            "https://adhs.expert/wp-content/uploads/a.pdf".to_string(),
        )];
        assert_eq!(medien_tabelle(PROBE, &neu).unwrap(), medien_tabelle(PROBE, &[]).unwrap());
    }

    #[test]
    fn medien_tabelle_weist_fremde_adressen_ab() {
        let neu = vec![(
            "X_01.01.1990.pdf".to_string(),
            "https://github.com/zdavatz/adhs-expert/blob/main/scan/X.pdf".to_string(),
        )];
        assert!(medien_tabelle(PROBE, &neu).is_err(), "GitHub darf nicht hinein");
    }

    #[test]
    fn vorige_adresse_wird_gefunden() {
        let roh = "<p>Text</p>\n<p><strong>Verzeichnis als PDF</strong></p>\n                   <p><a href=\"https://adhs.expert/wp-content/uploads/2026/11/V.pdf\"                    target=\"_blank\">V.pdf</a> (149 Seiten).</p>\n";
        assert_eq!(
            vorige_adresse(roh).as_deref(),
            Some("https://adhs.expert/wp-content/uploads/2026/11/V.pdf")
        );
        assert_eq!(vorige_adresse("<p>noch kein Verzeichnis</p>"), None);
    }

    #[test]
    fn jede_mediathek_adresse_zeigt_auf_adhs_expert() {
        for (datei, url) in medien::MEDIEN {
            assert!(url.starts_with("https://adhs.expert/"), "{datei}: {url}");
        }
    }
}
