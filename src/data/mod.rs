use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    v: Vec<f32>,
}

impl Data {
    pub fn new(v: &Vec<f32>) -> Data {
        Data { v: v.clone() }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}
