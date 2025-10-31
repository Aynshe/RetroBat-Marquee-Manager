use crate::api::{GameInfoAndUserProgress, UserProfile};
use crate::config::Config;
use std::fs::OpenOptions;
use std::io::Write;

pub fn send_to_mpv(data: &str, config: &Config) {
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .open(&config.settings.ipc_channel)
    {
        let command = format!("script-message ra-data {}\n", data);
        if let Err(e) = file.write_all(command.as_bytes()) {
            eprintln!("Failed to write to MPV pipe: {}", e);
        }
    } else {
        eprintln!("Failed to open MPV pipe at {}", config.settings.ipc_channel);
    }
}

pub fn format_user_profile(profile: &UserProfile) -> String {
    format!(
        "user_info|{}|{}",
        profile.user,
        profile.user_pic.replace("\\", "\\\\")
    )
}

pub fn format_game_info(game_info: &GameInfoAndUserProgress) -> String {
    let achievements_str = game_info
        .achievements
        .values()
        .map(|ach| {
            format!(
                "{};{};{};{}",
                ach.title,
                ach.description,
                ach.points,
                ach.badge_name.replace("\\", "\\\\")
            )
        })
        .collect::<Vec<String>>()
        .join("|");

    format!(
        "game_info|{}|{}|{}|{}|{}",
        game_info.title,
        game_info.game_icon.replace("\\", "\\\\"),
        game_info.num_awarded_to_user,
        game_info.user_completion,
        achievements_str
    )
}
