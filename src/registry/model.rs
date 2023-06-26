use std::path::Path;
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

pub fn set(key_value: String) {
    let key_name = "model";

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let _cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo").expect("Unable to open Tattoo Directory");


    let path = Path::new("Software").join("Tattoo");
    let (key, _disp) = hklm.create_subkey(&path).expect("Unable to create new key!");
    
    key.delete_value(key_name).unwrap_or(());

    // match disp {
    //     REG_CREATED_NEW_KEY => println!("A new key has been created"),
    //     REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    // }

    key.set_value(key_name, &key_value).expect("Could not create key!");
}
