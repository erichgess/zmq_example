use std::marker::PhantomData;

use crossbeam::channel::{SendError, Sender};

use crate::msg::Signal;

pub struct Stopped {
    ph: PhantomData<u8>,
}

impl Stopped {
    fn new() -> Stopped {
        Stopped { ph: PhantomData }
    }
}

pub struct SignalChan {
    ch: Sender<Signal>,
}

impl SignalChan {
    pub fn new(ch: Sender<Signal>) -> SignalChan {
        SignalChan { ch }
    }

    pub fn shutdown(&self) -> Result<Stopped, SendError<Signal>> {
        self.ch.send(Signal::Stop).map(|_| Stopped::new())
    }
}
