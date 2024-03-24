use std::fs;
use serde_json::Value;

pub fn load(path: &str) -> Value {
    let data: String = fs::read_to_string(path).unwrap().parse().unwrap();
    serde_json::from_str(&data).unwrap()
}