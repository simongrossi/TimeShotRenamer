# ⏱️ TimeShotRenamer (en développement Noob)

&#x20;&#x20;

**TimeShotRenamer** est un outil graphique pour **Windows**, développé en **Rust** avec `eframe/egui`, permettant de **renommer intelligemment des photos** en se basant sur leur **date EXIF** (DateTimeOriginal, etc.).

> Son objectif : te faire gagner du temps lors du tri de photos, avec une interface claire et un renommage sûr et personnalisable.

---

## 🌟 Objectifs

- 📂 Scanner un dossier de photos

- 📸 Lire la date EXIF (DateTimeOriginal, etc.)

- 🔍 Détecter si la date figure déjà dans le nom du fichier (même avec des séparateurs différents)

- 🧠 Comparer la date EXIF et celle du nom (match ou non)

- ✏️ Proposer un nouveau nom au format :

  ```
  YYYY-MM-DD_HHMMSS_nomoriginal.extension
  ```

- ✅ Permettre le renommage des fichiers sélectionnés

---

## 🧰 Fonctionnalités actuelles

- Interface graphique rapide et responsive via `egui`
- Sélecteur de dossier natif (`rfd`)
- Tableau dynamique avec :
  - ✅ Case à cocher par fichier
  - 📄 Nom actuel
  - 📷 Présence et lecture de la date EXIF
  - 🔎 Détection flexible de la date dans le nom (avec séparateurs variés)
  - 🔁 Comparaison date EXIF vs date dans nom
  - ✨ Aperçu du nouveau nom de fichier proposé
  - 🛠️ **Colonne debug masquée** (affiche tous les formats testés)
- ✅ Bouton pour sélectionner tous les fichiers avec EXIF
- 🔒 Mode simulation (dry-run) sans renommage réel
- 🧹 Mode tableau à rayures pour lisibilité

---

## 🚀 Lancer l’application

```bash
cargo run --release
```

⚠️ Conçu pour Windows. La compilation sous Linux/Mac n’a pas été testée.

---

## 🔧 Dépendances principales

| Crate             | Rôle                              |
| ----------------- | --------------------------------- |
| `eframe` + `egui` | Interface graphique               |
| `kamadak-exif`    | Lecture des métadonnées EXIF      |
| `walkdir`         | Parcours récursif des dossiers    |
| `chrono`          | Manipulation de dates             |
| `rfd`             | Fenêtres de sélection de dossiers |

---

## 🛠️ Compilation Windows

Le `Cargo.toml` est configuré pour éviter l’ouverture d’un terminal noir :

```toml
[[bin]]
name = "TimeShotRenamer"
path = "src/main.rs"
windows_subsystem = "windows"
```

---

## 📆 Release et binaire

Tu peux compiler un exécutable propre avec :

```bash
cargo build --release
```

Le binaire sera dans `target/release/TimeShotRenamer.exe`.

📝 Tu peux ensuite créer une release GitHub avec ce `.exe` pour le partager.

---

## 📌 Roadmap / TODO

- ✅ Détection de formats de date variés dans les noms
- ✅ Comparaison date EXIF vs date du nom
- ✅ Interface clean avec options avancées (colonne debug)
- 🔄 Menu pour insérer d’autres champs EXIF (appareil, lentille, etc.)
- ⏳ Barre de progression pendant l’analyse
- 🧪 Aperçu en direct du nouveau nom (avec insertion dynamique EXIF)
- 🔍 Recherche ou filtre par nom/date

---

## 👨‍💼 Auteur

Développé avec ❤️ par [Simon Grossi](https://github.com/simongrossi)\
Avec un coup de main des différentes IA et beaucoup de plaisir pour apprendre 🧰

---

## 🪪 Licence

Ce projet est distribué sous licence **MIT**.\
Feel free to fork, améliorer ou contribuer !

