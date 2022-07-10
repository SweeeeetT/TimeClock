use std::thread;
use gui::windows::windows;
use backend::{timekeeper::timekeeper, comms::comms};


fn main() {

    let channels = comms::DualChannel::new();
    
    let timekeeper_thread = thread::spawn(|| {
        let timekeeper = timekeeper::new(channels.timekeeper_channels);
    });
    let gui_windows_thread = thread::spawn(|| {
        windows::gui_run(channels.gui_channels);
    });

    gui_windows_thread.join().unwrap();
    timekeeper_thread.join().unwrap();
}