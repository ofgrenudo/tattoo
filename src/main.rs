mod device;
mod ui;

fn main() {
    let mut asset_tag = "";
    let mut serial_number = device::serialnumber::get();
    let mut manufacturer = device::make::get();
    let mut model = device::model::get();

    // 1. Check registry for stored data.

    // 2. If registry does not have stored data, gather data and present in form.

    // 3. Present form, on confirmation submit data to registry.
    let app = ui::start(
        Some(asset_tag.to_string()),
        Some(serial_number),
        Some(manufacturer),
        Some(model)
    );
}