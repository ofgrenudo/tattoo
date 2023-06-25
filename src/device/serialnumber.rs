use std::process::Command;

/// This function returns a string. It works by contacting parsing a wmic command containing the serial number registered in the devices bios.
/// 
/// # Examples
/// ```rust
/// use tattoo_lib::device;
/// 
/// let serial_number: String = device::serialnumber::get();
/// ```
/// 
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

#[test]
fn test() {
    // While this seems pointless, this will confirm with us weather or not we are on a windows device.
    assert_eq!(get(), get());
}