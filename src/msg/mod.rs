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

    pub fn len(&self) -> usize {
        self.d.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: Status,
}

impl Response {
    pub fn new(s: Status) -> Response {
        Response { status: s }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Good(usize),
    Bad,
}
