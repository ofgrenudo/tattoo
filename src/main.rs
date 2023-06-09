mod device;

use device::{make, model, serialnumber};
use std::env;

fn main() {
    /*
    * Options
    * --update
    * --all
    * --assettag [optional-i32]
    * --serialnumber
    * --make
    * --model 
    * --help
    */
    let args: Vec<String> = env::args().collect();
    let query = &args[1]; // What happens when no argument is provided? Go into console mode!

    match query.as_str() {
        "--create" => device::database::update(),
        "--update" => device::database::update(),
        "--all" => device::help::device_information(),
        "--assettag" => device::assettag::get(),
        "--serialnumber" => { let _serialnumber = device::serialnumber::get(); },
        "--make" => { let _make = device::make::get(); },
        "--model" => { let _model = device::model::get(); },
        "--help" => { device::help::show_menu(); },
        _ => { device::help::no_command_entered(); },
    }
}