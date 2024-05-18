use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone)]
pub struct PaperDistribute {
    pub score: f64,
    pub sum: i32,
}