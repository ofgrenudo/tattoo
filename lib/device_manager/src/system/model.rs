use std::process::Command;

/// This command will return the system model using the system command `wmic computersystem get model`.
/// It will be returned in a Result<String, String> fashion. The error message will be returned to you in a detailed message. 
/// You can either return the error as information to the end user, or you can write your own custom error.
/// 
/// ## Example Usage
/// 
/// ```rust
/// use device_manager::system;
/// 
/// fn main() {
///     let model = device_manager::system::model::get();
/// 
///     // Alternatively since, we know it will always return a string, you can just `println!("{}", model.unwrap());`
///     if model.is_err() { println!("{}", model.unwrap()); }
///     else { println!("{}", model.unwrap()); }
/// }
/// ```
pub fn get() -> Result<String, String> {
    let output = Command::new("cmd")
        .args(["/C", "wmic computersystem get model"])
        .output();

    if output.is_err() { return Err("Error, Failed to execute process, please ensure you are running the program as administrator.".to_string()) }
    else {
        let output = output.unwrap();
        let parse_me = String::from_utf8_lossy(&output.stdout);
        let parsed = parse_me.trim();
    
        let parse_me = parsed;
        let parse_me = parse_me.replace("\r", "");

        let parsed: Vec<&str> = parse_me.lines().collect();
        let model = parsed[1];
            
        // println!("{}", model);
        Ok(model.to_string())    
    }
}

#[test]
pub fn test_get() {
    let model = get();
    assert!(model.is_ok());
}