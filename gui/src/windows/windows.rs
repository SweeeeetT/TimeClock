extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use backend::timekeeper::{timekeeper, comms};
use std::sync::mpsc::{Sender, Receiver};

#[derive(Default, NwgUi)]
pub struct WinGUI {
    #[nwg_control(size: (400, 200), position: (300, 300), title: "Time Clock", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [WinGUI::kill_thread] )]
    window: nwg::Window,

    #[nwg_control(text: "Rate", size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Start Clock", size: (280, 25), position: (10, 40))]
    #[nwg_events( OnButtonClick: [WinGUI::toggle_clock] )]
    hello_button: nwg::Button,

    #[nwg_control(text: "", size: (280, 20), position: (10, 100), readonly: true)]
    clock: nwg::TextInput,
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

    fn notify_alive(&self) {
        nwg::simple_message("Alive!", "IT'S ALIVE!!");
    }

    fn kill_thread(&self) {
        nwg::stop_thread_dispatch();
    }

    fn check_response(expected: comms::Message, got: comms:Message) {

    }

}


pub fn gui_run((gui_out, gui_in): (Sender<comms::Message>, Receiver<comms::Message>)) {
    nwg::init().expect("Failed to init Native Windows GUI");

    let app = WinGUI::build_ui(Default::default()).expect("Failed to build UI");
    let comms = comms::SingleChannel::new((gui_out, gui_in));
    comms.send_alive();
    let mut status = comms::Operations::Alive;

    while status != comms::Operations::Shutdown {
        let request = self.comms.sc_in.recv().unwrap();
        status = request.op;
        match status {
            comms::Operations::StartTime=>self.stopwatch.start(),
            comms::Operations::StopTime=>self.stopwatch.stop(),
            comms::Operations::ResetTime=>self.stopwatch.reset(),
            comms::Operations::SetRate=>self.rate = request.data,
            comms::Operations::Alive=>self.comms.send_alive(),
            comms::Operations::Shutdown=>self.shutdown(),
            comms::Operations::Failure=>self.kill_thread(),
            _=>self.kill_thread(),
        }
    }    

    nwg::dispatch_thread_events();
}