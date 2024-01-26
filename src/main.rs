use device_manager::{registry, system};
slint::include_modules!();
use std::io::prelude::*;
use clap::Parser;
use std::io;
mod rights;


// Note, im attempting to keep this in alphabetical order for the most part.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    
    #[arg(short='a', long,
    help = "Returns all device information.")]
    all: bool,

    #[arg(short='t', long,
    help = "Returns the device asset tag.")]
    asset_tag: bool,

    #[arg(short='T', long, default_value = "None",
    help = "Inserts the device asset tag provided.")]
    set_asset_tag: String,

    #[arg(short='M', long,
    help = "Returns the device manufacturer.")]
    manufacturer: bool,
    
    #[arg(short='m', long,
    help = "Returns the device model.")]
    model: bool,

    #[arg(short='n', long,
    help = "Returns the device serial number.")]
    serial_number: bool,    

    #[arg(short='s', long,
    help = "Returns the device status.")]
    status: bool,
    
    #[arg(short='S', long, default_value = "None",
    help = "Inserts the device status with the string provided.")]
    set_status: String,

    #[arg(short='u', long,
    help = "Inserts all device information into the registry.")]
    update: bool,    
}


// !TODO: Eventually, Id like for a slint notification window to pop up when we use this instead of the terminal.
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();
    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}


fn main() -> Result<(), slint::PlatformError> {
    // Check if the application is running as an administrator.
    if !rights::is_app_elevated() { 
        println!("Please run this application as an administrator!");
        pause();
        std::process::exit(5); // This Error Code is in accordance to the Windows Error Code 5 (ERROR_ACCESS_DENIED)
    }

    let cli = Cli::parse();
    // println!("{:?}", cli); // A great way to debug when adding new parameters.
    
    // process --all
    if cli.all {
        println!(
            "Manufacturer: {}\nModel: {}\nSerial Number: {}\nStatus: {}\nAsset Tag: {}",
            system::manufacturer::get().unwrap_or("Null".to_string()),
            system::model::get().unwrap_or("Null".to_string()),
            system::serial_number::get().unwrap_or("Null".to_string()),
            registry::status::get().unwrap_or("Null".to_string()),
            registry::asset_tag::get().unwrap_or("Null".to_string())
        );
        std::process::exit(0);
    } 
    // process --asset-tag
    if cli.asset_tag { println!("{}", registry::asset_tag::get().unwrap_or("Null".to_string()))}
    // process --set-asset-tag
    if cli.set_asset_tag != "None".to_string() { 
        let _ = registry::asset_tag::set(cli.set_asset_tag).unwrap();
        std::process::exit(0);    
    }
    // process --manufacturer
    if cli.manufacturer { println!("{}", system::manufacturer::get().unwrap_or("Null".to_string()))}
    // process --model
    if cli.model { println!("{}", system::model::get().unwrap_or("Null".to_string()))}
    // process --serial_number
    if cli.serial_number { println!("{}", system::serial_number::get().unwrap_or("Null".to_string()))}
    // process --status
    if cli.status { println!("{}", registry::status::get().unwrap_or("Null".to_string()))}
    // process --set-status
    if cli.set_status != "None" { 
        let _ = registry::status::set(cli.set_status).unwrap();
        std::process::exit(0);
    }
    // process --update
    if cli.update {
        let _initalize_registry_folder = registry::manufacturer::set(system::manufacturer::get().unwrap());
        let _ = registry::manufacturer::set(system::manufacturer::get().unwrap());
        let _ = registry::model::set(system::model::get().unwrap());
        let _ = registry::serial_number::set(system::serial_number::get().unwrap());
        std::process::exit(0);
    } 

    // process no options, just program ? 
    if !(cli.asset_tag || cli.manufacturer || cli.model || cli.serial_number || cli.status || cli.update) {
        // Loadup UI
        let ui = AppWindow::new()?;
        // Set Global Variables for Slint
        ui.global::<SystemInformation>().set_manufacturer(system::manufacturer::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_model(system::model::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_serial_number(system::serial_number::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_status(registry::status::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_asset_tag(registry::asset_tag::get().unwrap_or("Null".to_string()).into());
        // Actually show the window.
        return ui.run();
    }
    std::process::exit(0)
}
