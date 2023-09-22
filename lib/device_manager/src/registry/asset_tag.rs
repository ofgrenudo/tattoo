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

/// This function will return the `asset_tag` value stored in the registry to you as a result (possible Ok(String) or Err(RegistryOutcomes)).
/// 
/// ## Example Usage
/// ```rust
/// use device_manager::registry;
/// 
/// fn main() {
///     // Get Asset Tag.
///     let possible_asset_tag = device_manager::registry::asset_tag::get();
/// 
///     if possible_asset_tag.is_err() { println!("{}", possible_asset_tag.unwrap())}
///     else { println!("{}", possible_asset_tag.unwrap()); }
///
/// }
/// ```
pub fn get() -> Result<String, RegistryOutcomes> {
    let key_name = "asset_tag".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let provided_sub_key = "SOFTWARE\\Tattoo".to_string();
    let current_version = hklm.open_subkey(provided_sub_key);
    let _key = hklm.create_subkey("SOFTWARE\\Tattoo".to_string()).unwrap();

    // What happens if we derail?
    if current_version.is_err() { return Err(RegistryOutcomes::CannotReadRegistry); }    
    
    // All looks good. Proceed forward
    let asset_tag = current_version.unwrap().get_value(key_name).unwrap_or("Null".to_string());
    Ok(asset_tag)
}

#[test]
fn test_get() {
     // Get Asset Tag.
     let possible_asset_tag = get();
    assert!(possible_asset_tag.is_ok());
}

/// This function takes a `String` as an input and will use it to set the value in the registry. It will return to you,
/// one of many Enum Types declared in the `RegistryOutcomes` Enum.
/// 
/// ## Example Usage
/// ```rust
/// use device_manager::registry;
/// 
/// fn main() {
///     let old_asset_tag = device_manager::registry::asset_tag::get().unwrap();///  
///     println!("{}", old_asset_tag);
/// 
///     println!("Updating Asset Tag Value to 12345");
///     let asset_tag_updated = device_manager::registry::asset_tag::set("12345".to_string());
///     
///     if asset_tag_updated.is_ok() { println!("The Asset Tag Value in the registry has been successfully updated!"); }
///     else { println!("There was an issue updating the Asset Tag Value in the registry. Please ensure that you are runing the program as an administrator."); }
/// 
///     println!("Asset Tag Value has been updated to {}", device_manager::registry::asset_tag::get().unwrap());
/// }
/// ```
pub fn set(input: String) -> Result<RegistryOutcomes, RegistryOutcomes> {
    let key_name = "asset_tag".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let provided_sub_key = "SOFTWARE\\Tattoo".to_string();
    let current_version = hklm.open_subkey(provided_sub_key);   
    let _key = hklm.create_subkey("SOFTWARE\\Tattoo".to_string()).unwrap();
    
    // What happens if we derail?
    if current_version.is_err() { return Err(RegistryOutcomes::CannotReadRegistry); }// Could not read registry. Run as admin! 
    let hklm_path = Path::new("Software").join("Tattoo");

    // This will bassicaly, confirm that the key exists. If its not there it creates it. If it is then it returns an operator.
    let attempt_create = hklm.create_subkey(&hklm_path);
    if attempt_create.is_err() { return Err(RegistryOutcomes::CouldNotCreateSubPath); }

    let (key, _disp) = attempt_create.unwrap();

    // Now we will delete the key, and create a new one.
    key.delete_value(&key_name).unwrap_or(());
    let key_created = key.set_value(&key_name, &input);
    if key_created.is_err() { return Err(RegistryOutcomes::CouldNotCreateKey); }

    Ok(RegistryOutcomes::KeyCreated)
}