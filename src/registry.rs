use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub fn tattoo_exists() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Tattoo");
    
    match cur_ver {  
        Ok(values) => println!("{:?}", values),
        Err(error) => initialize(),
    }
}

fn initialize() {
    println!("Could not find registry keys... Initializing now.");
    

}