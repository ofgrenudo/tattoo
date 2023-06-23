use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::process;

#[derive(Default, NwgUi)]
pub struct TattooUI {
    #[nwg_control(size: (300, 160), position: (300, 300), title: "Tattoo", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [TattooUI::safely_exit] )]
    window: nwg::Window,

    #[nwg_control(text: "Asset Tag", size: (280, 25), position: (10, 10))]
    asset_tag_edit: nwg::TextInput,

    #[nwg_control(text: "Serial Number", size: (280, 25), position: (10, 40))]
    serial_number_edit: nwg::TextInput,

    #[nwg_control(text: "Manufacturer", size: (280, 25), position: (10, 70))]
    manufacturer_edit: nwg::TextInput,

    #[nwg_control(text: "Model", size: (280, 25), position: (10, 100))]
    model_edit: nwg::TextInput,

    #[nwg_control(text: "Commit", size: (280, 25), position: (10, 130))]
    #[nwg_events( OnButtonClick: [TattooUI::confirm_input] )]
    commit: nwg::Button
}

impl TattooUI {

    fn confirm_input(&self) {
        nwg::simple_message("Hello", &format!("Asset Tag: {}\nSerial Number: {}", self.asset_tag_edit.text(), self.serial_number_edit.text()));
    }
    
    fn safely_exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = TattooUI::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}