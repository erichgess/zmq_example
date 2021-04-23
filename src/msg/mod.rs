use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    d: Data,
}

impl Request {
    pub fn new(d: &Data) -> Request {
        Request { d: d.clone() }
    }
}
