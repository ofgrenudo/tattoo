use winreg::enums::*;
use winreg::RegKey;

/// This function returns a string. It works by contacting a predefined registry path `HKLM:\Software\Tattoo\make`
/// 
/// # Examples
/// ```rust,ignore
/// use tattoo_lib::registry;
/// 
/// let make: String = registry::make::get();
/// ```
/// 
pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let manufacturer = cur_ver.get_value("manufacturer").unwrap_or("".to_string());

    manufacturer
}

pub fn set() -> String {
    "".to_string()
}
