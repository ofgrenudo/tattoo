use std::process::Command;

/// This command will return the system serial_number using the system command `wmic computersystem get serial_number`.
/// It will be returned in a Result<String, String> fashion. The error message will be returned to you in a detailed message. 
/// You can either return the error as information to the end user, or you can write your own custom error.
/// 
/// ## Example Usage
/// 
/// ```rust
/// use device_manager::system;
/// 
/// fn main() {
///     let serial_number = device_manager::system::serial_number::get();
/// 
///     // Alternatively since, we know it will always return a string, you can just `println!("{}", serial_number.unwrap());`
///     if serial_number.is_err() { println!("{}", serial_number.unwrap()); }
///     else { println!("{}", serial_number.unwrap()); }
/// }
/// ```
pub fn get() -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", "wmic bios get SerialNumber"])
        .output();

    if output.is_err() { return Err("Error, Failed to execute process, please ensure you are running the program as administrator.".to_string()) }
    else {
        let output = output.unwrap();
        let parse_me = String::from_utf8_lossy(&output.stdout);
        let parsed = parse_me.trim();
    
        let parse_me = parsed;
        let parse_me = parse_me.replace("\r", "");
        let parse_me = parse_me.replace(" ", "");
    
        let parsed: Vec<&str> = parse_me.lines().collect();
        let serial_number = parsed[1];
        
        // println!("{}", serial_number);
        Ok(serial_number.to_string())    
    }
}

#[test]
pub fn test_get() {
    let serial_number = get();
    assert!(serial_number.is_ok());
}