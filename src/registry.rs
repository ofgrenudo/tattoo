pub mod assettag;
pub mod make;
pub mod model;
pub mod serialnumber;

use winreg::enums::*;
use winreg::RegKey;

pub fn check_exists() -> Result<(), ()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let registry_exists = hklm.open_subkey("SOFTWARE\\Tattoo");

    match registry_exists {
        Ok(_repository) => Ok(()),
        Err(_error) => Err(()),
    }
}
