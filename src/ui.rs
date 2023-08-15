extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;

use tattoo_lib::device;
use tattoo_lib::registry;

#[derive(Default, NwgUi)]
#[depreciated]
pub struct TattooUI {
    #[nwg_control(size: (300, 230), position: (300, 300), title: "Tattoo")]
    #[nwg_events( OnWindowClose: [TattooUI::safely_exit] )]
    window: nwg::Window,

    #[nwg_control(text: "Asset Tag", size: (280, 25), position: (11, 5))]
    _asset_tag_label: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 22))]
    asset_tag_edit: nwg::TextInput,

    #[nwg_control(text: "Serial Number", size: (280, 25), position: (11, 52))]
    _serial_number_label: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 67))]
    serial_number_edit: nwg::TextInput,

    #[nwg_control(text: "Manufacturer", size: (280, 25), position: (10, 100))]
    manufacturer_edit_label: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 115))]
    manufacturer_edit: nwg::TextInput,

    #[nwg_control(text: "Model", size: (280, 25), position: (10, 147))]
    model_label: nwg::Label,

    #[nwg_control(text: "", size: (280, 25), position: (10, 162))]
    model_edit: nwg::TextInput,

    #[nwg_control(text: "Commit", size: (280, 25), position: (10, 195))]
    #[nwg_events( OnButtonClick: [TattooUI::confirm_input] )]
    commit: nwg::Button,
}

#[depreciated]
impl TattooUI {
    fn confirm_input(&self) {
        registry::assettag::set(self.get_asset_tag());
        registry::make::set(self.get_manufacturer());
        registry::model::set(self.get_model());
        registry::serialnumber::set(self.get_serial_number());
        
        nwg::simple_message(
            "Confirmation",
            &format!("{}", "Tattoo was able to successfully submit the data."),
        );
    }

    fn safely_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn set_asset_tag(&self, asset_tag: &str) {
        self.asset_tag_edit.set_text(&asset_tag);
    }

    fn get_asset_tag(&self) -> String {
        self.asset_tag_edit.text()
    }

    fn set_serial_number(&self, serial_number: &str) {
        self.serial_number_edit.set_text(&serial_number);
    }

    fn get_serial_number(&self) -> String {
        self.serial_number_edit.text()
    }

    fn set_manufacturer(&self, manufacturer: &str) {
        self.manufacturer_edit.set_text(&manufacturer);
    }

    fn get_manufacturer (&self) -> String {
        self.manufacturer_edit.text()
    }

    fn set_model(&self, make: &str) {
        self.model_edit.set_text(&make);
    }

    fn get_model (&self) -> String {
        self.model_edit.text()
    }

}

#[depreciated]
pub fn begin_ui() {
    let mut _asset_tag = "".to_string();
    let mut _manufacturer = "".to_string();
    let mut _serial_number = "".to_string();
    let mut _model = "".to_string();

    let registry_exists = registry::check_exists();
    // println!("{:?}", registry_exists);

    match registry_exists {
        Ok(()) => {
            _asset_tag = registry::assettag::get();
            _manufacturer = registry::make::get();
            _serial_number = registry::serialnumber::get();
            _model = registry::model::get();
        }
        Err(()) => {
            _manufacturer = device::make::get();
            _serial_number = device::serialnumber::get();
            _model = device::model::get();
        }
    }

    // Catch Null Values
    if _manufacturer == "".to_string() {
        _manufacturer = device::make::get();
    }
    if _serial_number == "".to_string() {
        _serial_number = device::serialnumber::get();
    }
    if _model == "".to_string() {
        _model = device::model::get();
    }

    // Begin Loading App
    nwg::init().expect("Failed to init Native Windows GUI");
    let app: tattoo_u_i_ui::TattooUIUi =
        TattooUI::build_ui(Default::default()).expect("Failed to build UI");

    // Set Fields to Information Passed to Function Device
    app.set_asset_tag(&_asset_tag);
    app.set_manufacturer(&_manufacturer);
    app.set_serial_number(&_serial_number);
    app.set_model(&_model);

    nwg::dispatch_thread_events();
}
