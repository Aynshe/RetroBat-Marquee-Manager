mod config;
mod events;
mod generator;
mod keyboard;
mod logger;
mod marquee;
mod process;
mod registry;
mod state;
mod systems;

use crate::config::Config;
use crate::state::AppState;
use log::{error, info, warn};
use std::path::{Path, PathBuf};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    logger::init();
    info!("Marquee Manager starting...");

    // Load configurations
    let config_path = Path::new("config.ini");
    let mut config = match Config::load_config(config_path) {
        Ok(c) => {
            info!("Config loaded successfully.");
            c
        }
        Err(e) => {
            error!("Failed to load config: {}", e);
            return;
        }
    };

    // --- RetroBat Path Logic ---
    if config.settings.retrobat_path.is_none() {
        warn!("RetroBatPath not found in config.ini. Attempting to read from Windows Registry...");
        if let Some(path_str) = registry::get_retrobat_path() {
            info!("Found RetroBat path in registry: {}", path_str);
            config.settings.retrobat_path = Some(PathBuf::from(path_str));
        } else {
            error!("Could not find RetroBat installation path in config or registry. Exiting.");
            return;
        }
    }
    // --- End RetroBat Path Logic ---


    let systems_path = Path::new("."); // In a real scenario, this would come from the config
    let systems = match systems::load_all_systems_configs(systems_path) {
        Ok(s) => {
            info!("Systems loaded successfully.");
            s
        }
        Err(e) => {
            error!("Failed to load systems: {}", e);
            return;
        }
    };

    // Create a shared state
    let app_state = Arc::new(Mutex::new(AppState::default()));

    // Launch the media player
    process::launch_media_player(&config);

    // Start the event watcher in a new thread
    let event_config = config.clone();
    let event_systems = systems.clone();
    let event_state = app_state.clone();
    let _event_thread = thread::spawn(move || {
        if let Err(e) = events::start_watching(&event_config, &event_systems, event_state) {
            error!("Error in event watcher: {}", e);
        }
    });
    info!("Event watcher started.");

    // Start the keyboard listener in a new thread
    let (tx, rx) = channel();
    let _keyboard_thread = thread::spawn(move || {
        keyboard::start_listening(tx);
    });
    info!("Keyboard listener started.");

    info!("Marquee Manager is running. Press F12 to exit.");

    // Main application loop
    loop {
        if let Ok(key_event) = rx.try_recv() {
            match key_event {
                keyboard::KeyboardEvent::F6 => info!("F6 pressed: Cycle gradient"),
                keyboard::KeyboardEvent::F7 => {
                    info!("F7 pressed, attempting to generate marquee...");
                    let state = app_state.lock().unwrap();
                    if let Some((system, game)) = &state.current_game {
                        if let Some(generated_marquee) =
                            generator::autogen_marquee(system, game, &config)
                        {
                            info!("Generated marquee: {:?}", generated_marquee);
                            process::update_marquee(&generated_marquee, &config);
                        } else {
                            error!("Could not generate marquee: fanart or logo missing.");
                        }
                    } else {
                        info!("No game selected, cannot generate marquee.");
                    }
                }
                keyboard::KeyboardEvent::F8 => info!("F8 pressed: Adjust fanart vertical alignment"),
                keyboard::KeyboardEvent::F9 => info!("F9 pressed: Cycle fanart vertical alignment"),
                keyboard::KeyboardEvent::F10 => info!("F10 pressed: Align logo left"),
                keyboard::KeyboardEvent::F11 => info!("F11 pressed: Align logo center"),
                keyboard::KeyboardEvent::F12 => {
                    info!("F12 pressed, exiting...");
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    // Clean up
    process::kill_media_player(&config);
    info!("Marquee Manager has shut down.");
}
