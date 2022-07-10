extern crate stopwatch;

use stopwatch::{Stopwatch};


pub struct TimeKeeper {
    pub stopwatch: Stopwatch,
    pub salary: u32,
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper {
            stopwatch: Stopwatch::new(),
            //TODO: read from persistent file for default
            salary: 61,
        }
        
    }
}

pub fn init() {
    TimeKeeper::new();
}

// let stopwatch = Stopwatch::new();