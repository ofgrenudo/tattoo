//!
//! The registry API contains the following sections
//! 
//! - asset_tag
//! - manufacturer
//! - model
//! - serial_number
//! 
//! Each of these sections will contain user submitted information. This information can be changed by anyone with administrative rights to the computer, so should be taken as a grain of salt and confirmed when in question.
//! 
//! ## Example Usage
//! 
//! ```rust
//! use device_manager::registry;
//! 
//! fn main() {
//!     println!("Hello, world!");
//!     let manufacturer = registry::manufacturer::get().unwrap();
//!     let make = registry::model::get().unwrap();
//!     let serial_number = registry::serial_number::get().unwrap();
//!     let asset_tag = registry::asset_tag::get().unwrap();
//!     println!("This program is running on a {} {}, with a Serial Number of {}, and asset tag of {}",
//!         manufacturer, make, serial_number, asset_tag)
//! }
//! ```

/// Gather and set device Asset Tag in the registry.
pub mod asset_tag;
/// Gather and set device Manufacturer in the registry.
pub mod manufacturer;
/// Gather and set device Model in the registry.
pub mod model;
/// Gather and set device Serial Number in the registry.
pub mod serial_number;