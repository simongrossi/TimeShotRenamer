# 📝 Changelog - TimeShotRenamer

## [0.2.0] - 2025-04-03
### ✨ Nouvelles fonctionnalités
- Ajout d'une **fenêtre modale EXIF** lisible avec scroll fluide, recherche, et regroupement par catégories (📅 Dates, 📸 Appareil…)
- Possibilité d'insérer une donnée EXIF **après un nombre de caractères défini**
- Interface **adaptative** à la taille de la fenêtre
- Ajout d'une **case à cocher pour l'inclusion récursive** des sous-dossiers
- Support du **glisser-déposer de dossier** dans l'application

### 🧠 Améliorations internes
- Les tags EXIF sont désormais affichés avec des noms lisibles (`DateTimeOriginal` au lieu de `Tag(Exif, 36867)`)
- Nettoyage et tronquage automatique des valeurs EXIF trop longues ou illisibles
- Aperçu dynamique du nouveau nom avec insertion de données EXIF simulée à la volée

### 🛠 Corrections
- Correction du bug lié à l’absence de récursion contrôlée dans `logic.rs`
- Affichage plus stable et clair dans l’aperçu du tableau des fichiers

---

🔗 Voir le dépôt : https://github.com/simongrossi/TimeShotRenamer

