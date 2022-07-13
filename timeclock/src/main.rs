use std::thread;
use gui::windows::windows;
use backend::timekeeper::{timekeeper, comms};


fn main() {

    let channels = comms::DualChannel::new();
    let timekeeper = timekeeper::TimeKeeper::new(channels.timekeeper_channels);

    let timekeeper_thread = thread::spawn(move|| {
        timekeeper.run();
    });
    let gui_windows_thread = thread::spawn(move|| {
        windows::gui_run(channels.gui_channels);
    });

    gui_windows_thread.join().unwrap();
    timekeeper_thread.join().unwrap();
}