use std::process::Command;

pub fn get() -> String {
    let output = Command::new("cmd")
        .args(["/C", "wmic bios get serialnumber"])
        .output()
        .expect("failed to execute process, please ensure you are running on windows!");

    let parse_me = String::from_utf8_lossy(&output.stdout);
    let parsed = parse_me.trim();

    let parse_me = parsed;
    let parse_me = parse_me.replace("\r", ""); 
    let parse_me = parse_me.replace(" ", ""); 

    let parsed: Vec<&str> = parse_me.lines().collect();
    let serial_number = parsed[1];
    
    // println!("{}", serial_number);
    serial_number.to_owned()
}