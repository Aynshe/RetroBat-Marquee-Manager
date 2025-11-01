use crate::config::Config;
use log::{error, info};
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub fn launch_media_player(config: &Config) {
    kill_media_player(config);

    let launch_command = config.settings.mpv_launch_command
        .replace("{MPVPath}", config.settings.mpv_path.as_ref().unwrap().to_str().unwrap_or(""))
        .replace("{IPCChannel}", &config.settings.ipc_channel)
        .replace("{ScreenNumber}", &config.settings.screen_number.to_string())
        .replace("{DefaultImagePath}", config.settings.default_image_path.as_ref().unwrap().to_str().unwrap_or(""));

    info!("Launching MPV with command: {}", launch_command);

    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg(launch_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    match cmd.spawn() {
        Ok(_) => info!("MPV launched successfully."),
        Err(e) => error!("Failed to launch MPV: {}", e),
    }
}

pub fn kill_media_player(config: &Config) {
    info!("Killing MPV with command: {}", &config.settings.mpv_kill_command);
    let _ = Command::new("cmd")
        .arg("/C")
        .arg(&config.settings.mpv_kill_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn update_marquee(marquee_path: &Path, config: &Config) {
    if let Some(command_template) = config.commands.commands.get("game-selected") {
        let command = command_template
            .replace("{marquee_file}", marquee_path.to_str().unwrap_or(""))
            .replace("{IPCChannel}", &config.settings.ipc_channel);

        info!("Updating marquee with command: {}", command);
        let _ = Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}
