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
    let query = &args[1];

    match query.as_str() {
        "--update" => println!("Updating the database."),
        "--all" => println!("Printing all device information."),
        "--assettag" => println!("Printing asset tag."),
        "--serialnumber" => println!("Printing serial number."),
        "--make" => println!("Printing device's manufacturer."),
        "--model" => println!("Printing device's model."),
        "--help" => println!("Displaying help menu..."),
        _ => println!("Unable to find command."),
    }
}