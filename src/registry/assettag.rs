use winreg::enums::*;
use winreg::RegKey;

pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let asset_tag = cur_ver.get_value("asset_tag").unwrap_or("".to_string());

    asset_tag
}

pub fn set() -> String {
    "".to_string()
}
