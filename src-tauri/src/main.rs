// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, simulate, Button, EventType, Key};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Mapping {
    mouse_button: u8,
    keyboard_key: String,
}

struct AppState {
    mappings: Arc<Mutex<Vec<Mapping>>>,
}

#[tauri::command]
fn add_mapping(
    state: tauri::State<AppState>,
    mapping: Mapping,
) -> Result<(), String> {
    let mut mappings = state.mappings.lock().map_err(|e| e.to_string())?;
    mappings.retain(|m| m.mouse_button != mapping.mouse_button);
    mappings.push(mapping);
    save_mappings(&mappings)
}

#[tauri::command]
fn remove_mapping(
    state: tauri::State<AppState>,
    mouse_button: u8,
) -> Result<(), String> {
    let mut mappings = state.mappings.lock().map_err(|e| e.to_string())?;
    mappings.retain(|m| m.mouse_button != mouse_button);
    save_mappings(&mappings)
}

#[tauri::command]
fn get_mappings(state: tauri::State<AppState>) -> Vec<Mapping> {
    state.mappings.lock().unwrap().clone()
}

fn start_event_listener(mappings: Arc<Mutex<Vec<Mapping>>>) {
    thread::spawn(move || {
        let listener_mappings = Arc::clone(&mappings);
        listen(move |event| {
            let mappings = listener_mappings.lock().unwrap();

            match event.event_type {
                EventType::ButtonPress(button)
                | EventType::ButtonRelease(button) => {
                    let button_u8 = match button {
                        Button::Left => 0,
                        Button::Middle => 1,
                        Button::Right => 2,
                        Button::Unknown(2) => 3,
                        Button::Unknown(1) => 4,
                        _ => 0,
                    };

                    if let Some(mapping) =
                        mappings.iter().find(|m| m.mouse_button == button_u8)
                    {
                        let is_press = matches!(
                            event.event_type,
                            EventType::ButtonPress(_)
                        );
                        // println!(
                        //     "Mouse button {} {}, simulating key {}",
                        //     button_u8,
                        //     if is_press { "pressed" } else { "released" },
                        //     mapping.keyboard_key
                        // );

                        let keyboard_key = mapping.keyboard_key.clone();
                        thread::spawn(move || {
                            if let Some(key) = match keyboard_key.as_str() {
                                "=" => Some(Key::Equal),
                                // Add more key mappings as needed
                                _ => None,
                            } {
                                let event_type = if is_press {
                                    EventType::KeyPress(key)
                                } else {
                                    EventType::KeyRelease(key)
                                };
                                simulate(&event_type).unwrap();
                            }
                        });
                    }
                }
                _ => {}
            }
        })
        .unwrap();
    });
}

fn get_mappings_file_path() -> PathBuf {
    let mut path =
        tauri::api::path::app_data_dir(&tauri::Config::default()).unwrap();
    path.push("mappings.json");
    path
}

fn save_mappings(mappings: &Vec<Mapping>) -> Result<(), String> {
    let path = get_mappings_file_path();
    let json = serde_json::to_string(mappings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn load_mappings() -> Vec<Mapping> {
    let path = get_mappings_file_path();
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let mappings = Arc::new(Mutex::new(load_mappings()));
            let state = AppState {
                mappings: mappings.clone(),
            };
            app.manage(state);

            // Start the event listener in a separate thread
            start_event_listener(mappings);

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } =
                event.event()
            {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if !(window.is_visible().unwrap()) {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            add_mapping,
            remove_mapping,
            get_mappings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
