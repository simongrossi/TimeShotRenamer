// timeshot_gui/src/main.rs

mod ui;
mod file_data_item; // Assurez-vous que ce module est bien déclaré

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.example.timeshotrenamergtk";

fn main() {
    println!("DEBUG: main() - Début"); // <--- AJOUTÉ ICI
    let app = Application::builder().application_id(APP_ID).build();

    // Connexion du signal activate
    app.connect_activate(|app| {
        println!("DEBUG: connect_activate - Appel de build_ui"); // <--- AJOUTÉ ICI
        ui::build_ui(app);
    });

    println!("DEBUG: main() - Avant app.run()"); // <--- AJOUTÉ ICI
    app.run(); // Lance la boucle principale GTK et affiche la fenêtre (si build_ui est correct)
    println!("DEBUG: main() - Après app.run()"); // <-- Sera affiché seulement quand l'app se ferme proprement
}