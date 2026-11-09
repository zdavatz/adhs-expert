// Verschickt eine Mail mit Anhängen über die Gmail-API.
//
//   cargo run --release --bin mail -- --an adresse@example.com \
//       --betreff "Betreff" --nachricht "Text" --anhang scan/x.pdf \
//       --token /pfad/zu/token.json
//
// Der Token ist eine Google-OAuth-Datei im Format «authorized user»
// (client_id, client_secret, refresh_token, token_uri), wie sie
// google-auth-oauthlib schreibt; Bereich gmail.send. Statt --token geht
// die Umgebungsvariable ADHS_MAIL_TOKEN. Weder Pfad noch Empfänger
// gehören ins Repository - es ist öffentlich.
//
// Mit --trocken wird der Zugang erneuert und die Nachricht gebaut, aber
// nicht gesendet; die Grösse der Rohnachricht wird ausgegeben.
//
// Lizenz: GPL-3.0

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;

const SENDEN_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Default)]
struct Optionen {
    an: Vec<String>,
    betreff: String,
    text: String,
    anhaenge: Vec<PathBuf>,
    token: Option<PathBuf>,
    trocken: bool,
}

fn optionen() -> Result<Optionen> {
    let mut o = Optionen::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut wert = || args.next().with_context(|| format!("{a} braucht einen Wert"));
        match a.as_str() {
            "--an" => o.an.push(wert()?),
            "--betreff" => o.betreff = wert()?,
            "--nachricht" => o.text = wert()?,
            "--text" => {
                let p = wert()?;
                o.text = std::fs::read_to_string(&p).with_context(|| p.clone())?;
            }
            "--anhang" => o.anhaenge.push(PathBuf::from(wert()?)),
            "--token" => o.token = Some(PathBuf::from(wert()?)),
            "--trocken" => o.trocken = true,
            "-h" | "--help" => {
                println!(
                    "mail --an ADRESSE [--an …] --betreff TEXT (--nachricht TEXT | --text DATEI) \
                     [--anhang DATEI …] [--token DATEI | $ADHS_MAIL_TOKEN] [--trocken]"
                );
                std::process::exit(0);
            }
            _ => bail!("unbekanntes Argument {a:?} (--help)"),
        }
    }
    if o.an.is_empty() {
        bail!("--an fehlt");
    }
    if o.betreff.is_empty() {
        bail!("--betreff fehlt");
    }
    if o.token.is_none() {
        o.token = std::env::var_os("ADHS_MAIL_TOKEN").map(PathBuf::from);
    }
    Ok(o)
}

/// Holt mit dem Refresh-Token ein frisches Zugriffstoken.
fn zugriff(token: &Path) -> Result<String> {
    let roh = std::fs::read_to_string(token).with_context(|| format!("{}", token.display()))?;
    let t: serde_json::Value = serde_json::from_str(&roh).context("Token ist kein JSON")?;
    let feld = |k: &str| t[k].as_str().with_context(|| format!("Token ohne {k}"));
    let url = t["token_uri"].as_str().unwrap_or(TOKEN_URL);
    let antwort = ureq::post(url).send_form(&[
        ("client_id", feld("client_id")?),
        ("client_secret", feld("client_secret")?),
        ("refresh_token", feld("refresh_token")?),
        ("grant_type", "refresh_token"),
    ]);
    let antwort = match antwort {
        Ok(r) => r,
        Err(ureq::Error::Status(code, r)) => {
            let text = r.into_string().unwrap_or_default();
            if text.contains("invalid_grant") {
                bail!("Token abgelaufen oder widerrufen (invalid_grant) - neu autorisieren");
            }
            bail!("Token-Erneuerung: HTTP {code}: {text}");
        }
        Err(e) => return Err(e).context("Token-Erneuerung"),
    };
    let j: serde_json::Value = antwort.into_json()?;
    Ok(j["access_token"].as_str().context("keine access_token in der Antwort")?.to_string())
}

/// Kopfzeile nach RFC 2047, wenn sie Nicht-ASCII enthält.
fn kopf(s: &str) -> String {
    if s.is_ascii() {
        s.to_string()
    } else {
        format!("=?UTF-8?B?{}?=", STANDARD.encode(s))
    }
}

/// Dateiname nach RFC 2231 (`filename*=UTF-8''…`), dazu `filename=` als
/// Rückfall für ältere Leser.
fn dateiname_kopf(name: &str) -> String {
    let mut kodiert = String::new();
    for b in name.bytes() {
        if b.is_ascii_alphanumeric() || b"-._".contains(&b) {
            kodiert.push(b as char);
        } else {
            kodiert.push_str(&format!("%{b:02X}"));
        }
    }
    format!("filename=\"{}\"; filename*=UTF-8''{kodiert}", kopf(name))
}

/// Base64 in Zeilen zu 76 Zeichen, wie MIME es verlangt.
fn b64_zeilen(daten: &[u8]) -> String {
    let s = STANDARD.encode(daten);
    let mut aus = String::with_capacity(s.len() + s.len() / 76 * 2);
    for teil in s.as_bytes().chunks(76) {
        aus.push_str(std::str::from_utf8(teil).unwrap());
        aus.push_str("\r\n");
    }
    aus
}

fn mime_typ(p: &Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("pdf") => "application/pdf",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("txt") => "text/plain; charset=UTF-8",
        _ => "application/octet-stream",
    }
}

fn nachricht(o: &Optionen) -> Result<Vec<u8>> {
    let grenze = format!("adhs-expert-{:x}", std::process::id() as u64 ^ 0x5eed_cafe);
    let mut m = String::new();
    m.push_str(&format!("To: {}\r\n", o.an.join(", ")));
    m.push_str(&format!("Subject: {}\r\n", kopf(&o.betreff)));
    m.push_str("MIME-Version: 1.0\r\n");
    m.push_str(&format!("Content-Type: multipart/mixed; boundary=\"{grenze}\"\r\n\r\n"));
    m.push_str(&format!("--{grenze}\r\n"));
    m.push_str("Content-Type: text/plain; charset=UTF-8\r\nContent-Transfer-Encoding: base64\r\n\r\n");
    m.push_str(&b64_zeilen(o.text.replace("\r\n", "\n").replace('\n', "\r\n").as_bytes()));
    for p in &o.anhaenge {
        let daten = std::fs::read(p).with_context(|| format!("{}", p.display()))?;
        let name = p.file_name().and_then(|n| n.to_str()).context("Dateiname")?;
        m.push_str(&format!("--{grenze}\r\n"));
        m.push_str(&format!("Content-Type: {}\r\n", mime_typ(p)));
        m.push_str(&format!("Content-Disposition: attachment; {}\r\n", dateiname_kopf(name)));
        m.push_str("Content-Transfer-Encoding: base64\r\n\r\n");
        m.push_str(&b64_zeilen(&daten));
    }
    m.push_str(&format!("--{grenze}--\r\n"));
    Ok(m.into_bytes())
}

fn main() -> Result<()> {
    let o = optionen()?;
    let roh = nachricht(&o)?;
    let token = o.token.as_deref().context("--token oder ADHS_MAIL_TOKEN fehlt")?;
    let zugang = zugriff(token)?;
    if o.trocken {
        println!("trocken: Zugang erneuert, Nachricht {} B, nicht gesendet", roh.len());
        return Ok(());
    }
    let antwort = ureq::post(SENDEN_URL)
        .set("Authorization", &format!("Bearer {zugang}"))
        .send_json(serde_json::json!({ "raw": URL_SAFE_NO_PAD.encode(&roh) }));
    let antwort = match antwort {
        Ok(r) => r,
        Err(ureq::Error::Status(code, r)) => {
            bail!("Senden: HTTP {code}: {}", r.into_string().unwrap_or_default())
        }
        Err(e) => return Err(e).context("Senden"),
    };
    let j: serde_json::Value = antwort.into_json()?;
    println!("gesendet, ID {}, {} B", j["id"].as_str().unwrap_or("?"), roh.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kopf_kodiert_umlaute() {
        assert_eq!(kopf("Vortrag"), "Vortrag");
        assert!(kopf("Vorträge").starts_with("=?UTF-8?B?"));
    }

    #[test]
    fn dateiname_prozentkodiert() {
        let k = dateiname_kopf("Süchte_1988.pdf");
        assert!(k.contains("filename*=UTF-8''S%C3%BCchte_1988.pdf"), "{k}");
    }

    #[test]
    fn nachricht_hat_anhang() {
        let o = Optionen {
            an: vec!["a@example.com".into()],
            betreff: "Grüsse".into(),
            text: "Hallo\nWelt".into(),
            anhaenge: vec![PathBuf::from("Cargo.toml")],
            ..Default::default()
        };
        let m = String::from_utf8(nachricht(&o).unwrap()).unwrap();
        assert!(m.contains("Content-Disposition: attachment"));
        assert!(m.lines().all(|l| l.len() <= 998));
    }
}
