use crate::config::Config;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn autogen_marquee(
    system_name: &str,
    game_name: &str,
    config: &Config,
) -> Option<PathBuf> {
    let fanart_path = find_fanart_file(system_name, game_name, config);
    let logo_path = find_logo_file(system_name, game_name, config);

    if let (Some(fanart), Some(logo)) = (fanart_path, logo_path) {
        let target_path = config.settings.marquee_image_path.join(format!("{}-{}-generated.png", system_name, game_name));

        let convert_command = config.settings.im_convert_command
            .replace("{IMPath}", config.settings.im_path.to_str().unwrap_or(""))
            .replace("{FanartPath}", fanart.to_str().unwrap_or(""))
            .replace("{LogoPath}", logo.to_str().unwrap_or(""))
            .replace("{ImgTargetPath}", target_path.to_str().unwrap_or(""));

        println!("Generating marquee with command: {}", convert_command);

        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(convert_command);

        if cmd.status().is_ok() {
            return Some(target_path);
        }
    }
    None
}

fn find_fanart_file(system_name: &str, game_name: &str, config: &Config) -> Option<PathBuf> {
    let fanart_path = config.settings.roms_path.join(system_name).join("images").join(format!("{}-fanart.jpg", game_name));
    if fanart_path.exists() {
        return Some(fanart_path);
    }
    None
}

fn find_logo_file(system_name: &str, game_name: &str, config: &Config) -> Option<PathBuf> {
    let logo_path = config.settings.roms_path.join(system_name).join("images").join(format!("{}-marquee.png", game_name));
    if logo_path.exists() {
        return Some(logo_path);
    }
    None
}
