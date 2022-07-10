use std::thread;
use gui::windows::windows;
use backend::timekeeper::timekeeper;
use std::sync::mpsc;

fn main() {

    /* Create channel using multiple producer single consumer in a tuple
       with tx being transmitter, rx being receiver
       https://doc.rust-lang.org/book/ch16-02-message-passing.html */
    let (tx, rx) = mpsc::channel();

    let timekeeper = thread::spawn(|| {
        timekeeper::init();
    });
    let gui_window = thread::spawn(|| {
        windows::gui_main(timekeeper);
    });

    gui_window.join().unwrap();
    timekeeper.join().unwrap();
}