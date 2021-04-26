use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub frame: u32,
    pub v: Vec<f32>,
}

impl Data {
    pub fn new(v: &Vec<f32>) -> Data {
        Data {
            frame: 0,
            v: v.clone(),
        }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}
