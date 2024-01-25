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

fn main() {
    let args = Args::parse();
    //println!("{:?}", args); // This is a nice way to debug when adding new commands

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

    //This is our default behavoiur, what will we do. Basically, since all values are default we will take the inverse of that.
    //f any one value is set to true, then it will become false and this will into run.
    if !(args.serial_number || args.manufacturer || args.model || args.get_asset_tag || args.get_status) {
        // println!(
        //     "Manufacturer: {}\nModel: {}\nSerial Number: {}\nStatus: {}\nAsset Tag: {}",
        //     system::manufacturer::get().unwrap(),
        //     system::model::get().unwrap(),
        //     system::serial_number::get().unwrap(),
        //     registry::status::get().unwrap_or("Null".to_string()),
        //     registry::asset_tag::get().unwrap_or("Null".to_string())
        // );
        slint::slint! {
            import { Button, VerticalBox} from "std-widgets.slint";
    
            export global SystemInformation { 
                in property <string> manufacturer;
                in property <string> model;
                in property <string> serial_number;
                in property <string> status;
                in property <string> asset_tag;
            }
    
            export component App inherits Window {
                always-on-top: true;
                default-font-size: 24px;
                title: "Tattoo";
                VerticalBox {
                    padding: 50px;
                    spacing: 12px;
    
                    HorizontalLayout {
    
                        Text { 
                            text: "Manufacturer: ";
                        }
                        
                        TextInput {
                            text: SystemInformation.manufacturer;
                            horizontal-alignment: right; 
                            read-only: true;
                        }
                    }
    
                    HorizontalLayout {
                        Text { 
                            text: "Model Name:  ";
                        }
                        
                        TextInput {
                            text: SystemInformation.model;
                            horizontal-alignment: right; 
                            read-only: true;
                        }
                    }
    
                    HorizontalLayout {
                        Text { 
                            text: "Serial Number: ";
                        }
                        
                        TextInput {
                            text: SystemInformation.serial-number;
                            horizontal-alignment: right; 
                            read-only: true;
                        }
                    }
    
                    HorizontalLayout {
                        Text { 
                            text: "Status: ";
                        }
                        
                        TextInput {
                            text: SystemInformation.status;
                            horizontal-alignment: right; 
                            read-only: true;
                        }
                    }
    
                    HorizontalLayout {
                        Text { 
                            text: "Asset Tag: ";
                        }
                        
                        TextInput {
                            text: SystemInformation.asset-tag;
                            horizontal-alignment: right; 
                            read-only: true;
                        }
                    }
                }
            }
        }
    
        let app = App::new().unwrap();
        app.global::<SystemInformation>().set_manufacturer(system::manufacturer::get().unwrap_or("Null".to_string()).into());
        app.global::<SystemInformation>().set_model(system::model::get().unwrap_or("Null".to_string()).into());
        app.global::<SystemInformation>().set_serial_number(system::serial_number::get().unwrap_or("Null".to_string()).into());
        app.global::<SystemInformation>().set_status(registry::status::get().unwrap_or("Null".to_string()).into());
        app.global::<SystemInformation>().set_asset_tag(registry::asset_tag::get().unwrap_or("Null".to_string()).into());
        // Example, Getting data.
        // println!("{}", app.global::<SystemInformation>().get_asset_tag());
        app.run().unwrap()    
    }

    std::process::exit(1)
}
