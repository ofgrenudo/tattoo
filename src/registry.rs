//!
//! Device contains three modules to it.
//! 
//! 1. make
//! 2. model
//! 3. serialnumber
//! 
//! For more information on each individual module, please navigate to their individual page.
//! 
//! # Example Syntax
//! 
//! ```rust
//! use tattoo_lib::device;
//! 
//! println!("{}", device::serialnumber::get());
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
