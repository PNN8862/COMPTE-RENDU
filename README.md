# Compte rendu d'intervention (Rust)

Programme en ligne de commande pour rédiger des comptes rendus d'intervention de niveau cadre,
conformes à un cadre en 6 blocs, avec deux modes de saisie et trois formats de sortie.

## Compilation

```bash
cargo build --release
# binaire : target/release/compte-rendu
```

Aucune dépendance externe (bibliothèque standard uniquement).

## Utilisation

```bash
compte-rendu nouveau                  # rédaction guidée champ par champ
compte-rendu restructurer <fichier>   # restructure un brouillon texte dans le cadre
compte-rendu cadre                    # affiche le cadre de référence
```

### Options

- `-o <fichier>` : écrit le rapport dans un fichier
- `-f <format>` : format de sortie
  - `markdown` (défaut) : rapport structuré selon les 6 cadres, cases 🔲/☒ automatiques
  - `courriel` : objet + corps prêts à coller dans un e-mail
  - `corporate` : rapport HTML formel, imprimable en PDF

### Exemples

```bash
./target/release/compte-rendu restructurer brouillon.txt -f corporate -o rapport.html
./target/release/compte-rendu nouveau -f courriel -o mail.txt
```

## Le cadre en 6 blocs

1. 📋 En-tête & Informations Générales (réf., date/heure, intervenants, client, localisation)
2. 🎯 Objet & Contexte (type d'intervention, motif, équipements)
3. 🔍 Diagnostic & Constats (symptômes, analyses/tests, cause racine)
4. 🛠️ Actions Réalisées & Pièces Utilisées (étapes, fournitures)
5. 🚀 Résultat & Clôture (statut final, tests de validation, recommandations)
6. ✍️ Validation & Signatures (commentaires client, visas)

## Numérotation automatique

Si le numéro de rapport est laissé vide (mode `nouveau`) ou absent du brouillon (mode `restructurer`),
un numéro unique est attribué automatiquement au format `CR-<année>-<séquence>` (ex : `CR-2025-0001`).
Le compteur est persistant : il est mémorisé dans `~/.compte-rendu.seq` et s'incrémente à chaque rapport.

## Restructuration d'un brouillon

Chaque ligne du brouillon est classée automatiquement dans le bon champ par mots-clés
(symptôme, cause, étape, pièce, statut, recommandation…), les étiquettes redondantes
sont supprimées, les lignes répétées deviennent des listes à puces, et les cases
(type d'intervention, statut final) sont cochées automatiquement.
