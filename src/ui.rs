use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry, Label, Orientation,
    ScrolledWindow, TextView, ResponseType
};
use crate::networking::send_message_tcp;
use crate::sound::play_notification_sound;

pub fn build_ui(app: &Application) {
    // Solicitar nombre de usuario al iniciar
    let username = request_username(app);

    // Crear ventana principal con el nombre de usuario
    create_main_window(app, &username);
}

fn request_username(app: &Application) -> String {
    // Crear un diálogo modal para pedir el nombre de usuario
    let dialog = gtk::Dialog::new();
    dialog.set_title("Iniciar sesión");
    dialog.set_modal(true);
    dialog.set_default_width(350);
    dialog.set_default_height(150);
    dialog.set_application(Some(app));

    // Obtener el content area del diálogo
    let content_area = dialog.content_area();

    // Crear una caja vertical para organizar los elementos
    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    // Crear el label con instrucciones
    let label = Label::new(Some("Por favor, introduce tu nombre de usuario:"));
    vbox.pack_start(&label, false, false, 0);

    // Crear campo de texto para el nombre de usuario
    let username_entry = Entry::new();
    username_entry.set_activates_default(true);
    vbox.pack_start(&username_entry, false, false, 0);

    // Añadir la caja vertical al content area
    content_area.add(&vbox);

    // Añadir botones al diálogo
    dialog.add_button("Cancelar", ResponseType::Cancel);
    let ok_button = dialog.add_button("Aceptar", ResponseType::Accept);
    ok_button.set_sensitive(false);
    dialog.set_default_response(ResponseType::Accept);

    // Habilitar el botón Aceptar solo cuando hay texto
    let ok_button_clone = ok_button.clone();
    username_entry.connect_changed(move |entry| {
        let text = entry.text();
        ok_button_clone.set_sensitive(!text.is_empty());
    });

    // Mostrar el diálogo
    dialog.show_all();

    // Variable para almacenar el nombre de usuario
    let mut username = String::from("Usuario");

    // Ejecutar el diálogo y procesar la respuesta
    let response = dialog.run();

    if response == ResponseType::Accept {
        let text = username_entry.text().to_string();
        if !text.is_empty() {
            username = text;
        }
    }

    // Cerrar y destruir el diálogo
    dialog.hide();
    
    unsafe {
        dialog.destroy();
    }
    username
}

fn create_main_window(app: &Application, username: &str) {
    // Crear la ventana principal
    let window = ApplicationWindow::new(app);
    window.set_title(&format!("Chat - {}", username));
    window.set_default_size(600, 400);

    // Crear una caja vertical para organizar los elementos
    let vbox = GtkBox::new(Orientation::Vertical, 5);

    // Mostrar nombre de usuario en la ventana
    let welcome_label = Label::new(Some(&format!("Bienvenido, {}!", username)));
    welcome_label.set_margin_top(10);
    welcome_label.set_margin_bottom(5);
    welcome_label.set_halign(gtk::Align::Start);
    welcome_label.set_margin_start(10);
    vbox.pack_start(&welcome_label, false, false, 0);

    // Crear un área de scroll para el TextView de mensajes
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_hscrollbar_policy(gtk::PolicyType::Never);
    scrolled_window.set_vscrollbar_policy(gtk::PolicyType::Automatic);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    // Crear TextView para mostrar mensajes
    let text_view = TextView::new();
    text_view.set_margin_top(10);
    text_view.set_margin_bottom(10);
    text_view.set_margin_start(10);
    text_view.set_margin_end(10);
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    let buffer = text_view.buffer().expect("Error al obtener el buffer");

    // Añadir TextView al ScrolledWindow
    scrolled_window.add(&text_view);

    // Crear caja horizontal para entrada de texto y botón
    let hbox = GtkBox::new(Orientation::Horizontal, 5);
    hbox.set_margin_top(5);
    hbox.set_margin_bottom(5);
    hbox.set_margin_start(5);
    hbox.set_margin_end(5);

    // Entrada de texto
    let text_entry = Entry::new();
    text_entry.set_hexpand(true);
    // Corregir: Envolver el texto en Some()
    text_entry.set_placeholder_text(Some("Escribe un mensaje..."));

    // Botón de enviar
    let send_button = Button::with_label("Enviar");
    send_button.set_sensitive(false);

    // Habilitar/deshabilitar botón según contenido de texto
    let send_button_clone = send_button.clone();
    text_entry.connect_changed(move |entry| {
        let text = entry.text();
        send_button_clone.set_sensitive(!text.is_empty());
    });

    // Función para enviar mensajes (usando clonación explícita)
    let send_message = {
        let buffer = buffer.clone();
        let text_entry = text_entry.clone();
        let username = username.to_string();

        move || {
            let text = text_entry.text().to_string();
            if !text.is_empty() {
                let formatted_message = format!("{}: {}\n", username, text);

                buffer.insert_at_cursor(&formatted_message);

                // Enviar mensaje por la red
                match send_message_tcp(&format!("[{}] {}", username, text)) {
                    Ok(_) => {
                        // Reproducir sonido de notificación
                        match play_notification_sound() {
                            Ok(_) => {},
                            Err(e) => println!("Error al reproducir notificación: {}", e),
                        }
                    },
                    Err(e) => {
                        let error_msg = format!("Error al enviar mensaje: {}\n", e);
                        buffer.insert_at_cursor(&error_msg);
                    }
                }

                text_entry.set_text("");
            }
        }
    };

    // Enviar mensaje al presionar Enter
    let send_message_clone = send_message.clone();
    text_entry.connect_activate(move |_| {
        send_message_clone();
    });

    // Enviar mensaje al hacer clic en el botón
    send_button.connect_clicked(move |_| {
        send_message();
    });

    // Añadir elementos a sus contenedores
    hbox.pack_start(&text_entry, true, true, 0);
    hbox.pack_end(&send_button, false, false, 0);

    vbox.pack_start(&scrolled_window, true, true, 0);
    vbox.pack_end(&hbox, false, false, 0);

    // Añadir la caja vertical a la ventana
    window.add(&vbox);

    // Mostrar la ventana
    window.show_all();
}