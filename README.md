# ⏱️ TimeShotRenamer

![Rust](https://img.shields.io/badge/Rust-2021-orange)
![Windows](https://img.shields.io/badge/platform-Windows-blue)
![License](https://img.shields.io/badge/license-MIT-green)

**TimeShotRenamer** est un outil graphique Windows développé en **Rust** avec **egui/eframe**, conçu pour faciliter le **renommage intelligent de photos** selon leurs **métadonnées EXIF** (notamment la date de prise de vue).

---

## 🎯 Objectif

- 📂 Analyser un dossier contenant des photos
- 🕒 Lire la date EXIF (DateTimeOriginal)
- 🔍 Vérifier si cette date est déjà présente dans le nom du fichier
- ✏️ Proposer un **nouveau nom** avec la date intégrée :
  
  ```
  YYYY-MM-DD_HHMMSS_nomoriginal.extension
  ```
- ✅ Renommer les fichiers sélectionnés de manière sécurisée

---

## 🧰 Fonctionnalités actuelles

- Interface graphique simple et rapide avec `egui`
- Sélection d’un dossier via une boîte de dialogue native
- Tableau interactif :
  - ✅ Case à cocher par fichier
  - 📛 Nom du fichier original
  - 📷 Présence EXIF avec date (✅ ou ❌)
  - 🔍 Vérification si la date figure déjà dans le nom
  - ✏️ Prévisualisation du nouveau nom proposé
- Sélection rapide des fichiers avec EXIF
- Mode rayé (striped) pour une meilleure lisibilité du tableau

---

## 🚀 Lancer l’application

```bash
cargo run --release
```

⚠️ L'application est conçue pour Windows.

---

## 🔧 Dépendances principales

- [`eframe`](https://docs.rs/eframe) + [`egui`](https://docs.rs/egui) – Interface graphique
- [`kamadak-exif`](https://crates.io/crates/kamadak-exif) – Lecture EXIF
- [`walkdir`](https://crates.io/crates/walkdir) – Parcours récursif des dossiers
- [`serde` / `serde_json`](https://serde.rs) – Sauvegarde temporaire des métadonnées (à venir)
- [`rfd`](https://crates.io/crates/rfd) – Sélecteur de fichiers natif

---

## 🛠️ Compilation Windows

Le fichier `Cargo.toml` est configuré pour créer un binaire propre, sans fenêtre console :

```toml
[[bin]]
name = "TimeShotRenamer"
path = "src/main.rs"
windows_subsystem = "windows"
```

---

## 📌 Prochaines évolutions

- 🔄 Renommage effectif des fichiers sélectionnés
- 🧠 Ajout d’un menu déroulant pour insérer d'autres champs EXIF dans le nom
- 🔁 Barre de chargement ou spinner pendant le scan
- 🔍 Filtre ou recherche dans le tableau
- ❗ Affichage d’erreurs dans l’UI

---

## 👨‍💻 Auteur

Développé avec ❤️ et surtout curiosité par [Simon Grossi](https://github.com/simongrossi) avec l’aide de différentes IA (Open AI, Gemini, Mistral).
