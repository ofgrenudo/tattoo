use std::process::Command;

/// This function returns a string. It works by contacting parsing a wmic command containing the model registered in the devices bios.
/// 
/// # Examples
/// ```rust
/// use tattoo_lib::device;
/// 
/// let model: String = device::model::get();
/// ```
/// 
pub fn get() -> String {
    let output = Command::new("cmd")
        .args(["/C", "wmic computersystem get model"])
        .output()
        .expect("failed to execute process, please ensure you are running on windows!");

    let parse_me = String::from_utf8_lossy(&output.stdout);
    let parsed = parse_me.trim();

    let parse_me = parsed;
    let parse_me = parse_me.replace("\r", "");
    // let mut parse_me = parse_me.replace(" ", "");

    let parsed: Vec<&str> = parse_me.lines().collect();
    let model = parsed[1];

    // println!("{}", model);
    model.to_owned()
}

#[test]
fn test() {
    // While this seems pointless, this will confirm with us weather or not we are on a windows device.
    assert_eq!(get(), get());
}