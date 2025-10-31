mod api;
mod config;
mod ipc;
mod log_monitor;
mod systems; // Although unused, we keep it for consistency with the main app

use crate::config::Config;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

#[tokio::main]
async fn main() {
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

    // In a real scenario, the username and API key would come from es_settings.cfg
    let api_client = api::ApiClient::new("testuser".to_string(), "testkey".to_string());

    // Start the log monitor
    let (tx, rx) = channel();
    let log_path = config.settings.retrobat_path.join("emulators/retroarch/logs/retroarch.log");
    let _log_thread = thread::spawn(move || {
        if let Err(e) = log_monitor::start_watching(&log_path, tx) {
            eprintln!("Error in log monitor: {}", e);
        }
    });

    println!("RetroAchievements monitor is running.");

    // Main event loop
    loop {
        if let Ok(log_event) = rx.recv() {
            match log_event {
                log_monitor::LogEvent::GameIdentified(id) => {
                    println!("Game identified: {}", id);
                    match api_client.get_user_profile("TestUser").await {
                        Ok(profile) => {
                            let formatted_profile = ipc::format_user_profile(&profile);
                            ipc::send_to_mpv(&formatted_profile, &config);
                        }
                        Err(e) => eprintln!("Failed to get user profile: {}", e),
                    }
                    match api_client.get_game_info_and_user_progress(id, "TestUser").await {
                        Ok(game_info) => {
                            let formatted_game_info = ipc::format_game_info(&game_info);
                            ipc::send_to_mpv(&formatted_game_info, &config);
                        }
                        Err(e) => eprintln!("Failed to get game info: {}", e),
                    }
                }
                log_monitor::LogEvent::AchievementUnlocked(id) => {
                    println!("Achievement unlocked: {}", id);
                    let notification = format!("achievement|{}", id);
                    ipc::send_to_mpv(&notification, &config);
                }
            }
        }
    }
}
