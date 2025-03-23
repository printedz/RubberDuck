use gtk::prelude::*;
use glib::clone;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Orientation, ScrolledWindow, TextView, TextBuffer};

use crate::networking::send_message_tcp;

// Función para enviar un mensaje a través de TCP/IP

pub fn build_ui(app: &Application) {
    // Crear una nueva ventana de aplicación
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Patito de Goma")
        .default_width(400)
        .default_height(300) // Aumentado para acomodar el historial
        .build();

    // Crear un contenedor vertical para los widgets
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);

    // Crear un TextView para mostrar el historial de mensajes
    let text_view = TextView::new();
    text_view.set_editable(false); // Solo lectura
    text_view.set_wrap_mode(gtk::WrapMode::Word);

    // Crear un buffer para el TextView
    let buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&buffer));

    // Colocar el TextView dentro de un ScrolledWindow para permitir desplazamiento
    let scrolled_window = ScrolledWindow::builder().build();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.add(&text_view);

    // Añadir el ScrolledWindow al contenedor vertical
    vbox.pack_start(&scrolled_window, true, true, 0);

    // Crear un contenedor horizontal para el campo de texto y el botón
    let hbox = Box::new(Orientation::Horizontal, 5);

    // Crear un campo de entrada de texto
    let text_entry = Entry::new();
    text_entry.set_placeholder_text(Some("Ingrese su texto aquí"));
    text_entry.set_hexpand(true);

    // Crear un botón de "Enviar"
    let send_button = Button::with_label("Enviar");

    // Conectar la señal "clicked" del botón
    send_button.connect_clicked(clone!(@weak text_entry, @weak buffer => move |_| {
        let text = text_entry.text().to_string();
        if !text.is_empty() {
            println!("Texto ingresado: {}", text);
            
            // Enviar el mensaje por TCP/IP
            match send_message_tcp(&text) {
                Ok(_) => {
                    println!("Mensaje enviado correctamente");
                    
                    // Añadir el mensaje al historial
                    let mut end_iter = buffer.end_iter();
                    
                    // Añadir una nueva línea si el buffer no está vacío
                    if buffer.char_count() > 0 {
                        buffer.insert(&mut end_iter, "\n");
                    }
                    
                    // Añadir el mensaje con formato
                    buffer.insert(&mut end_iter, &format!("Enviado: {}", text));
                    
                    // Desplazarse al final del texto para mostrar el mensaje más reciente
                    let mut end_iter = buffer.end_iter();
                    text_view.scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
                },
                Err(e) => {
                    println!("Error al enviar el mensaje: {}", e);
                    
                    // Opcionalmente, también puedes mostrar los errores en el historial
                    let mut end_iter = buffer.end_iter();
                    if buffer.char_count() > 0 {
                        buffer.insert(&mut end_iter, "\n");
                    }
                    buffer.insert(&mut end_iter, &format!("Error: No se pudo enviar '{}' - {}", text, e));
                },
            }
            
            text_entry.set_text("");  // Limpiar el campo después de enviar
        }
    }));

    // También permitir enviar al presionar Enter en el campo de texto
    text_entry.connect_activate(clone!(@weak send_button => move |_| {
        send_button.emit_clicked();
    }));

    // Añadir el campo de texto y el botón al contenedor horizontal
    hbox.pack_start(&text_entry, true, true, 0);
    hbox.pack_start(&send_button, false, false, 0);

    // Añadir el contenedor horizontal al contenedor vertical (al final)
    vbox.pack_start(&hbox, false, false, 0);

    // Añadir el contenedor vertical a la ventana
    window.set_child(Some(&vbox));

    // Mostrar la ventana
    window.show_all();
}