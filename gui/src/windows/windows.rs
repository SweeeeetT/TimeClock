extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct TCWindow {
    #[nwg_control(size: (400, 200), position: (300, 300), title: "Time Clock", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [TCWindow::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Salary", size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Start Clock", size: (280, 25), position: (10, 40))]
    #[nwg_events( OnButtonClick: [TCWindow::toggle_clock] )]
    hello_button: nwg::Button,

    // #[nwg_control(size: (280, 20), position: (10, 100))]
    // clock: nwg::GridLayout
}

impl TCWindow {

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

pub fn gui_main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = TCWindow::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}