use serde::{de::{self, Deserializer}, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

// Custom deserializer for boolean values that might be strings
fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(&s, &["true", "false"])),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "MarqueeWidth")]
    pub marquee_width: i32,
    #[serde(rename = "MarqueeHeight")]
    pub marquee_height: i32,
    #[serde(rename = "MarqueeBorder")]
    pub marquee_border: i32,
    #[serde(rename = "MarqueeAutoConvert", deserialize_with = "deserialize_bool_from_string")]
    pub marquee_auto_convert: bool,
    #[serde(rename = "AcceptedFormats")]
    pub accepted_formats: String,
    #[serde(rename = "RetroBatPath")]
    pub retrobat_path: PathBuf,
    #[serde(rename = "RomsPath")]
    pub roms_path: PathBuf,
    #[serde(rename = "DefaultImagePath")]
    pub default_image_path: PathBuf,
    #[serde(rename = "MarqueeImagePath")]
    pub marquee_image_path: PathBuf,
    #[serde(rename = "MarqueeFilePath")]
    pub marquee_file_path: String,
    #[serde(rename = "MarqueeImagePathDefault")]
    pub marquee_image_path_default: PathBuf,
    #[serde(rename = "MarqueeFilePathDefault")]
    pub marquee_file_path_default: String,
    #[serde(rename = "MarqueeAutoScraping", deserialize_with = "deserialize_bool_from_string")]
    pub marquee_auto_scraping: bool,
    #[serde(rename = "MarqueeAutoScrapingDebug", deserialize_with = "deserialize_bool_from_string")]
    pub marquee_auto_scraping_debug: bool,
    #[serde(rename = "SystemMarqueePath")]
    pub system_marquee_path: PathBuf,
    #[serde(rename = "SystemFilePath")]
    pub system_file_path: String,
    #[serde(rename = "CollectionMarqueePath")]
    pub collection_marquee_path: PathBuf,
    #[serde(rename = "CollectionFilePath")]
    pub collection_file_path: String,
    #[serde(rename = "CollectionAlternativNames")]
    pub collection_alternativ_names: String,
    #[serde(rename = "CollectionCorrelation")]
    pub collection_correlation: String,
    #[serde(rename = "IPCChannel")]
    pub ipc_channel: String,
    #[serde(rename = "ScreenNumber")]
    pub screen_number: i32,
    #[serde(rename = "MPVPath")]
    pub mpv_path: PathBuf,
    #[serde(rename = "MPVLaunchCommand")]
    pub mpv_launch_command: String,
    #[serde(rename = "MPVKillCommand")]
    pub mpv_kill_command: String,
    #[serde(rename = "MPVTestCommand")]
    pub mpv_test_command: String,
    #[serde(rename = "IMPath")]
    pub im_path: PathBuf,
    #[serde(rename = "IMConvertCommand")]
    pub im_convert_command: String,
    pub host: String,
    pub port: i32,
    #[serde(rename = "logFile", deserialize_with = "deserialize_bool_from_string")]
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
}
