use clap::Parser;
use tattoo_lib::{*};

#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]

// todo!() need to research into making an option be either true or false, if true contain a value
struct Args {
    #[arg(short, long, default_value_t=false)]
    serial_number: bool,

    #[arg(long, default_value_t=false)]
    make: bool,

    #[arg(long, default_value_t=false)]
    model: bool,

    #[arg(short, long, default_value_t=false)]
    asset_tag: bool,

    #[arg(short, long, default_value_t=false)]
    update: bool,
}

fn main() {
    let args = Args::parse();
    //println!("{:?}", args); // This is a nice way to debug when adding new commands
    
    // we want to update first, before printing out device information this is currently a todo!()
    if args.update { println!("Update is true, update information automatically using tattoo features."); }
    if args.make { println!("Make: {}", device::make::get()); }
    if args.model { println!("Model: {}", device::model::get()); }
    if args.serial_number { println!("Serial Number: {}", device::serialnumber::get()); }
    if args.asset_tag { println!("Asset Tag: {}", registry::assettag::get()); }
    
    // This is our default behavoiur, what will we do. Basically, since all values are default we will take the inverse of that. If any one value is set to true, then it will become false and this will nto run.
    if !(args.serial_number || args.make || args.model || args.asset_tag || args.asset_tag) { println!("Make: {}\nModel: {}\nSerial Number: {}\nAsset Tag: {}", device::make::get(), device::model::get(), device::serialnumber::get(), registry::assettag::get()); }
}
