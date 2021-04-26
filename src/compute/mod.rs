use crossbeam::channel::{Receiver, Sender};
use log::{debug, error, info};

use crate::{data::Data, msg::Signal};

pub fn computer(
    input: Receiver<Data>,
    output: Sender<Data>,
    cell_0: Data,
    neighbor_0: Data,
    signal: Sender<Signal>,
) {
    let mut out_data = f(1, &cell_0, &neighbor_0);
    for frame in 1..=5 {
        if frame > 1 {
            match input.recv() {
                Ok(neighbor) => {
                    info!("Neighbor: {:?}", neighbor);
                    out_data = f(frame, &out_data, &neighbor);
                }
                Err(msg) => {
                    panic!("Failed to read from channel: {}", msg)
                }
            }
        }

        // Sleep to fake doing work!
        std::thread::sleep(std::time::Duration::from_millis(1000));

        info!("My State: {:?}", out_data);
        match output.send(out_data.clone()) {
            Ok(_) => debug!("Wrote data to output channel"),
            Err(msg) => error!("Failed to send data to output channel: {}", msg),
        }
    }

    info!("Final Value is {:?}", out_data);
    info!("Work complete.  Beginning Shutdown");
    signal.send(Signal::Stop).unwrap();
    signal.send(Signal::Stop).unwrap();
}

pub fn f(frame: u32, cell: &Data, neighbor: &Data) -> Data {
    assert!(cell.frame == neighbor.frame);
    assert!(cell.frame == frame - 1);

    Data {
        frame,
        v: cell
            .v
            .iter()
            .zip(neighbor.v.iter())
            .map(|(c, n)| c + n / 4.)
            .collect(),
    }
}
