mod ui;
pub mod networking;

use gtk::prelude::*;
use gtk::{Application};
use crate::networking::send_message_tcp;

fn main() {
    let _ = send_message_tcp("Started running.");    // Inicializar la aplicación GTK
    let app = Application::builder()
        .application_id("com.example.textbox")
        .build();

    // Conectar la señal "activate" para construir la UI cuando la aplicación se active
    app.connect_activate(|app| {
        ui::build_ui(app);
    });

    // Ejecutar la aplicación
    app.run();
}