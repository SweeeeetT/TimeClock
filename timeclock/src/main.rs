use std::thread;
use gui::windows::windows;


fn main() {

    let gui_window = thread::spawn(|| {
        windows::gui_main();
    });

    gui_window.join().unwrap();
}