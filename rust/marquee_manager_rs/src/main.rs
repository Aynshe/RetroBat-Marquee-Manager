mod config;
mod events;
mod generator;
mod keyboard;
mod marquee;
mod process;
mod state;
mod systems;

use crate::config::Config;
use crate::state::AppState;
use std::path::Path;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Load configurations
    let config_path = Path::new("config.ini");
    let config = match Config::load_config(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            return;
        }
    };
    println!("Config loaded successfully.");

    let systems_path = Path::new("."); // In a real scenario, this would come from the config
    let systems = match systems::load_all_systems_configs(systems_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load systems: {}", e);
            return;
        }
    };
    println!("Systems loaded successfully.");

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
            eprintln!("Error in event watcher: {}", e);
        }
    });

    // Start the keyboard listener in a new thread
    let (tx, rx) = channel();
    let _keyboard_thread = thread::spawn(move || {
        keyboard::start_listening(tx);
    });

    println!("Marquee Manager is running. Press F12 to exit.");

    // Main application loop
    loop {
        if let Ok(key_event) = rx.try_recv() {
            match key_event {
                keyboard::KeyboardEvent::F6 => println!("F6 pressed: Cycle gradient"),
                keyboard::KeyboardEvent::F7 => {
                    println!("F7 pressed, attempting to generate marquee...");
                    let state = app_state.lock().unwrap();
                    if let Some((system, game)) = &state.current_game {
                        if let Some(generated_marquee) =
                            generator::autogen_marquee(system, game, &config)
                        {
                            println!("Generated marquee: {:?}", generated_marquee);
                            process::update_marquee(&generated_marquee, &config);
                        } else {
                            println!("Could not generate marquee: fanart or logo missing.");
                        }
                    } else {
                        println!("No game selected, cannot generate marquee.");
                    }
                }
                keyboard::KeyboardEvent::F8 => println!("F8 pressed: Adjust fanart vertical alignment"),
                keyboard::KeyboardEvent::F9 => println!("F9 pressed: Cycle fanart vertical alignment"),
                keyboard::KeyboardEvent::F10 => println!("F10 pressed: Align logo left"),
                keyboard::KeyboardEvent::F11 => println!("F11 pressed: Align logo center"),
                keyboard::KeyboardEvent::F12 => {
                    println!("F12 pressed, exiting...");
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    // Clean up
    process::kill_media_player(&config);
    println!("Marquee Manager has shut down.");
}
