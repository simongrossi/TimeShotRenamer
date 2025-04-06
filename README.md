# 📸 TimeShotRenamer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**TimeShotRenamer** est un outil développé en Rust avec une interface graphique GTK4 pour analyser, proposer des renommages intelligents et organiser vos fichiers photos. Il se base sur les données EXIF et l'analyse des noms de fichiers existants.

---

## 🚀 Fonctionnalités Implémentées

### Bibliothèque Core (`timeshot_core`)

* 📖 Lecture des métadonnées **EXIF** (`DateTimeOriginal`, `CreateDate`, `Artist`, etc.).
* 📅 Analyse du **nom de fichier** pour détecter des dates existantes.
* 🧠 Génération de **nouveaux noms de fichiers** structurés au format : `YYYY-MM-DD_HHMMSS[_suffix]_NomDossierParent_NomOriginal.ext`.
* ⏱️ Gestion des **rafales** par ajout de suffixes (`_01`, `_02`, ...).
* 🧬 Calcul du hash **BLAKE3** pour chaque fichier.
* ✔️ Détection et marquage des **doublons** basés sur le hash BLAKE3.
* 📂 Analyse **récursive** (optionnelle) des sous-dossiers.
* 💾 Stockage du **chemin complet original** de chaque fichier analysé.
* 📊 Fonctions pour exporter les données d'analyse aux formats **CSV** ou **JSON** (logique présente, pas encore de bouton dans l'UI).

### Interface Graphique (`timeshot_gui`)

* 🖼️ Interface basée sur **GTK4**.
* 📂 Sélection d'un dossier via une boîte de dialogue native.
* ✔️ Option "Inclure les sous-dossiers" pour l'analyse récursive.
* 📋 **Affichage détaillé** des fichiers analysés dans une liste :
    * Case à cocher pour la sélection.
    * Nom original.
    * Nom proposé par la logique de renommage.
    * Date de prise de vue (extraite des EXIF si disponible).
    * Statut (affiche "Doublon" si détecté).
* 🖱️ **Boutons d'aide à la sélection fonctionnels :** "Tout Sélectionner", "Tout Désélectionner", "Sélectionner si Date EXIF".
* ❗ **Bouton "Renommer Sélection" :** Présent et connecté. Effectue le renommage des fichiers sélectionnés sur le disque en utilisant `std::fs::rename` et affiche un dialogue de résumé. **(Nécessite des tests approfondis par l'utilisateur)**. Met à jour la liste en retirant les éléments renommés.

---

## ⚠️ Statut Actuel

* **Prototype Fonctionnel / Beta :** L'application charge les données, propose des noms, permet la sélection et inclut la logique de renommage.
* La **fonctionnalité de renommage doit être testée avec précaution** par l'utilisateur, de préférence sur des copies de fichiers.
* L'export CSV/JSON n'est pas encore accessible depuis l'interface.
* L'alignement visuel des colonnes dans la liste peut être amélioré.

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

    * **🍎 macOS:**
        Utilisez [Homebrew](https://brew.sh/):
        ```bash
        brew install gtk4 pkg-config adwaita-icon-theme
        ```

---

## ▶️ Lancer l’application

1.  Clonez le dépôt (ou naviguez dans le dossier du projet).
2.  Ouvrez un terminal dans le dossier racine du projet (`TimeShotRenamer`).
3.  Compilez et lancez l'interface graphique :
    ```bash
    # Recommandé pour tester la version optimisée
    cargo run --release --package timeshot_gui

    # Ou pour le développement/débogage
    # cd timeshot_gui
    # cargo run
    ```

---

## 📦 Structure du projet

TimeShotRenamer/
├── timeshot_core/      # Bibliothèque principale (logique métier)
│   ├── Cargo.toml
│   └── src/
├── timeshot_gui/       # Interface graphique GTK4
│   ├── Cargo.toml
│   └── src/
├── .gitignore          # Fichiers ignorés par Git
├── LICENSE             # Licence MIT
└── README.md           # Ce fichier


---

## 📝 Exemple de Nom de Fichier Généré (par `timeshot_core`)

Fichier original : `IMG_001.jpg` dans le dossier `Vacances_Ete`.
Date EXIF : `2025-07-15 10:30:00`

Nom généré possible : `2025-07-15_103000_Vacances_Ete_IMG_001.jpg`

---

## 🛠️ TODO / Améliorations Possibles

* [x] Analyse EXIF et noms de fichiers.
* [x] Génération de noms proposés + gestion rafales.
* [x] Hash et détection doublons.
* [x] Interface GTK basique avec liste.
* [x] Analyse récursive optionnelle.
* [x] Boutons de sélection multiple fonctionnels.
* [x] Implémentation bouton Renommer (+ dialogue résumé).
* [ ] **Tester intensivement la fonction Renommer.**
* [ ] Boutons pour Export CSV / JSON.
* [ ] Améliorer l'alignement des colonnes dans la liste.
* [ ] Améliorer le retour visuel pour les doublons.
* [ ] Corriger les avertissements `deprecated clone!`.
* [ ] Ajouter plus de gestion d'erreurs (permissions, I/O pendant renommage).
* [ ] Ajouter une icône d'application.
* [ ] Considérer des options de configuration (format du nom, etc.).
* [ ] Créer des paquets d'installation (MSI, Deb, etc.).

---

## 🧪 Dépendances principales

* **GUI:** [gtk4](https://crates.io/crates/gtk4), [glib](https://crates.io/crates/glib)
* **Core:** [chrono](https://crates.io/crates/chrono), [exif](https://crates.io/crates/exif), [blake3](https://crates.io/crates/blake3), [serde](https://serde.rs/), [csv](https://crates.io/crates/csv), [serde_json](https://crates.io/crates/serde_json), [walkdir](https://crates.io/crates/walkdir), [regex](https://crates.io/crates/regex), [once_cell](https://crates.io/crates/once_cell)

---

## 👤 Auteur

Simon Grossi — [github.com/simongrossi](https://github.com/simongrossi)

---

## 📜 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.
