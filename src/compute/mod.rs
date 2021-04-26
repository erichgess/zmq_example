use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};

use crate::data::Data;

pub fn computer(input: Receiver<Data>, output: Sender<Data>, cell_0: Data, neighbor_0: Data) {
    let mut out_data = f(&cell_0, &neighbor_0);
    for frame in 0..5 {
        if frame > 0 {
            match input.recv() {
                Ok(neighbor) => {
                    info!("Input: {:?}", neighbor);
                    out_data = f(&out_data, &neighbor);
                }
                Err(msg) => {
                    panic!("Failed to read from channel: {}", msg)
                }
            }
        }

        // Sleep to fake doing work!
        std::thread::sleep(std::time::Duration::from_millis(1000));

        debug!("Output: {:?}", out_data);
        match output.send(out_data.clone()) {
            Ok(_) => debug!("Wrote data to output channel"),
            Err(msg) => error!("Failed to send data to output channel: {}", msg),
        }
    }
}

pub fn f(cell: &Data, neighbor: &Data) -> Data {
    Data {
        v: cell
            .v
            .iter()
            .zip(neighbor.v.iter())
            .map(|(c, n)| c + n / 4.)
            .collect(),
    }
}
