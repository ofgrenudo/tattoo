use winreg::enums::*;
use winreg::RegKey;

/// This function returns a string. It works by contacting a predefined registry path `HKLM:\Software\Tattoo\model`
/// 
/// # Examples
/// ```rust,ignore
/// use tattoo_lib::registry;
/// 
/// let model: String = registry::model::get();
/// ```
/// 
pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let model = cur_ver.get_value("model").unwrap_or("".to_string());

    model
}

pub fn set() -> String {
    "".to_string()
}
