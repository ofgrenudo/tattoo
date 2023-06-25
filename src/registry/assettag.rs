use winreg::enums::*;
use winreg::RegKey;

/// This function returns a string. It works by contacting a predefined registry path `HKLM:\Software\Tattoo\asset_tag`
/// 
/// # Examples
/// ```rust
/// use tattoo_lib::registry;
/// 
/// let asset_tag: String = registry::assettag::get();
/// ```
/// 
pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let asset_tag = cur_ver.get_value("asset_tag").unwrap_or("".to_string());

    asset_tag
}

pub fn set() -> String {
    "".to_string()
}
