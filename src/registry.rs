pub mod serialnumber;
pub mod assettag;
pub mod make;
pub mod model;

use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn check_exists() -> Result<(), ()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let registry_exists = hklm.open_subkey("SOFTWARE\\Tattoo");

    match registry_exists {  
        Ok(repository) => {
            Ok(())
        },
        Err(error) => Err(()),
    }
}