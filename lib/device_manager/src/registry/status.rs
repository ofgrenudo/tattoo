use std::fmt;
use std::path::Path;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

/// `RegistryOutcomes` is a enum containing possible values that the program could encounter when modifying the registry.
/// 
/// ### CannotReadRegistry
/// 
/// Often this is because the application is not being ran as an administrator. Please run the application as administrator
/// and try again.
/// 
/// ### CouldNotCreateSubPath
/// 
/// Please ensure that you **One** have a valid file path. and **Two** have the appropriate permissions to modify the HKLM.
/// 
/// ### CouldNotCreateKey
/// 
/// Please check all of the above errors, and ensure that these are all true. Generally you should never run into this error unless
/// you have an invalid name, or invalid data input.
/// 
/// ### KeyCreated
/// 
/// The proccess successfully went through without any errors.
#[derive(Clone, Copy)]
pub enum RegistryOutcomes {
    CannotReadRegistry,
    CouldNotCreateSubPath,
    CouldNotCreateKey,
    KeyCreated
}

impl fmt::Debug for RegistryOutcomes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            RegistryOutcomes::CannotReadRegistry    => write!(f, "Error, Could not read registry. Please ensure you are running this application as administrator."),
            RegistryOutcomes::CouldNotCreateSubPath => write!(f, "Error, Please check that you are running the application as administrator, and that you have a valid subpath."),
            RegistryOutcomes::CouldNotCreateKey     => write!(f, "Error, Please ensure that the key name is correct, and that you have a valid value."),
            RegistryOutcomes::KeyCreated            => write!(f, "Success! The key was successfully created!"),
        }
    }
}

pub fn get() -> Result<String, RegistryOutcomes> {
    let key_name = "status".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let provided_sub_key = "SOFTWARE\\Tattoo".to_string();
    let current_version = hklm.open_subkey(provided_sub_key);
    let _key = hklm.create_subkey("SOFTWARE\\Tattoo".to_string()).unwrap();

    // What happens if we derail? 
    if current_version.is_err() { return Err(RegistryOutcomes::CannotReadRegistry); }

    // All looks good. Proceed forward.
    let status = current_version.unwrap().get_value(key_name).unwrap_or("Null".to_string());
    Ok(status)
}

pub fn set(input: String) -> Result<RegistryOutcomes, RegistryOutcomes> {
    let key_name = "status".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let provided_sub_key = "SOFTWARE\\Tattoo".to_string();
    let current_version = hklm.open_subkey(provided_sub_key);   
    let _key = hklm.create_subkey("SOFTWARE\\Tattoo".to_string()).unwrap();
    
    // What happens if we derail?
    if current_version.is_err() { return Err(RegistryOutcomes::CannotReadRegistry); }// Could not read registry. Run as admin! 
    let hklm_path = Path::new("Software").join("Tattoo");

    // This will basically, confirm that the key exists. If its not there it creates it. If it is then it returns an operator.
    let attempt_create = hklm.create_subkey(&hklm_path);
    if attempt_create.is_err() { return Err(RegistryOutcomes::CouldNotCreateSubPath); }

    let (key, _disp) = attempt_create.unwrap();

    // Now we will delete the key, and create a new one.
    key.delete_value(&key_name).unwrap_or(());
    let key_created = key.set_value(&key_name, &input);
    if key_created.is_err() { return Err(RegistryOutcomes::CouldNotCreateKey); }
    
    Ok(RegistryOutcomes::KeyCreated)
}
