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

}

pub fn start() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = TattooUI::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}