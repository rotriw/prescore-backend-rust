
// use std::thread;

#[macro_export]
macro_rules! Json {
    () => {
        serde_json::json!({}).to_string()
    };

    ($($json:tt)+) => {
        serde_json::json!({$($json)+}).to_string()
    };
}

// Default Values
const DEFAULT_PORT: u16 = 8060;
const DEFAULT_DB_SERVER: &'static str = "postgres://localhost:5432/prescore";
pub const DEFAULT_ZHIXUE_LINK: &'static str = "https://www.zhixue.com/";

// require mods.
include!("./require.rs");


fn main() {
    //load config
    let config = service::config::load("config.json");
    // start service
    service::apply(config["dblink"].as_str().unwrap_or(DEFAULT_DB_SERVER));
    // start handler.
   // let _ = predict("2883f02a-ccc1-4323-828c-1d03e9485f0d".to_string(), "5".to_string(), 2 as f64);
    let _ = handle::main(config["port"].as_str().unwrap_or("").parse::<u16>().unwrap_or(DEFAULT_PORT));
}
