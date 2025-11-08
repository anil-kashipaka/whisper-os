#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState};
use std::time::{Duration, Instant};
use tauri_plugin_notification::NotificationExt;



fn main() {
    #[cfg(target_os = "windows")]
{
    use windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;
    use windows::core::PCWSTR;
    unsafe {
        let id = widestring::U16CString::from_str("com.anil.whisperos.dev").unwrap();
        SetCurrentProcessExplicitAppUserModelID(PCWSTR(id.as_ptr()));
    }
}

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            println!("🪶 Whisper Nudge Engine with Notifications started...");

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let device_state = DeviceState::new();
                let mut last_input = Instant::now();
                let mut last_mouse_pos = (0, 0);

                loop {
                    let keys = device_state.get_keys();
                    let mouse = device_state.get_mouse();

                    // Check for keyboard activity or mouse movement
                    if !keys.is_empty() || mouse.coords != last_mouse_pos {
                        last_input = Instant::now();
                        last_mouse_pos = mouse.coords;
                    }

                    let idle_time = last_input.elapsed().as_secs();

                    // Nudge logic
                    if idle_time > 60 {
                        println!("🧘 Whisper: sending notification...");
                    
                        handle
                            .notification()
                            .builder()
                            .title("🪶 Whisper")
                            .body("Time to stretch your mind and body!")
                            .show()
                            .unwrap();
                    
                        last_input = Instant::now();
                    }
                    
                    
                    std::thread::sleep(Duration::from_secs(10));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Whisper OS");
}
