use std::{sync::mpsc::{channel, Sender, Receiver}, fmt};

pub struct DualChannel {
    pub timekeeper_channels: (Sender<Operations>, Receiver<Operations>),
    pub gui_channels: (Sender<Operations>, Receiver<Operations>),
}

impl DualChannel {
    pub fn new() -> Self {
        /*  Create channel using ResetTimeiple producer single consumer in a tuple
            with tx being transmitter, rx being receiver
            https://doc.rust-lang.org/book/ch16-02-message-passing.html */
        let (timekeeper_out, gui_in) = channel();
        let (gui_out, timekeeper_in) = channel();
        DualChannel {
            timekeeper_channels: (timekeeper_out, timekeeper_in),
            gui_channels: (gui_out, gui_in),
        }
    }
}

pub enum Operations {
    StartTime = 0x01,
    StopTime,
    ResetTime,
    SetRate,
    Alive,
    Shutdown,
    Failure = 0xff,
}

impl Operations {
    pub fn from_u8(val: u8) -> Operations {
        match val {
            0x01 => return Operations::StartTime,
            0x02 => return Operations::StopTime,
            0x03 => return Operations::ResetTime,
            0x04 => return Operations::SetRate,
            0x05 => return Operations::Alive,
            0x06 => return Operations::Shutdown,
            _ => return Operations::Failure,
        }
    }
}

impl fmt::Debug for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operations::StartTime => write!(f, "StartTime"),
            Operations::StopTime => write!(f, "StopTime"),
            Operations::ResetTime => write!(f, "ResetTime"),
            Operations::SetRate => write!(f, "SetRate"),
            Operations::Alive => write!(f, "Alive"),
            Operations::Shutdown => write!(f, "Shutdown"),
            Operations::Failure => write!(f, "Failure"),
        }
    }
}