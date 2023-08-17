use clap::Parser;
use tattoo_lib::{device, registry};

// todo!() need to research into making an option be either true or false, if true contain a value
#[derive(Parser, Debug)]
#[command(author = env!("CARGO_PKG_AUTHORS"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[arg(long, help = "Returns the computer make", default_value_t = false)]
    make: bool,

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
        registry::make::set(device::make::get());
        registry::model::set(device::model::get());
        registry::serialnumber::set(device::serialnumber::get());
    }
    if args.make {
        println!("Make: {}", device::make::get());
    }
    if args.model {
        println!("Model: {}", device::model::get());
    }
    if args.serial_number {
        println!("Serial Number: {}", device::serialnumber::get());
    }
    if args.get_asset_tag {
        println!("Asset Tag: {}", registry::assettag::get());
    }

    if args.asset_tag.is_some() {
        registry::assettag::set(args.asset_tag.unwrap_or("".to_string()));
    }

    // This is our default behavoiur, what will we do. Basically, since all values are default we will take the inverse of that.
    // If any one value is set to true, then it will become false and this will into run.
    if !(args.serial_number || args.make || args.model || args.get_asset_tag) {
        println!(
            "Make: {}\nModel: {}\nSerial Number: {}\nAsset Tag: {}",
            device::make::get(),
            device::model::get(),
            device::serialnumber::get(),
            registry::assettag::get()
        );
    }
}
