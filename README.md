<<<<<<< HEAD
Markdown

# 📸 TimeShotRenamer

**TimeShotRenamer** est un outil Rust modulaire conçu pour analyser, renommer et organiser intelligemment vos fichiers photos. Il est composé d'une bibliothèque *core* pour la logique métier et d'une interface graphique *GUI* (en cours de développement) basée sur GTK4.

---

## ✨ Fonctionnalités

### Bibliothèque Core (`timeshot_core`)

La bibliothèque `timeshot_core` fournit les fonctionnalités suivantes :

-   📖 Lecture des métadonnées **EXIF** (`DateTimeOriginal`, `CreateDate`, `Artist`, etc.) via la crate `exif`.
-   📅 Analyse du **nom de fichier** pour détecter des dates existantes (même mal formatées).
-   🧠 Génération de **nouveaux noms de fichiers** structurés, incluant la date, l'heure, le nom du dossier parent et le nom original (`YYYY-MM-DD_HHMMSS_nomdossier_nomoriginal.ext`).
-   ⏱️ Gestion des **rafales** (fichiers pris à la même seconde) par ajout de suffixes (`_01`, `_02`, ...).
-    Hashing des fichiers via **BLAKE3** pour la détection future des doublons.
-   📊 Export des données d'analyse (résultats potentiels) aux formats **CSV** ou **JSON**.

### Interface Graphique (`timeshot_gui`) - *Prototype*

L'interface graphique `timeshot_gui` est **actuellement un prototype** et offre les fonctionnalités suivantes :

-   🖼️ Fenêtre principale simple basée sur **GTK4**.
-   📂 Bouton "Ouvrir un dossier" permettant de sélectionner un répertoire via une boîte de dialogue native.
-   🏷️ Affichage du chemin du dossier sélectionné.
-   📋 **Affichage d'une liste de fichiers (ListView)** avec une case à cocher et le nom du fichier.
-   ⚠️ **Important :** Actuellement, la liste affiche des **données factices** à des fins de démonstration et de développement. L'intégration avec `timeshot_core` pour analyser le contenu du dossier sélectionné et afficher les vrais fichiers **n'est pas encore implémentée**.

---

## 📦 Structure du projet

La structure actuelle est la suivante :

TimeShotRenamer/
├── timeshot_core/      # Bibliothèque principale (logique métier)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── types.rs      # Structures de données (FileAnalysis, ExifData)
│       ├── exif/         # Lecture EXIF
│       ├── filename/     # Analyse des noms de fichiers
│       ├── renamer/      # Génération des nouveaux noms
│       ├── hash/         # Calcul et détection de hash (doublons)
│       └── export/       # Export CSV / JSON
│
├── timeshot_gui/       # Interface graphique GTK4
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── ui.rs         # Construction de l'interface GTK
│       └── file_data_item.rs # Objet GObject pour les données de fichier dans la liste
│
└── README.md           # Ce fichier


---

## ⚙️ Prérequis et Installation

Assurez-vous d'avoir les éléments suivants installés :

1.  **Rust:** Installez Rust via [rustup](https://rustup.rs/).
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
            pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg
            ```
        4.  Assurez-vous que le répertoire `mingw64/bin` de votre installation MSYS2 (ex: `C:\msys64\mingw64\bin`) est ajouté à votre `PATH` Windows.
        5.  Il peut être nécessaire de configurer Rust pour utiliser la toolchain GNU : `rustup default stable-x86_64-pc-windows-gnu`

    * **🍎 macOS:**
        Utilisez [Homebrew](https://brew.sh/):
        ```bash
        brew install gtk4 pkg-config
        ```

---

## 🚀 Lancer l’application GUI (Prototype)

1.  Naviguez dans le dossier de l'interface graphique :
    ```bash
    cd timeshot_gui
    ```
2.  Lancez l'application avec Cargo :
    ```bash
    cargo run
    ```
    > L'application s'ouvrira, vous pourrez sélectionner un dossier, mais la liste affichera des données d'exemple.

---

## 🧩 Utiliser la Bibliothèque Core

Vous pouvez compiler et utiliser la bibliothèque `timeshot_core` indépendamment pour l'intégrer dans d'autres projets ou scripts :

```bash
cd timeshot_core
cargo build
📝 Exemple de Nom de Fichier Généré (par timeshot_core)
Basé sur la logique actuelle de timeshot_core/src/renamer/generator.rs :

Fichier original : IMG_001.jpg dans le dossier Vacances_Ete
Date EXIF : 2025-04-05 14:30:12

Nom généré possible : 2025-04-05_143012_Vacances_Ete_IMG_001.jpg

En cas de rafale (même seconde) : 2025-04-05_143012_01_Vacances_Ete_IMG_002.jpg

🛠️ TODO / Prochaines Étapes
Priorités pour faire évoluer le prototype GUI :

[ ] Intégrer timeshot_core : Appeler la logique d'analyse de timeshot_core lors de la sélection d'un dossier dans la GUI.
[ ] Afficher les vraies données : Remplacer les données factices par les informations réelles des fichiers analysés (nom original, statut, date trouvée, etc.) dans le ListView. Adapter FileDataItem si besoin.
[ ] Afficher le nom proposé : Ajouter une colonne dans le ListView pour montrer le nom de fichier qui serait généré.
[ ] Implémenter le renommage : Ajouter un bouton "Renommer" qui utilise timeshot_core pour renommer les fichiers sélectionnés dans la liste.
[ ] Gestion des doublons : Utiliser l'information is_duplicate de FileAnalysis pour marquer visuellement les doublons dans la liste.
[ ] Boutons d'Export : Ajouter des boutons pour déclencher l'export CSV/JSON depuis la GUI.
[ ] Améliorer la gestion des erreurs et les retours utilisateur dans la GUI.
[ ] (Optionnel) Créer une interface en ligne de commande (CLI) pure utilisant timeshot_core.
🧪 Dépendances principales
GUI: gtk4, glib
Core: chrono, exif, blake3, serde, csv, serde_json, walkdir, regex
👤 Auteur
Simon Grossi — github.com/simongrossi

📜 Licence
MIT


**Changements clés :**

1.  **Distinction Core/GUI :** Séparation claire des fonctionnalités implémentées dans `timeshot_core` et de l'état actuel de `timeshot_gui`.
2.  **Statut GUI:** Le terme "Prototype" est utilisé, et il est explicitement mentionné que la GUI utilise des données factices et n'est pas encore connectée au core.
3.  **Installation:** Instructions mises à jour et plus détaillées pour Linux, Windows (MSYS2) et macOS (suggestion Homebrew). Mention de `pkg-config` (ou `pkgconf` sur MSYS2) comme dépendance clé.
4.  **TODO List:** Mise à jour pour refléter les prochaines étapes logiques, en commençant par l'intégration core-GUI.
5.  **Précisions:** Ajout de petites précisions (ex: gestion des rafales, format du nom de fichier généré).
6.  **Dépendances:** Liste des dépendances légèrement étendue pour inclure celles utilisées dans le core comme `walkdir` et `regex`.

J'espère que cette version correspond mieux à l'état actuel de votre projet !
=======
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

>>>>>>> 625bdd35065cdd6eb7ecea58f3f4f80fc1fddd0e
