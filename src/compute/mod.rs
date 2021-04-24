use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};

use crate::data::Data;

pub fn computer(input: Receiver<Data>, output: Sender<Data>) {
    loop {
        match input.recv() {
            Ok(data) => {
                info!("Input: {:?}", data);
                let result = data.v.iter().map(|i| i * 2.).collect();
                let out_data = Data::new(&result);

                // Sleep to fake doing work!
                std::thread::sleep(std::time::Duration::from_millis(1000));

                debug!("Output: {:?}", out_data);
                match output.send(out_data) {
                    Ok(_) => debug!("Wrote data to output channel"),
                    Err(msg) => error!("Failed to send data to output channel: {}", msg),
                }
            }
            Err(msg) => {
                error!("Failed to read from channel: {}", msg);
            }
        }
    }
}
