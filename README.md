# 📸 TimeShotRenamer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**TimeShotRenamer** est un outil développé en Rust avec une interface graphique GTK4 pour analyser, proposer des renommages intelligents et organiser vos fichiers photos et vidéos. Il se base sur les données EXIF (pour les images) et l'analyse des noms de fichiers existants.

---

## 🚀 Fonctionnalités Implémentées

### Bibliothèque Core (`timeshot_core`)

* 📖 Lecture des métadonnées **EXIF** (`DateTimeOriginal`, `CreateDate`, `Artist`, etc.).
* 📅 Analyse du **nom de fichier** pour détecter des dates existantes.
* 🧠 Génération de **nouveaux noms de fichiers** structurés au format configurable (par défaut: `YYYY-MM-DD_HHMMSS[_suffix][_NomDossierParent]_NomOriginal.ext`).
* ⏱️ Gestion des **rafales** par ajout de suffixes (`_01`, `_02`, ...).
* 🧬 Calcul du hash **BLAKE3** pour chaque fichier.
* ✔️ Détection et marquage des **doublons** basés sur le hash BLAKE3.
* 📂 Analyse **récursive** (optionnelle) des sous-dossiers.
* 💾 Stockage du **chemin complet original** de chaque fichier analysé.
* 📊 Fonctions pour exporter les données d'analyse aux formats **CSV** ou **JSON** (logique présente, pas encore de bouton dans l'UI).

### Interface Graphique (`timeshot_gui`)

* 🖼️ Interface basée sur **GTK4** avec layout vertical (Répertoires / Filtres & Résultats).
* 📂 Ajout/Retrait de multiples répertoires à analyser via une boîte de dialogue native.
* ✔️ Option "Récursif" pour l'analyse des sous-dossiers.
* 📋 **Affichage détaillé** des fichiers analysés dans une liste :
    * Case à cocher pour la sélection.
    * Nom original.
    * Nom proposé par la logique de renommage.
    * Date de prise de vue (extraite des EXIF si disponible).
    * Statut (affiche "Doublon" si détecté).
* 🔍 **Filtres pour affiner la liste des résultats :**
    * Exclusion par extensions (ex: `png, jpg`).
    * Filtrage par expression régulière sur le nom de fichier original.
    * Masquer les fichiers ayant déjà un nom proposé.
    * **(Nouveau)** Masquer les fichiers dont le nom original contient déjà une date (format `YYYY-MM-DD`, `YYYY_MM_DD` ou `YYYYMMDD`).
* 🖱️ **Boutons d'aide à la sélection fonctionnels :** "Tout Sélectionner", "Tout Désélectionner", "Sélectionner si Date EXIF".
* ❗ **Bouton "Renommer Sélection" :** Présent et connecté. Effectue le renommage des fichiers sélectionnés sur le disque en utilisant `std::fs::rename` et affiche un dialogue de résumé. **(Nécessite des tests approfondis par l'utilisateur)**. Met à jour la liste en retirant les éléments renommés.

---

## 📸 Captures d'écran

*(Note : Les captures d'écran actuelles dans le projet peuvent être obsolètes. Il faudrait les mettre à jour pour refléter le nouveau layout vertical et les filtres).*

**(Exemple de placeholder pour une future capture)**
---

## ⚠️ Statut Actuel

* **Prototype Fonctionnel / Beta :** L'application charge les données, propose des noms, permet la sélection, le filtrage avancé et inclut la logique de renommage. Le layout a été amélioré (vertical).
* La **fonctionnalité de renommage doit toujours être testée avec précaution** par l'utilisateur, de préférence sur des copies de fichiers.
* L'export CSV/JSON n'est pas encore accessible depuis l'interface.
* Des avertissements de compilation existent concernant des éléments GTK dépréciés (à corriger).

---

## ⚙️ Prérequis et Installation

1.  **Rust:** Installez Rust et Cargo via [rustup](https://rustup.rs/).
2.  **Dépendances GTK4 & pkg-config:** L'installation dépend de votre système d'exploitation :

    * **🐧 Linux (Debian/Ubuntu):**
        ```bash
        sudo apt update
        sudo apt install libgtk-4-dev pkg-config
        ```

    * **🪟 Windows:**
        La méthode recommandée est d'utiliser **MSYS2**:
        1.  Installez [MSYS2](https://www.msys2.org).
        2.  Ouvrez le terminal **MSYS2 MinGW 64-bit**.
        3.  Installez GTK4 et les outils nécessaires avec `pacman`:
            ```bash
            pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-gsettings-desktop-schemas mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg
            ```
        4.  Assurez-vous que le répertoire `mingw64/bin` de votre installation MSYS2 (ex: `C:\msys64\mingw64\bin`) est ajouté à votre `PATH` Windows.
        5.  Définissez la variable d'environnement `XDG_DATA_DIRS` pour pointer vers le dossier `share` de MinGW64 (ex: `C:\msys64\mingw64\share`).
        6.  Il peut être nécessaire de configurer Rust pour utiliser la toolchain GNU : `rustup default stable-x86_64-pc-windows-gnu`.
        7.  Redémarrez votre terminal après avoir modifié les variables d'environnement.

        **‼️ Note Importante pour l'Exécution sous Windows ‼️**
        Après avoir compilé le projet avec `cargo build` ou `cargo run`, il est très probable que l'application ne se lance pas directement et affiche une erreur `0xc0000139 STATUS_ENTRYPOINT_NOT_FOUND`.
        Ceci est dû au fait que l'exécutable ne trouve pas les fichiers DLL de GTK requis au moment de l'exécution.
        **Solution :** Copiez manuellement les fichiers DLL nécessaires depuis votre installation MSYS2 vers le dossier de sortie de Cargo :
            * **Source :** Le dossier `bin` de votre installation MinGW64 (par défaut `C:\msys64\mingw64\bin`).
            * **Destination :** Le dossier `target/debug` (après `cargo run` ou `cargo build`) ou `target/release` (après `cargo run --release` ou `cargo build --release`) dans le répertoire de votre projet.
            * **Fichiers à copier (au minimum) :** `libgtk-4-1.dll`, `libglib-2.0-0.dll`, `libgobject-2.0-0.dll`, `libgio-2.0-0.dll`, `libpango-1.0-0.dll`, `libgdk_pixbuf-2.0-0.dll`, `libcairo-2.dll`, `libharfbuzz-0.dll`, et potentiellement d'autres (`zlib1.dll`, `libpng16-16.dll`, `fribidi-0.dll`, etc.) si l'erreur persiste.
        Cette copie peut être nécessaire après chaque `cargo clean`.

    * **🍎 macOS:**
        Utilisez [Homebrew](https://brew.sh/):
        ```bash
        brew install gtk4 pkg-config adwaita-icon-theme
        ```

---

## ▶️ Lancer l’application

1.  Clonez le dépôt (ou naviguez dans le dossier du projet).
2.  Ouvrez un terminal dans le dossier racine du projet (`timeshotrenamer_complet_final`).
3.  Compilez l'interface graphique. **Note :** Comme ce projet est un espace de travail (workspace) Cargo, si vous lancez depuis la racine, vous devez spécifier quel paquet compiler/exécuter avec l'option `--package` (ou `-p`).
    ```bash
    # Compiler et lancer en mode Debug (depuis la racine)
    cargo run --package timeshot_gui

    # Compiler et lancer en mode Release (optimisé, depuis la racine)
    cargo run --release --package timeshot_gui

    # Alternative : se placer dans le dossier de la GUI d'abord
    # cd timeshot_gui
    # cargo run
    ```
4.  **(Windows uniquement, si nécessaire)** Si l'application ne se lance pas (erreur `0xc0000139`), copiez les DLLs comme expliqué dans la section d'installation Windows ci-dessus, puis relancez l'exécutable directement depuis `target/debug` ou `target/release`.

---

## 📦 Structure du projet

TimeShotRenamer/
├── .git/               # (Dossier caché de Git)
├── .gitignore          # Fichiers et dossiers ignorés par Git
├── assets/             # (Optionnel) Ressources, comme les captures d'écran
│   └── screenshot.png  #   (Exemple)
├── Cargo.lock          # Fichier de lock des dépendances
├── Cargo.toml          # Manifeste racine (définit le workspace)
├── LICENSE             # Fichier de licence (MIT)
├── README.md           # Ce fichier d'information
│
├── timeshot_core/      # Crate pour la logique métier (bibliothèque)
│   ├── Cargo.toml      #   Manifeste de la crate core
│   └── src/            #   Code source de la crate core
│       ├── exif/       #     Module de lecture EXIF
│       ├── export/     #     Module d'export (CSV, JSON)
│       ├── filename/   #     Module d'analyse des noms de fichiers
│       ├── hash/       #     Module de hachage (BLAKE3)
│       ├── renamer/    #     Module de génération des nouveaux noms
│       ├── lib.rs      #     Point d'entrée de la bibliothèque core
│       └── types.rs    #     Définitions des structures (FileAnalysis, etc.)
│
└── timeshot_gui/       # Crate pour l'interface graphique (binaire)
├── Cargo.toml      #   Manifeste de la crate GUI
└── src/            #   Code source de la crate GUI
├── file_data_item.rs # Définition GObject pour les items de la liste
├── main.rs     #     Point d'entrée de l'application GUI
└── ui.rs       #     Construction de l'interface GTK4


---

---

## 🧪 Dépendances principales

* **GUI:** [gtk4](https://crates.io/crates/gtk4), [glib](https://crates.io/crates/glib), [once_cell](https://crates.io/crates/once_cell) (pour Regex statique)
* **Core:** [chrono](https://crates.io/crates/chrono), [exif](https://crates.io/crates/exif), [blake3](https://crates.io/crates/blake3), [serde](https://serde.rs/), [csv](https://crates.io/crates/csv), [serde_json](https://crates.io/crates/serde_json), [walkdir](https://crates.io/crates/walkdir), [regex](https://crates.io/crates/regex), [once_cell](https://crates.io/crates/once_cell)

---

## 👤 Auteur

Simon Grossi — [github.com/simongrossi](https://github.com/simongrossi)

---

## 📜 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.