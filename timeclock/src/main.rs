use std::thread;
use gui::windows::windows;
use backend::timekeeper::timekeeper;

fn main() {

    let timekeeper = thread::spawn(|| {
        timekeeper::init();
    });
    let gui_window = thread::spawn(|| {
        windows::gui_main();
    });

    gui_window.join().unwrap();
    timekeeper.join().unwrap();
}