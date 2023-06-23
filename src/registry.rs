use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn tattoo_exists() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let registry_exists = hklm.open_subkey("SOFTWARE\\Tattoo");
    
    match registry_exists {  
        Ok(repository) => {
            let pf: String = repository.get_value("damn_daniel")?;
            println!("{}", pf);
        },
        Err(error) => initialize(),
    }

    Ok(())
}

fn initialize() {
    println!("Could not find registry keys... Initializing now.");
    

}