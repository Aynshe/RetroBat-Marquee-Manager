use crate::config::Config;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub enum MarqueeType<'a> {
    System {
        system_name: &'a str,
    },
    Game {
        system_name: &'a str,
        game_name: &'a str,
    },
    Collection {
        collection_name: &'a str,
    },
}

pub fn find_marquee_file(
    marquee_type: MarqueeType,
    config: &Config,
    systems: &HashMap<String, String>,
) -> PathBuf {
    let default_path = config.settings.default_image_path.as_ref().unwrap().clone();
    match marquee_type {
        MarqueeType::System { system_name } => {
            find_system_marquee(system_name, config, systems).unwrap_or(default_path)
        }
        MarqueeType::Game { system_name, game_name } => {
            find_game_marquee(system_name, game_name, config, systems).unwrap_or(default_path)
        }
        MarqueeType::Collection { collection_name } => {
            find_collection_marquee(collection_name, config).unwrap_or(default_path)
        }
    }
}

fn find_system_marquee(
    system_name: &str,
    config: &Config,
    systems: &HashMap<String, String>,
) -> Option<PathBuf> {
    let system_folder = systems.get(system_name).map(|s| s.as_str()).unwrap_or(system_name);
    let marquee_path_str = config.settings.system_file_path.replace("{system_name}", system_folder);
    let full_marquee_path = config.settings.system_marquee_path.as_ref().unwrap().join(marquee_path_str);
    find_file(&full_marquee_path, &config.settings.accepted_formats)
}

fn find_game_marquee(
    system_name: &str,
    game_name: &str,
    config: &Config,
    systems: &HashMap<String, String>,
) -> Option<PathBuf> {
    let system_folder = systems.get(system_name).map(|s| s.as_str()).unwrap_or(system_name);
    let marquee_path_str = config.settings.marquee_file_path
        .replace("{system_name}", system_folder)
        .replace("{game_name}", game_name);

    let full_marquee_path = config.settings.marquee_image_path.as_ref().unwrap().join(marquee_path_str);
    if let Some(path) = find_file(&full_marquee_path, &config.settings.accepted_formats) {
        return Some(path);
    }

    let marquee_path_default_str = config.settings.marquee_file_path_default
        .replace("{system_name}", system_folder)
        .replace("{game_name}", game_name);

    let full_marquee_path_default = config.settings.marquee_image_path_default.as_ref().unwrap().join(marquee_path_default_str);
    if let Some(path) = find_file(&full_marquee_path_default, &config.settings.accepted_formats) {
        return Some(path);
    }

    find_system_marquee(system_name, config, systems)
}

fn find_collection_marquee(collection_name: &str, config: &Config) -> Option<PathBuf> {
    let marquee_path_str = config.settings.collection_file_path.replace("{collection_name}", collection_name);
    let full_marquee_path = config.settings.collection_marquee_path.as_ref().unwrap().join(marquee_path_str);
    find_file(&full_marquee_path, &config.settings.accepted_formats)
}

fn find_file(base_path: &Path, accepted_formats: &str) -> Option<PathBuf> {
    for ext in accepted_formats.split(',') {
        let path = base_path.with_extension(ext.trim());
        if path.exists() {
            return Some(path);
        }
    }
    None
}
