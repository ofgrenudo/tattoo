use clap::Parser;
use device_manager::{registry, system};
mod rights;

slint::include_modules!();

#[derive(Parser, Debug)]
#[command(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[arg(
        long,
        help ="Returns all device information",
        default_value_t = false
    )]
    all: bool,

    #[arg(
        long,
        help = "Returns the computer manufacturer",
        default_value_t = false
    )]
    manufacturer: bool,

    #[arg(long, help = "Returns the computer model", default_value_t = false)]
    model: bool,

    #[arg(
        long,
        help = "Returns the computer serial number",
        default_value_t = false
    )]
    serial_number: bool,

    #[arg(
        long = "status",
        help = "Returns the defined status",
        default_value_t = false
    )]
    get_status: bool,

    #[arg(long = "set-status", help = "Assigns the status")]
    status: Option<String>,

    #[arg(
        long = "asset-tag",
        help = "Returns the defined asset tag",
        default_value_t = false
    )]
    get_asset_tag: bool,

    #[arg(long = "set-asset-tag", help = "Assigns the asset tag")]
    asset_tag: Option<String>,

    #[arg(
        long,
        help = "Updates hardware values into the registry (must be ran as administrator)",
        default_value_t = false
    )]
    update: bool,
}

fn main() -> Result<(), slint::PlatformError> {
    let args = Args::parse();
    // println!("{:?}", args); // This is a nice way to debug when adding new commands
    if !rights::is_app_elevated() {
        // PermissionError::new().unwrap().run().unwrap();
        println!("Please run this application as an administrator.")
    } 

    if args.update {
        let _initalize_registry_folder = registry::manufacturer::set(system::manufacturer::get().unwrap());
        
        let _ = registry::manufacturer::set(system::manufacturer::get().unwrap());
        let _ = registry::model::set(system::model::get().unwrap());
        let _ = registry::serial_number::set(system::serial_number::get().unwrap());
        std::process::exit(0)
    }
    if args.manufacturer {
        println!("{}", system::manufacturer::get().unwrap());
    }
    if args.model {
        println!("{}", system::model::get().unwrap());
    }
    if args.serial_number {
        println!("{}", system::serial_number::get().unwrap());
    }
    if args.get_asset_tag {
        println!("{}", registry::asset_tag::get().unwrap());
    }
    if args.get_status {
        println!("{}", registry::status::get().unwrap());
    }

    if args.status.is_some() {
        let _ = registry::status::set(args.status.unwrap_or("".to_string()));
        std::process::exit(0)
    }
    if args.asset_tag.is_some() {
        let _ = registry::asset_tag::set(args.asset_tag.unwrap_or("".to_string()));
        std::process::exit(0)
    }

    if args.all {
        println!(
            "Manufacturer: {}\nModel: {}\nSerial Number: {}\nStatus: {}\nAsset Tag: {}",
            system::manufacturer::get().unwrap(),
            system::model::get().unwrap(),
            system::serial_number::get().unwrap(),
            registry::status::get().unwrap_or("Null".to_string()),
            registry::asset_tag::get().unwrap_or("Null".to_string())
        );
        std::process::exit(0);
    }

    //This is our default behavoiur, what will we do. Basically, since all values are default we will take the inverse of that.
    //f any one value is set to true, then it will become false and this will into run.
    if !(args.serial_number || args.manufacturer || args.model || args.get_asset_tag || args.get_status) {
        let ui = AppWindow::new()?;
        ui.global::<SystemInformation>().set_manufacturer(system::manufacturer::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_model(system::model::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_serial_number(system::serial_number::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_status(registry::status::get().unwrap_or("Null".to_string()).into());
        ui.global::<SystemInformation>().set_asset_tag(registry::asset_tag::get().unwrap_or("Null".to_string()).into());
        return ui.run();
    }

    std::process::exit(1)
}
