#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
pub fn get_retrobat_path() -> Option<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(retrobat_key) = hklm.open_subkey("Software\\RetroBat") {
        if let Ok(path) = retrobat_key.get_value("LatestKnownInstallPath") {
            return Some(path);
        }
    }
    None
}

#[cfg(not(windows))]
pub fn get_retrobat_path() -> Option<String> {
    None
}
