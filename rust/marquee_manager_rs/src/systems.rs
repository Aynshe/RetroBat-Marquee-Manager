use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;

#[derive(Debug, Deserialize, PartialEq)]
struct System {
    name: String,
    path: String,
    theme: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SystemList {
    #[serde(rename = "system", default)]
    systems: Vec<System>,
}

pub fn load_all_systems_configs(config_directory: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut all_system_folders = HashMap::new();
    let pattern = config_directory.join("es_systems*.cfg");

    for entry in glob(pattern.to_str().unwrap())? {
        let path = entry?;
        let xml_content = fs::read_to_string(path)?;
        let system_list: SystemList = from_str(&xml_content)?;

        for system in system_list.systems {
            // Replicate Python's os.path.basename logic
            let system_path = PathBuf::from(system.path.strip_prefix("~/").unwrap_or(&system.path));
            if let Some(folder_name) = system_path.file_name().and_then(|s| s.to_str()) {
                all_system_folders.insert(system.name.clone(), folder_name.to_string());
            }
        }
    }

    Ok(all_system_folders)
}
