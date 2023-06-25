//! # Device
//! 
//! Device contains three modules to it.
//! 
//! 1. make
//! 2. model
//! 3. serialnumber
//! 
//! For more information on each individual module, please navigate to their individual page.
//! 
//! ## Example Syntax
//! 
//! ```rust
//! use tattoo_lib::device;
//! 
//! println!("{}", device::serialnumber::get());
//! ```
//!

/// Device manufacturer information
pub mod make; 
/// Device model information
pub mod model; 
/// Device serial number information
pub mod serialnumber;  
