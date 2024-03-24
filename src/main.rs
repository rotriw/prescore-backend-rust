// use std::thread;


mod handler;
mod handle;
mod service;
mod model;
pub mod schema;

// Default Values
const DEFAULT_PORT: u16 = 8060;
const DEFAULT_DB_SERVER: &str = "postgres://localhost:5432/prescore";

fn main() {
    //load config
    let config = service::config::load("config.json");
    // start service
    service::apply(config["dblink"].as_str().unwrap_or(DEFAULT_DB_SERVER));
    // start handler.
    let _ = handle::main(config["port"].as_str().unwrap_or("").parse::<u16>().unwrap_or(DEFAULT_PORT));
    
}
