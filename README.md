# 🕒 TimeShotRenamer

**TimeShotRenamer** est un outil graphique Windows écrit en Rust permettant de **renommer automatiquement des photos** selon leur **date EXIF** (date de prise de vue).

![screenshot](docs/screenshot.png)

---

## ✨ Fonctionnalités

- 📂 Parcours d’un dossier contenant des photos
- 📸 Lecture automatique des **données EXIF** (DateTimeOriginal)
- 🔎 Indique si la date est déjà présente dans le nom du fichier
- 🔄 **Renommage automatique** au format :

  ```
  YYYY-MM-DD_HHMMSS_nomoriginal.extension
  ```
  Exemples :
  - `IMG_4431.JPG` → `2024-10-29_105953_IMG_4431.JPG`

- ❌ Fichiers sans EXIF non modifiés
- 🖥 Interface simple et rapide grâce à `egui`

---

## 🚀 Installation

### 🧱 Pré-requis
- [Rust](https://www.rust-lang.org/tools/install)
- Windows 10/11 recommandé

### 🧪 Compilation

```bash
cargo build --release
```

L’exécutable sera disponible dans :

```
target/release/TimeShotRenamer.exe
```

### ❌ Éviter la fenêtre noire en mode GUI

Ajoutez ceci à la fin du `Cargo.toml` :

```toml
[[bin]]
name = "TimeShotRenamer"
path = "src/main.rs"
windows_subsystem = "windows"
```

---

## 💡 À venir (Roadmap)

- ✅ Prévisualisation des nouveaux noms
- ✅ Détection de date dans le nom du fichier
- ⏳ Barre de progression lors du renommage
- 🧩 Choix du format de date et du nom final
- 📦 Export CSV ou JSON des noms avant/après
- 📂 Support du glisser-déposer
- 🌍 Version multi-plateforme (Linux/macOS)

---

## 👨‍💻 Développé par

Simon Grossi  ·  [GitHub](https://github.com/simongrossi)

---

## 📄 Licence

MIT – Utilisation libre et open-source.
