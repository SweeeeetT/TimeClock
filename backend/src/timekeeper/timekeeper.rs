extern crate stopwatch;
use std::sync::mpsc::{Sender, Receiver};

use stopwatch::{Stopwatch};

pub struct TimeKeeper {
    pub stopwatch: Stopwatch,
    pub salary: u32,
    pub comms: (Sender<comms::Operations>, Receiver<comms::Operations>),
}

impl TimeKeeper {
    pub fn new((timekeeper_out, timekeeper_in): (Sender<comms::Operations>, Receiver<comms::Operations>)) -> Self {
        TimeKeeper {
            stopwatch: Stopwatch::new(),
            //TODO: read from persistent file for default
            salary: 61,
            comms: (timekeeper_out, timekeeper_in),
        }
    }
    pub fn run(&self)
    {
        //Placeholder
    }
}
