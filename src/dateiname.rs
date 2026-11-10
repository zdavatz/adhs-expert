// Dateinamen der Scans: `Titel_TT.MM.JJJJ.pdf`, `Titel_MM.JJJJ.pdf`, mit
// optionaler Fassung `_2`. Gemeinsam genutzt von `scanliste` und `eintragen`.
//
// Lizenz: GPL-3.0

/// Zerlegt `Titel_TT.MM.JJJJ[_N].pdf` oder `Titel_MM.JJJJ[_N].pdf`.
pub fn zerlegen(datei: &str) -> Option<(String, String, (u32, u32, u32), Option<u32>)> {
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
