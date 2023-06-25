use winreg::enums::*;
use winreg::RegKey;

pub fn get() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("");

    let serial_number = cur_ver.get_value("serial_number").unwrap_or("".to_string());

    serial_number
}

pub fn set() -> String {
    "".to_string()
}
