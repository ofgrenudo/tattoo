use clap::Parser;
//use tattoo_lib::{device, registry};
use device_manager::{registry, system};

#[derive(Parser, Debug)]
#[command(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
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

fn main() {
    let args = Args::parse();
    //println!("{:?}", args); // This is a nice way to debug when adding new commands

    if args.update {
        let _initalize_registry_folder = registry::manufacturer::set(system::manufacturer::get().unwrap());
        
        let _ = registry::manufacturer::set(system::manufacturer::get().unwrap());
        let _ = registry::model::set(system::model::get().unwrap());
        let _ = registry::serial_number::set(system::serial_number::get().unwrap());
    }
    if args.manufacturer {
        println!("manufacturer: {}", system::manufacturer::get().unwrap());
    }
    if args.model {
        println!("Model: {}", system::model::get().unwrap());
    }
    if args.serial_number {
        println!("Serial Number: {}", system::serial_number::get().unwrap());
    }
    if args.get_asset_tag {
        println!("Asset Tag: {}", registry::asset_tag::get().unwrap());
    }

    if args.asset_tag.is_some() {
        let _ = registry::asset_tag::set(args.asset_tag.unwrap_or("".to_string()));
    }

    // This is our default behavoiur, what will we do. Basically, since all values are default we will take the inverse of that.
    // If any one value is set to true, then it will become false and this will into run.
    if !(args.serial_number || args.manufacturer || args.model || args.get_asset_tag) {
        println!(
            "manufacturer: {}\nModel: {}\nSerial Number: {}\nAsset Tag: {}",
            system::manufacturer::get().unwrap(),
            system::model::get().unwrap(),
            system::serial_number::get().unwrap(),
            registry::asset_tag::get().unwrap_or("Null".to_string())
        );
    }
}
