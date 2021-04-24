use serde::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    id: usize,
    d: Data,
}

impl Request {
    pub fn new(id: usize, d: &Data) -> Request {
        Request { id, d: d.clone() }
    }

    pub fn len(&self) -> usize {
        self.d.len()
    }

    pub fn id(&self) -> usize {
        self.id
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
