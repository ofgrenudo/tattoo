//!
//! The system API contains the following sections
//! 
//! - manufacturer
//! - model
//! - serial_number
//! 
//! Each of these sections use the power of WMIC Querys to read system components and return their values. Below is an example of how you could use the api.
//! 
//! ## Example Usage
//! 
//! ```rust
//! use device_manager::system;
//! 
//! fn main() {
//!     println!("Hello, world!");
//!     let manufacturer = system::manufacturer::get().unwrap();
//!     let model = system::model::get().unwrap();
//!     let serial_number = system::serial_number::get().unwrap();
//!     println!("This program is running on a {} {}, with a Serial Number of {}", 
//!         manufacturer, model, serial_number)
//! }
//! ```

/// Gather device manufacturer information.
pub mod manufacturer;
/// Gather device model information.
pub mod model;
/// Gather device serial number.
pub mod serial_number;