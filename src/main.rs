use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

const CHAMPS: [(&str, &str); 6] = [
    ("1", "📋 Cadre 1 — En-tête & Informations Générales"),
    ("2", "🎯 Cadre 2 — Objet & Contexte de l'intervention"),
    ("3", "🔍 Cadre 3 — Diagnostic & Constats (Avant intervention)"),
    ("4", "🛠️ Cadre 4 — Actions Réalisées & Pièces Utilisées"),
    ("5", "🚀 Cadre 5 — Résultat & Clôture de l'intervention"),
    ("6", "✍️ Cadre 6 — Validation & Signatures"),
];

fn sous_champs() -> Vec<Vec<(&'static str, &'static str, bool)>> {
    vec![
        vec![
        ("numero", "Numéro du rapport (Réf. dossier)", false),
        ("date_heure", "Date et heure (ex : 12/05/2025 de 08h00 à 11h30)", false),
        ("intervenants", "Intervenant(s) (Nom, Prénom, Fonction / Société)", false),
        ("client_site", "Client / Site (Entreprise, contact sur place, adresse)", false),
        ("localisation", "Localisation précise (Bâtiment, étage, bureau / équipement)", false),
    ],
        vec![
        ("type_intervention", "Type d'intervention (Préventive / Curative / Installation-Mise en service)", false),
        ("motif", "Motif / Problématique signalée", false),
        ("equipements", "Équipement(s) concerné(s) (Marque, modèle, n° de série)", false),
    ],
        vec![
        ("symptomes", "Symptômes observés (une ligne par symptôme, ligne vide pour terminer)", true),
        ("analyses", "Analyses / Tests effectués (une ligne par test, ligne vide pour terminer)", true),
        ("cause_racine", "Cause racine identifiée", false),
    ],
        vec![
        ("operations", "Détail des opérations (une étape par ligne, ligne vide pour terminer)", true),
        ("pieces", "Matériel / fournitures consommées (ex : Filtre - Réf X123 - Qté 2 ; ligne vide pour terminer)", true),
    ],
        vec![
        ("statut_final", "Statut final (Résolu-Opérationnel / Résolu partiellement / Non résolu-En attente)", false),
        ("tests_validation", "Tests de validation", false),
        ("recommandations", "Recommandations / Actions à prévoir (une ligne par item, ligne vide pour terminer)", true),
    ],
        vec![
        ("commentaires_client", "Commentaires éventuels du client", false),
        ("visa_intervenant", "Visa de l'intervenant (nom)", false),
        ("visa_client", "Visa du client / Réceptionnaire (nom, précédé de « Lu et approuvé »)", false),
        ],
    ]
}

fn lire_ligne(invite: &str) -> String {
    print!("{invite} ");
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("lecture stdin");
    s.trim().to_string()
}

fn lire_liste(invite: &str) -> Vec<String> {
    println!("{invite}");
    let mut v = Vec::new();
    loop {
        let s = lire_ligne("  >");
        if s.is_empty() {
            break;
        }
        v.push(s);
    }
    v
}

fn mode_manuel() -> BTreeMap<String, String> {
    let mut d = BTreeMap::new();
    println!("=== Rédaction guidée du compte rendu d'intervention ===\n");
    for (i, (_, titre)) in CHAMPS.iter().enumerate() {
        println!("\n{titre}");
        for (cle, invite, liste) in &sous_champs()[i] {
            if *liste {
                let items = lire_liste(&format!("{invite} :"));
                d.insert(cle.to_string(), items.join("\n"));
            } else {
                let s = lire_ligne(&format!("{invite} :"));
                d.insert(cle.to_string(), s);
            }
        }
    }
    d
}

fn nettoyer(ligne: &str) -> String {
    ligne
        .trim_start_matches(['-', '•', '*', '·', '\u{25aa}', '>', ' '])
        .trim()
        .to_string()
}

fn classer(ligne: &str) -> Option<&'static str> {
    let l = ligne.to_lowercase();
    let mots_cles: &[(&str, &str)] = &[
        ("numéro de rapport", "numero"), ("réf", "numero"), ("ref.", "numero"),
        ("intervenant", "intervenants"), ("technicien", "intervenants"),
        ("client", "client_site"), ("site", "client_site"), ("adresse", "client_site"),
        ("bâtiment", "localisation"), ("étage", "localisation"), ("localisation", "localisation"),
        ("type d'intervention", "type_intervention"), ("préventive", "type_intervention"),
        ("curative", "type_intervention"), ("mise en service", "type_intervention"),
        ("motif", "motif"), ("problématique", "motif"), ("demande du client", "motif"),
        ("équipement", "equipements"), ("matériel", "equipements"), ("marque", "equipements"),
        ("numéro de série", "equipements"),
        ("symptôme", "symptomes"), ("constat", "symptomes"), ("message d'erreur", "symptomes"),
        ("anomalie", "symptomes"), ("bruit", "symptomes"),
        ("mesure", "analyses"), ("analyse", "analyses"), ("test effectué", "analyses"),
        ("diagnostic", "analyses"),
        ("cause", "cause_racine"), ("origine", "cause_racine"),
        ("étape", "operations"), ("operation", "operations"), ("consignation", "operations"),
        ("démontage", "operations"), ("remplacement", "operations"), ("nettoyage", "operations"),
        ("remontage", "operations"), ("remise en route", "tests_validation"),
        ("pièce", "pieces"), ("réf :", "pieces"),
        ("statut", "statut_final"), ("résolu", "statut_final"), ("opérationnel", "statut_final"),
        ("essai de charge", "tests_validation"), ("test de validation", "tests_validation"),
        ("recommandation", "recommandations"), ("à prévoir", "recommandations"),
        ("prévoir", "recommandations"), ("former", "recommandations"),
        ("commentaire", "commentaires_client"),
        ("visa", "visa_intervenant"), ("signature", "visa_client"), ("lu et approuvé", "visa_client"),
    ];
    let mut meilleur: Option<(&str, usize)> = None;
    for (mot, cle) in mots_cles {
        if let Some(pos) = l.find(mot) {
            if meilleur.map_or(true, |(_, best)| pos < best) {
                meilleur = Some((cle, pos));
            }
        }
    }
    meilleur.map(|(cle, _)| cle)
}


fn retirer_etiquette(ligne: &str) -> String {
    let etiquettes = [
        "numéro de rapport", "intervenant", "technicien", "client", "site", "adresse",
        "localisation", "bâtiment", "type d'intervention", "motif", "problématique",
        "équipement", "matériel", "symptôme", "constat", "mesure", "analyse",
        "cause racine", "cause", "origine", "pièce", "statut", "test de validation",
        "test", "recommandation", "commentaire client", "commentaire",
        "visa intervenant", "visa client", "visa",
    ];
    if let Some(pos) = ligne.find(':') {
        let prefixe = ligne[..pos].to_lowercase();
        if prefixe.len() < 40 && etiquettes.iter().any(|e| prefixe.contains(e)) {
            return ligne[pos + 1..].to_string();
        }
    }
    ligne.to_string()
}

fn mode_brouillon(chemin: &str) -> BTreeMap<String, String> {
    let texte = fs::read_to_string(chemin)
        .unwrap_or_else(|e| { eprintln!("Impossible de lire « {chemin} » : {e}"); process::exit(1); });
    let mut d: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut courant = "motif";
    for brut in texte.lines() {
        let ligne = brut.trim();
        if ligne.is_empty() {
            continue;
        }
        if let Some(cle) = classer(ligne) {
            courant = cle;
        }
        let propre = nettoyer(&retirer_etiquette(ligne));
        d.entry(courant.to_string()).or_default().push(propre);
    }
    d.into_iter().map(|(k, v)| (k, v.join("\n"))).collect()
}

fn cocher(valeur: &str, option: &str) -> &'static str {
    if valeur.to_lowercase().contains(&option.to_lowercase()) {
        "☒"
    } else {
        "🔲"
    }
}

fn bloc(d: &BTreeMap<String, String>, cle: &str) -> String {
    d.get(cle).map(|s| s.trim()).unwrap_or("").to_string()
}

fn liste_md(valeur: &str) -> String {
    if valeur.is_empty() {
        return "  - (à compléter)\n".to_string();
    }
    valeur
        .lines()
        .map(|l| format!("  - {}\n", l.trim()))
        .collect()
}

fn generer(d: &BTreeMap<String, String>) -> String {
    let type_int = bloc(d, "type_intervention");
    let statut = bloc(d, "statut_final");
    format!(
        "# COMPTE RENDU D'INTERVENTION\n\n\
## 📋 Cadre 1 — En-tête & Informations Générales\n\n\
- **Numéro du rapport :** {}\n\
- **Date et heure :** {}\n\
- **Intervenant(s) :** {}\n\
- **Client / Site :** {}\n\
- **Localisation précise :** {}\n\n\
## 🎯 Cadre 2 — Objet & Contexte de l'intervention\n\n\
- **Type d'intervention :** {} Préventive | {} Curative (Dépannage) | {} Installation / Mise en service\n\
- **Motif / Problématique signalée :** {}\n\
- **Équipement(s) concerné(s) :** {}\n\n\
## 🔍 Cadre 3 — Diagnostic & Constats (Avant intervention)\n\n\
- **Symptômes observés :**\n{}\
- **Analyses / Tests effectués :**\n{}\
- **Cause racine :** {}\n\n\
## 🛠️ Cadre 4 — Actions Réalisées & Pièces Utilisées\n\n\
- **Détail des opérations :**\n{}\
- **Matériel et fournitures consommées :**\n{}\
\n## 🚀 Cadre 5 — Résultat & Clôture de l'intervention\n\n\
- **Statut final :** {} Résolu / Opérationnel | {} Résolu partiellement | {} Non résolu (En attente)\n\
- **Tests de validation :** {}\n\
- **Recommandations / Actions à prévoir :**\n{}\
\n## ✍️ Cadre 6 — Validation & Signatures\n\n\
- **Commentaires éventuels du client :** {}\n\
- **Visa de l'intervenant :** {}\n\n\
- **Visa du client / Réceptionnaire :** {} — « Lu et approuvé »\n\n\
---\n*Document généré le {}*\n",
        bloc(d, "numero"),
        bloc(d, "date_heure"),
        bloc(d, "intervenants"),
        bloc(d, "client_site"),
        bloc(d, "localisation"),
        cocher(&type_int, "préventive"),
        cocher(&type_int, "curative"),
        cocher(&type_int, "mise en service"),
        bloc(d, "motif"),
        bloc(d, "equipements"),
        liste_md(&bloc(d, "symptomes")),
        liste_md(&bloc(d, "analyses")),
        bloc(d, "cause_racine"),
        liste_md(&bloc(d, "operations")),
        liste_md(&bloc(d, "pieces")),
        cocher(&statut, "résolu / opérationnel"),
        cocher(&statut, "partiellement"),
        cocher(&statut, "non résolu"),
        bloc(d, "tests_validation"),
        liste_md(&bloc(d, "recommandations")),
        bloc(d, "commentaires_client"),
        bloc(d, "visa_intervenant"),
        bloc(d, "visa_client"),
        chrono_now(),
    )
}

#[cfg(unix)]
fn chrono_now() -> String {
    std::process::Command::new("date")
        .arg("+%d/%m/%Y %H:%M")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

#[cfg(not(unix))]
fn chrono_now() -> String {
    String::new()
}


fn lignes(valeur: &str) -> Vec<String> {
    if valeur.trim().is_empty() {
        vec![]
    } else {
        valeur.lines().map(|l| l.trim().to_string()).collect()
    }
}

fn puces(valeur: &str, puce: &str) -> String {
    let items = lignes(valeur);
    if items.is_empty() {
        format!("{puce} (à compléter)\n")
    } else {
        items.iter().map(|l| format!("{puce} {l}\n")).collect()
    }
}

fn generer_courriel(d: &BTreeMap<String, String>) -> String {
    let numero = bloc(d, "numero");
    let client = bloc(d, "client_site");
    let date = bloc(d, "date_heure");
    let objet = format!("Objet : [{}] Compte rendu d'intervention — {}", numero, client);
    format!(
        "{objet}\n\n\
Bonjour,\n\n\
Veuillez trouver ci-dessous le compte rendu de l'intervention réalisée {}.\n\n\
— — — — — — — — — —\n\
RAPPORT N° {}\n— — — — — — — — — —\n\n\
1) CONTEXTE\n\
Intervenant(s) : {}\n\
Client / Site : {}\n\
Localisation : {}\n\
Type d'intervention : {}\n\
Motif : {}\n\
Équipement(s) : {}\n\n\
2) DIAGNOSTIC\n\
Symptômes observés :\n{}\
Analyses / Tests :\n{}\
Cause racine : {}\n\n\
3) ACTIONS RÉALISÉES\n{}\
Pièces / fournitures :\n{}\
\n4) RÉSULTAT\n\
Statut final : {}\n\
Tests de validation : {}\n\
Recommandations :\n{}\
\n5) VALIDATION\n\
Commentaires du client : {}\n\n\
Bien cordialement,\n\
{} — Intervenant\n\
Visa client : {} — « Lu et approuvé »\n",
        date,
        numero,
        bloc(d, "intervenants"),
        client,
        bloc(d, "localisation"),
        bloc(d, "type_intervention"),
        bloc(d, "motif"),
        bloc(d, "equipements"),
        puces(&bloc(d, "symptomes"), " -"),
        puces(&bloc(d, "analyses"), " -"),
        bloc(d, "cause_racine"),
        puces(&bloc(d, "operations"), " -"),
        puces(&bloc(d, "pieces"), " -"),
        bloc(d, "statut_final"),
        bloc(d, "tests_validation"),
        puces(&bloc(d, "recommandations"), " -"),
        bloc(d, "commentaires_client"),
        bloc(d, "visa_intervenant"),
        bloc(d, "visa_client"),
    )
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn td(cle: &str, etiquette: &str, d: &BTreeMap<String, String>) -> String {
    format!("<tr><th>{}</th><td>{}</td></tr>\n", etiquette, esc(&bloc(d, cle)))
}

fn td_liste(cle: &str, etiquette: &str, d: &BTreeMap<String, String>) -> String {
    let items = lignes(&bloc(d, cle));
    if items.is_empty() {
        return format!("<tr><th>{}</th><td><ul><li>À compléter</li></ul></td></tr>\n", etiquette);
    }
    let lis: String = items.iter().map(|l| format!("<li>{}</li>", esc(l))).collect();
    format!("<tr><th>{}</th><td><ul>{}</ul></td></tr>\n", etiquette, lis)
}

fn case_cochee(valeur: &str, option: &str) -> &'static str {
    if cocher(valeur, option) == "☒" { "checked" } else { "" }
}

fn generer_corporate(d: &BTreeMap<String, String>) -> String {
    let type_int = bloc(d, "type_intervention");
    let statut = bloc(d, "statut_final");
    format!(
        r#"<!DOCTYPE html>
<html lang="fr"><head><meta charset="utf-8">
<title>Compte rendu d'intervention {numero}</title>
<style>
 body {{ font-family: 'Segoe UI', Arial, sans-serif; color: #1f2a37; margin: 40px auto; max-width: 900px; }}
 h1 {{ font-size: 22px; letter-spacing: 1px; color: #0f2b46; border-bottom: 3px solid #0f2b46; padding-bottom: 8px; text-transform: uppercase; }}
 h2 {{ font-size: 15px; background: #0f2b46; color: #fff; padding: 6px 10px; margin-top: 28px; text-transform: uppercase; letter-spacing: .5px; }}
 table {{ width: 100%; border-collapse: collapse; margin-top: 6px; }}
 th {{ text-align: left; width: 32%; background: #eef2f6; border: 1px solid #c8d2dc; padding: 6px 8px; font-size: 13px; }}
 td {{ border: 1px solid #c8d2dc; padding: 6px 8px; font-size: 13px; }}
 ul {{ margin: 4px 0; padding-left: 20px; }}
 .checkboxes span {{ margin-right: 18px; }}
 .signatures {{ display: flex; gap: 24px; margin-top: 24px; }}
 .sign-box {{ flex: 1; border: 1px solid #0f2b46; min-height: 90px; padding: 8px; font-size: 13px; }}
 .mention {{ font-size: 11px; color: #5b6b7c; margin-top: 4px; }}
 footer {{ margin-top: 36px; font-size: 11px; color: #5b6b7c; border-top: 1px solid #c8d2dc; padding-top: 8px; }}
</style></head><body>
<h1>Compte rendu d'intervention</h1>
<p style="font-size:13px">Réf. : <b>{numero}</b> — {date}</p>

<h2>📋 Cadre 1 — En-tête &amp; Informations Générales</h2>
<table>
{c1a}{c1b}{c1c}{c1d}{c1e}
</table>

<h2>🎯 Cadre 2 — Objet &amp; Contexte de l'intervention</h2>
<table>
<tr><th>Type d'intervention</th><td class="checkboxes">
 <input type="checkbox" disabled {t_p}> Préventive
 <input type="checkbox" disabled {t_c}> Curative (Dépannage)
 <input type="checkbox" disabled {t_i}> Installation / Mise en service</td></tr>
{c2b}{c2c}
</table>

<h2>🔍 Cadre 3 — Diagnostic &amp; Constats (Avant intervention)</h2>
<table>
{c3a}{c3b}{c3c}
</table>

<h2>🛠️ Cadre 4 — Actions Réalisées &amp; Pièces Utilisées</h2>
<table>
{c4a}{c4b}
</table>

<h2>🚀 Cadre 5 — Résultat &amp; Clôture de l'intervention</h2>
<table>
<tr><th>Statut final</th><td class="checkboxes">
 <input type="checkbox" disabled {s_r}> Résolu / Opérationnel
 <input type="checkbox" disabled {s_p}> Résolu partiellement
 <input type="checkbox" disabled {s_n}> Non résolu (En attente)</td></tr>
{c5b}{c5c}
</table>

<h2>✍️ Cadre 6 — Validation &amp; Signatures</h2>
<table>
{c6a}
</table>
<div class="signatures">
 <div class="sign-box">Visa de l'intervenant<br><br><br><b>{visa_int}</b></div>
 <div class="sign-box">Visa du client / Réceptionnaire<br><br><br><b>{visa_cli}</b><div class="mention">Mention « Lu et approuvé »</div></div>
</div>

<footer>Document généré le {now} — compte-rendu v0.1</footer>
</body></html>
"#,
        numero = esc(&bloc(d, "numero")),
        date = esc(&bloc(d, "date_heure")),
        c1a = td("intervenants", "Intervenant(s)", d),
        c1b = td("client_site", "Client / Site", d),
        c1c = td("localisation", "Localisation précise", d),
        c1d = td("type_intervention", "Type d'intervention (texte)", d),
        c1e = td("motif", "Motif / Problématique signalée", d),
        t_p = case_cochee(&type_int, "préventive"),
        t_c = case_cochee(&type_int, "curative"),
        t_i = case_cochee(&type_int, "mise en service"),
        c2b = td("equipements", "Équipement(s) concerné(s)", d),
        c2c = td("motif", "Motif (détail)", d),
        c3a = td_liste("symptomes", "Symptômes observés", d),
        c3b = td_liste("analyses", "Analyses / Tests effectués", d),
        c3c = td("cause_racine", "Cause racine", d),
        c4a = td_liste("operations", "Détail des opérations", d),
        c4b = td_liste("pieces", "Matériel et fournitures consommées", d),
        s_r = case_cochee(&statut, "résolu / opérationnel"),
        s_p = case_cochee(&statut, "partiellement"),
        s_n = case_cochee(&statut, "non résolu"),
        c5b = td("tests_validation", "Tests de validation", d),
        c5c = td_liste("recommandations", "Recommandations / Actions à prévoir", d),
        c6a = td("commentaires_client", "Commentaires éventuels du client", d),
        visa_int = esc(&bloc(d, "visa_intervenant")),
        visa_cli = esc(&bloc(d, "visa_client")),
        now = chrono_now(),
    )
}

fn usage() {
    eprintln!(
        "Compte rendu d'intervention — usage :\n\
  compte-rendu nouveau                 Rédaction guidée champ par champ\n\
  compte-rendu restructurer <fichier>  Restructure un brouillon texte dans le cadre imposé\n\
  compte-rendu cadre                   Affiche le cadre de référence\n\
Options :
\
  -o <fichier>    écrit le rapport dans un fichier
\
  -f <format>     format de sortie : markdown (défaut), courriel, corporate (HTML imprimable)"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reste: Vec<String> = args[1..].to_vec();
    let mut sortie: Option<String> = None;
    let mut format = String::from("markdown");
    if let Some(pos) = reste.iter().position(|a| a == "-o") {
        if pos + 1 < reste.len() {
            sortie = Some(reste.remove(pos + 1));
            reste.remove(pos);
        }
    }
    if let Some(pos) = reste.iter().position(|a| a == "-f" || a == "--format") {
        if pos + 1 < reste.len() {
            format = reste.remove(pos + 1).to_lowercase();
            reste.remove(pos);
        }
    }
    let donnees = match reste.as_slice() {
        [cmd] if cmd == "nouveau" => mode_manuel(),
        [cmd, fichier] if cmd == "restructurer" => mode_brouillon(fichier),
        [cmd] if cmd == "cadre" => {
            for (_, t) in CHAMPS {
                println!("{t}");
            }
            return;
        }
        _ => {
            usage();
            process::exit(1);
        }
    };
    let rapport = match format.as_str() {
        "markdown" | "md" => generer(&donnees),
        "courriel" | "email" | "mail" => generer_courriel(&donnees),
        "corporate" | "html" => generer_corporate(&donnees),
        autre => {
            eprintln!("Format inconnu : {autre} (formats : markdown, courriel, corporate)");
            process::exit(1);
        }
    };
    match sortie {
        Some(f) => {
            fs::write(&f, &rapport).expect("écriture du fichier de sortie");
            println!("Rapport écrit dans « {f} » (format {format})");
        }
        None => print!("{rapport}"),
    }
}
