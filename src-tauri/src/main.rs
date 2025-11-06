#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Wry;

fn main() {
    tauri::Builder::<Wry>::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            println!("🪶 Whisper OS backend active...");
            // Example background task (heartbeat)
            tauri::async_runtime::spawn(async move {
                loop {
                    println!("💓 Whisper heartbeat...");
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Whisper OS");
}
