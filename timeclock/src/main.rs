use std::thread;
use gui::windows::windows;
use backend::timekeeper::{timekeeper, comms};


fn main() {

    let channels = comms::DualChannel::new();
    let timekeeper = timekeeper::TimeKeeper::new(channels.timekeeper_channels);
    let gui = windows::GUI::new(channels.gui_channels);

    let timekeeper_thread = thread::spawn(move|| {
        timekeeper.run();
    });
    let gui_windows_thread = thread::spawn(move|| {
        gui.run();
    });

    gui_windows_thread.join().unwrap();
    timekeeper_thread.join().unwrap();
}