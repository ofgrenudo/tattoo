use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

/// This function returns a string. It works by contacting a predefined registry path `HKLM:\Software\Tattoo\serial_number`
///
/// # Examples
/// ```rust,ignore
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

/// This function takes a `String` as an input and will use it to set the value in the registry.
///
/// ## Example Usage
///
/// ```rust,ignore
/// use tattoo_lib::registry;
///
/// registry::serialnumber::set("fadsf78sdf6sav57a");
/// ```
///
pub fn set(key_value: String) {
    let key_name = "serial_number";

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software").join("Tattoo");
    let (key, _disp) = hklm
        .create_subkey(&path)
        .expect("Unable to create new key!");

    key.delete_value(key_name).unwrap_or(());

    // match disp {
    //     REG_CREATED_NEW_KEY => println!("A new key has been created"),
    //     REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    // }

    key.set_value(key_name, &key_value)
        .expect("Could not create key!");
}
