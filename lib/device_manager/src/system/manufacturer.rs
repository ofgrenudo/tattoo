use std::process::Command;

/// This command will return the system manufacturer using the system command `wmic computersystem get manufacturer`.
/// It will be returned in a Result<String, String> fashion. The error message will be returned to you in a detailed message. 
/// You can either return the error as information to the end user, or you can write your own custom error.
/// 
/// ## Example Usage
/// 
/// ```rust
/// use device_manager::system;
/// 
/// fn main() {
///     let manufacturer = device_manager::system::manufacturer::get();
/// 
///     // Alternatively since, we know it will always return a string, you can just `println!("{}", manufacturer.unwrap());`
///     if manufacturer.is_err() { println!("{}", manufacturer.unwrap()); }
///     else { println!("{}", manufacturer.unwrap()); }
/// }
/// ```
pub fn get() -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", "wmic computersystem get manufacturer"])
        .output();

    if output.is_err() { return Err("Error, Failed to execute process, please ensure you are running the program as administrator.".to_string()) }
    else {
        let output = output.unwrap();
        let parse_me = String::from_utf8_lossy(&output.stdout);
        let parsed = parse_me.trim();
    
        let parse_me = parsed;
        let parse_me = parse_me.replace("\r", "");
    
        let parsed: Vec<&str> = parse_me.lines().collect();
        let manufacturer = parsed[1];
    
        // println!("{}", manufacturer);
        Ok(manufacturer.to_string())    
    }
}

#[test]
pub fn test_get() {
    let manufacturer = get();
    assert!(manufacturer.is_ok());
}