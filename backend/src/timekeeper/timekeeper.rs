extern crate stopwatch;
use std::sync::mpsc::{Sender, Receiver};
use stopwatch::{Stopwatch};
use super::comms;


pub struct TimeKeeper {
    pub stopwatch: Stopwatch,
    pub rate: i64,
    pub comms: comms::SingleChannel,
}

impl TimeKeeper {
    pub fn new((timekeeper_out, timekeeper_in): (Sender<comms::Message>, Receiver<comms::Message>)) -> Self {
        TimeKeeper {
            stopwatch: Stopwatch::new(),
            //TODO: read from persistent file for default
            rate: 61,
            comms: comms::SingleChannel::new((timekeeper_out, timekeeper_in)),
        }
    }

    fn shutdown(&self) {
        //placeholder
    }

    fn fail_graceful(&self) {
        //placeholder
    }

    pub fn run(mut self)
    {
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
                comms::Operations::Failure=>self.fail_graceful(),
                _=>self.fail_graceful(),
            }
        }    
    }
}
