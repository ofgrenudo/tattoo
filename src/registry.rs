//!
//! The registry contains four modules to it.
//! 
//! 1. assettag 
//! 2. make
//! 3. model
//! 4. serialnumber
//! 
//! For more information on each individual module, please navigate to their individual page.
//! 
//! > WARNING! In order to use the set features of this library, you will need to have administrative access to the system. Without it, you will no be able use these functions without a panic.
//! 
//! # Example Usage
//! 
//! ```rust,ignore
//! use tattoo_lib::registry;
//! 
//! // Gather Information
//! let asset_tag = registry::assettag::get();
//! let make = registry::make::get();
//! let model = registry::model::get();
//! let serial_number = registry::serialnumber::get();
//! 
//! println!("Asset Tag: {}\nMake: {}\nModel: {}\nSerial Number: {}", asset_tag, make, model, serial_number);
//! 
//! // NOTE: you will need administrative access to write to the registry. I cannot change this.
//! registry::assettag::set("123456789".to_string());
//! 
//! assert_eq!(registry::assettag::get().as_str(), "123456789");
//! ```
//!
pub mod assettag;
pub mod make;
pub mod model;
pub mod serialnumber;

use winreg::enums::*;
use winreg::RegKey;

/// This function ensures that the `HKLM:\Software\Tattoo` folder exists and returns either an `Ok(()), or Err(())`
/// 
/// # Example
/// 
/// ```rust
/// use tattoo_lib::registry;
/// 
/// let exists = registry::check_exists();
/// 
/// match exists {
///     Ok(()) => println!("Do Something :)"),
///     Err(()) => println!("Dont do something :)"),
/// }
/// ```
/// 
/// You could in your event, swap the inner parentheses (`Ok(>()<)`) with a value like e or registry_confirmed.
/// 
/// 
/// `Ok(registry_confirmed) => println!("{:?}", registry_confirmed)`
/// `Err(my_error) => println!("{:?}", my_error)`
/// 
pub fn check_exists() -> Result<(), ()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let registry_exists = hklm.open_subkey("SOFTWARE\\Tattoo");

    match registry_exists {
        Ok(_repository) => Ok(()),
        Err(_error) => Err(()),
    }
}
