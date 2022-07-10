extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use backend::{timekeeper::timekeeper, comms::comms};
use std::sync::mpsc::{Sender, Receiver};

#[derive(Default, NwgUi)]
pub struct WinGUI {
    #[nwg_control(size: (400, 200), position: (300, 300), title: "Time Clock", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [WinGUI::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Rate", size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,



    #[nwg_control(text: "Start Clock", size: (280, 25), position: (10, 40))]
    #[nwg_events( OnButtonClick: [WinGUI::toggle_clock] )]
    hello_button: nwg::Button,

    #[nwg_control(text: "", size: (280, 20), position: (10, 100), readonly: true)]
    clock: nwg::TextInput
}

impl WinGUI {

    fn toggle_clock(&self) {
        // nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
        if self.hello_button.text() == "Start Clock" {
            self.hello_button.set_text("Stop Clock");
        } else {
            self.hello_button.set_text("Start Clock");
        }
    }

    fn say_goodbye(&self) {
        // nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}


pub fn gui_run((gui_out, gui_in): (Sender<comms::Operations>, Receiver<comms::Operations>)) {
    nwg::init().expect("Failed to init Native Windows GUI");

    let comms = (gui_out, gui_in);

    let _app = WinGUI::build_ui(Default::default()).expect("Failed to build UI");
    gui_out.send(comms::Operations::Alive).unwrap();

    nwg::dispatch_thread_events();
}