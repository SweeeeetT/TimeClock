use std::{sync::mpsc::{channel, Sender, Receiver}, fmt};

#[derive(Debug)]
pub struct Message {
    pub op: Operations,
    pub data: i64,
}

impl Message {
    pub fn new(operation: Operations, dat: i64) -> Self {
        Message {
            op: operation,
            data: dat,
        }
    }
}

pub struct DualChannel {
    pub timekeeper_channels: (Sender<Message>, Receiver<Message>),
    pub gui_channels: (Sender<Message>, Receiver<Message>),
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

pub struct SingleChannel {
    pub sc_out: Sender<Message>,
    pub sc_in: Receiver<Message>,
}

impl SingleChannel {
    pub fn new((timekeeper_out, timekeeper_in): (Sender<Message>, Receiver<Message>)) -> Self {
        SingleChannel {
            sc_out: timekeeper_out,
            sc_in: timekeeper_in,            
        }
    }

    pub fn send_alive(&self) {
        let message = Message::new(Operations::Alive, 0);
        self.sc_out.send(message).unwrap();
    }
}

#[derive(PartialEq, Eq)]
pub enum Status {
    Success = 0x0,
    Failure = 0x7FFFFFFFFFFFFFFF,
}

impl Status {
    pub fn from_data_field(data: i64) -> Self {
        match data {
            0x01 => return Status::Success,
            0x7FFFFFFFFFFFFFFF => return Status::Failure,
            _ => return Status::Failure,
        }
    }
    
    pub fn to_data_field(status: Status) -> i64 {
        match status {
            Status::Success => return 0x01,
            Status::Failure => return 0x7FFFFFFFFFFFFFFF,
        }
    }

    pub fn is_success(&self) -> bool {
        return *self == Status::Success
    }
}

#[derive(PartialEq, Eq)]
pub enum Operations {
    StartTime = 0x1,
    StopTime = 0x2,
    ResetTime = 0x3,
    SetRate = 0x4,
    Alive = 0x5,
    Shutdown = 0x6,
    TimeUpdate = 0x7,
    Failure = 0xff,
}


impl Operations {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x01 => return Operations::StartTime,
            0x02 => return Operations::StopTime,
            0x03 => return Operations::ResetTime,
            0x04 => return Operations::SetRate,
            0x05 => return Operations::Alive,
            0x06 => return Operations::Shutdown,
            0x07 => return Operations::TimeUpdate,
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
            Operations::TimeUpdate => write!(f, "TimeUpdate"),
            Operations::Failure => write!(f, "Failure"),
        }
    }
}
