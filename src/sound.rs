use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use rodio::{Decoder, OutputStream, Sink};

pub fn play_sound(file_path: &str) -> Result<(), String> {
    // Validar que el archivo existe
    if !Path::new(file_path).exists() {
        return Err(format!("El archivo no existe: {}", file_path));
    }

    // Verificar que es un archivo MP3 (básico)
    if !file_path.to_lowercase().ends_with(".mp3") {
        return Err("El archivo no es un MP3".to_string());
    }

    // Iniciar reproducción en un hilo separado para no bloquear la UI
    let file_path = file_path.to_string();
    thread::spawn(move || {
        match play_mp3_internal(&file_path) {
            Ok(_) => println!("Reproducción de audio finalizada"),
            Err(e) => eprintln!("Error al reproducir audio: {}", e),
        }
    });

    Ok(())
}

fn play_mp3_internal(file_path: &str) -> Result<(), String> {
    // Inicializar el dispositivo de salida de audio
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(stream) => stream,
        Err(e) => return Err(format!("Error al inicializar dispositivo de audio: {}", e)),
    };

    // Crear un Sink para controlar la reproducción
    let sink = match Sink::try_new(&stream_handle) {
        Ok(sink) => sink,
        Err(e) => return Err(format!("Error al crear el sink de audio: {}", e)),
    };

    // Abrir el archivo
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error al abrir el archivo: {}", e)),
    };

    let reader = BufReader::new(file);

    // Decodificar MP3
    let source = match Decoder::new(reader) {
        Ok(source) => source,
        Err(e) => return Err(format!("Error al decodificar MP3: {}", e)),
    };

    // Reproducir audio
    sink.append(source);

    // Esperar a que termine la reproducción
    sink.sleep_until_end();

    Ok(())
}

// Función simple para reproducir un sonido de notificación
pub fn play_notification_sound() -> Result<(), String> {
    // Ruta al sonido de notificación, ajusta según tu proyecto
    play_sound("./assets/notif.mp3")
}

// Función para reproducir sonido cuando se recibe un mensaje
pub fn play_message_received_sound() -> Result<(), String> {
    play_sound("./assets/notif.mp3")
}