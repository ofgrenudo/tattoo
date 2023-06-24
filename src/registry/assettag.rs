use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

pub fn get() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
    let path = Path::new("Tattoo").join("example_insert");

    hklm.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => panic!("Key doesn't exist"),
        io::ErrorKind::PermissionDenied => panic!("Access denied"),
        _ => panic!("{:?}", e),
    });

    Ok(())
}

pub fn set() -> &'static str {
    ""
}