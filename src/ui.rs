use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct TattooUI {
    // Was 300x160
    #[nwg_control(size: (300, 230), position: (300, 300), title: "Tattoo", flags: "WINDOW|VISIBLE")]
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
    commit: nwg::Button
}

impl TattooUI {

    fn confirm_input(&self) {
        nwg::simple_message("Confirmation", &format!("{}", "Tattoo was able to successfully submit the data."));
    }
    
    fn safely_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn set_asset_tag(&self, asset_tag: String) {
        self.asset_tag_edit.set_text(&asset_tag);
    }

    fn set_serial_number(&self, serial_number: String) {
        self.serial_number_edit.set_text(&serial_number);
    }

    fn set_manufacturer(&self, manufacturer: String) {
        self.manufacturer_edit.set_text(&manufacturer);
    }

    fn set_model(&self, make: String) {
        self.model_edit.set_text(&make);
    }

}

pub fn start(asset_tag: Option<String>, serial_number: Option<String>, manufacturer: Option<String>, model: Option<String>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let app: tattoo_u_i_ui::TattooUIUi = TattooUI::build_ui(Default::default()).expect("Failed to build UI");
    
    // Set Fields to Information Passed to Function
    app.set_asset_tag(asset_tag.unwrap_or("".to_string()));
    app.set_manufacturer(manufacturer.unwrap_or("".to_string()));
    app.set_serial_number(serial_number.unwrap_or("".to_string()));
    app.set_model(model.unwrap_or("".to_string()));

    nwg::dispatch_thread_events();
}