use serde::{de::{self, Deserializer}, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use log::info;

// ... (default value functions and custom deserializer remain the same) ...

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    // ... (fields remain the same) ...
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_marquee_width")]
    pub marquee_width: i32,
    #[serde(default = "default_marquee_height")]
    pub marquee_height: i32,
    #[serde(default = "default_marquee_border")]
    pub marquee_border: i32,
    #[serde(default, deserialize_with = "deserialize_bool_from_string_opt")]
    pub marquee_auto_convert: bool,
    #[serde(default = "default_accepted_formats")]
    pub accepted_formats: String,
    #[serde(rename = "RetroBatPath")]
    pub retrobat_path: Option<PathBuf>,
    #[serde(rename = "RomsPath")]
    pub roms_path: Option<PathBuf>,
    #[serde(rename = "DefaultImagePath")]
    pub default_image_path: Option<PathBuf>,
    #[serde(rename = "MarqueeImagePath")]
    pub marquee_image_path: Option<PathBuf>,
    #[serde(default = "default_marquee_file_path")]
    pub marquee_file_path: String,
    #[serde(rename = "MarqueeImagePathDefault")]
    pub marquee_image_path_default: Option<PathBuf>,
    #[serde(default = "default_marquee_file_path_default")]
    pub marquee_file_path_default: String,
    #[serde(default, deserialize_with = "deserialize_bool_from_string_opt")]
    pub marquee_auto_scraping: bool,
    #[serde(default, deserialize_with = "deserialize_bool_from_string_opt")]
    pub marquee_auto_scraping_debug: bool,
    #[serde(rename = "SystemMarqueePath")]
    pub system_marquee_path: Option<PathBuf>,
    #[serde(default = "default_system_file_path")]
    pub system_file_path: String,
    #[serde(rename = "CollectionMarqueePath")]
    pub collection_marquee_path: Option<PathBuf>,
    #[serde(default = "default_collection_file_path")]
    pub collection_file_path: String,
    #[serde(default = "default_collection_alternativ_names")]
    pub collection_alternativ_names: String,
    #[serde(default = "default_collection_correlation")]
    pub collection_correlation: String,
    #[serde(default = "default_ipc_channel")]
    pub ipc_channel: String,
    #[serde(default = "default_screen_number")]
    pub screen_number: i32,
    #[serde(rename = "MPVPath")]
    pub mpv_path: Option<PathBuf>,
    #[serde(default = "default_mpv_launch_command")]
    pub mpv_launch_command: String,
    #[serde(default = "default_mpv_kill_command")]
    pub mpv_kill_command: String,
    #[serde(default = "default_mpv_test_command")]
    pub mpv_test_command: String,
    #[serde(rename = "IMPath")]
    pub im_path: Option<PathBuf>,
    #[serde(default = "default_im_convert_command")]
    pub im_convert_command: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: i32,
    #[serde(default, deserialize_with = "deserialize_bool_from_string_opt")]
    pub log_file: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Commands {
    #[serde(flatten)]
    pub commands: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "Settings")]
    pub settings: Settings,
    #[serde(rename = "Commands")]
    pub commands: Commands,
}

impl Config {
    pub fn load_config(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = serde_ini::from_str(&content)?;
        Ok(config)
    }

    pub fn complete_paths(&mut self, retrobat_path: &Path) {
        let current_dir = std::env::current_dir().unwrap();

        if self.settings.roms_path.is_none() {
            let path = retrobat_path.join("roms");
            info!("RomsPath not set, defaulting to: {:?}", path);
            self.settings.roms_path = Some(path);
        }
        if self.settings.default_image_path.is_none() {
            let path = current_dir.join("images/default.png");
            info!("DefaultImagePath not set, defaulting to: {:?}", path);
            self.settings.default_image_path = Some(path);
        }
        if self.settings.marquee_image_path.is_none() {
            let path = current_dir.join("images");
            info!("MarqueeImagePath not set, defaulting to: {:?}", path);
            self.settings.marquee_image_path = Some(path);
        }
        if self.settings.marquee_image_path_default.is_none() {
            let path = retrobat_path.join("roms");
            info!("MarqueeImagePathDefault not set, defaulting to: {:?}", path);
            self.settings.marquee_image_path_default = Some(path);
        }
        if self.settings.system_marquee_path.is_none() {
            let path = retrobat_path.join("emulationstation/.emulationstation/themes/es-theme-carbon/art/logos");
            info!("SystemMarqueePath not set, defaulting to: {:?}", path);
            self.settings.system_marquee_path = Some(path);
        }
        if self.settings.collection_marquee_path.is_none() {
            let path = retrobat_path.join("emulationstation/.emulationstation/themes/es-theme-carbon/art/logos");
            info!("CollectionMarqueePath not set, defaulting to: {:?}", path);
            self.settings.collection_marquee_path = Some(path);
        }
        if self.settings.mpv_path.is_none() {
            let path = current_dir.join("mpv/mpv.exe");
            info!("MPVPath not set, defaulting to: {:?}", path);
            self.settings.mpv_path = Some(path);
        }
        if self.settings.im_path.is_none() {
            let path = current_dir.join("imagemagick/convert.exe");
            info!("IMPath not set, defaulting to: {:?}", path);
            self.settings.im_path = Some(path);
        }
    }
}

// --- Default values for optional fields ---
fn default_language() -> String { "en".to_string() }
fn default_marquee_width() -> i32 { 1920 }
fn default_marquee_height() -> i32 { 360 }
fn default_marquee_border() -> i32 { 0 }
fn default_accepted_formats() -> String { "png,jpg,gif,mp4".to_string() }
fn default_marquee_file_path() -> String { "{system_name}-{game_name}".to_string() }
fn default_marquee_file_path_default() -> String { "{system_name}/images/{game_name}-marquee".to_string() }
fn default_system_file_path() -> String { "{system_name}".to_string() }
fn default_collection_file_path() -> String { "auto-{collection_name}".to_string() }
fn default_collection_alternativ_names() -> String { "".to_string() }
fn default_collection_correlation() -> String { "".to_string() }
fn default_ipc_channel() -> String { "\\\\.\\pipe\\mpv-pipe".to_string() }
fn default_screen_number() -> i32 { 1 }
fn default_mpv_launch_command() -> String { "\"{MPVPath}\" --input-ipc-server={IPCChannel} --screen={ScreenNumber} --no-border --ontop --autofit-larger=100%x100% \"{DefaultImagePath}\"".to_string() }
fn default_mpv_kill_command() -> String { "taskkill /IM mpv.exe /F".to_string() }
fn default_mpv_test_command() -> String { "echo test > {IPCChannel}".to_string() }
fn default_im_convert_command() -> String { "\"{IMPath}\" \"{ImgPath}\" -resize {MarqueeWidth}x{MarqueeHeight} \"{ImgTargetPath}\"".to_string() }
fn default_host() -> String { "127.0.0.1".to_string() }
fn default_port() -> i32 { 8080 }

// Custom deserializer for optional booleans
fn deserialize_bool_from_string_opt<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => match s.to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(de::Error::unknown_variant(&s, &["true", "false"])),
        },
        None => Ok(false),
    }
}
