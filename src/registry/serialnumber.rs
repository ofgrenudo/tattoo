use winreg::enums::*;
use winreg::RegKey;

/// This function returns a string. It works by contacting a predefined registry path `HKLM:\Software\Tattoo\serial_number`
/// 
/// # Examples
/// ```rust
/// use tattoo_lib::registry;
/// 
/// let serial_number: String = registry::serialnumber::get();
/// ```
/// 
pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let serial_number = cur_ver.get_value("serial_number").unwrap_or("".to_string());

    serial_number
}

pub fn set() -> String {
    "".to_string()
}
