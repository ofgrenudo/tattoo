use std::process::Command;

pub fn get() -> String{
    let output = Command::new("cmd")
        .args(["/C", "wmic computersystem get manufacturer"])
        .output()
        .expect("failed to execute process");

    let parse_me = String::from_utf8_lossy(&output.stdout);
    let parsed = parse_me.trim();

    let parse_me = parsed;
    let parse_me = parse_me.replace("\r", ""); 
    // let mut parse_me = parse_me.replace(" ", ""); 

    let parsed: Vec<&str> = parse_me.lines().collect();
    let manufacturer = parsed[1];
    
    println!("{}", manufacturer);
    manufacturer.to_owned()
}