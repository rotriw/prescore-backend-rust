
// use std::thread;

use serde_json::ser::Formatter;

#[macro_export]
macro_rules! Json {
    () => {
        serde_json::json!({}).to_string()
    };

    ($($json:tt)+) => {
        serde_json::json!({$($json)+}).to_string()
    };
}


pub struct FixedPrecisionFloat;

impl Formatter for FixedPrecisionFloat {
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> std::io::Result<()>
    where
        W: ?Sized + std::io::Write,
    {
        writer.write_all(format!("{:.2}", value).as_bytes())
    }
}

#[macro_export]
macro_rules! JsonWithFloat {
    ($($json:tt)+) => {{
        let data = serde_json::json!({$($json)+});
        let mut out = vec![];
        let mut ser = serde_json::ser::Serializer::with_formatter(
            &mut out, 
            crate::FixedPrecisionFloat
        );
        use serde::Serialize;
        data.serialize(&mut ser).unwrap();
        String::from_utf8(out).unwrap()
    }};
}
// Default Values
const DEFAULT_PORT: u16 = 8060;
const DEFAULT_DB_SERVER: &'static str = "postgres://localhost:5432/prescore";
pub const DEFAULT_ZHIXUE_LINK: &'static str = "https://www.zhixue.com/";
pub const DEFAULT_FONTPATH: &str = "/System/Library/Fonts";

pub static mut FONTPATH: Option<String> = None;

// require mods.
include!("./require.rs");

fn main() {
    //load config
    let config = service::config::load("config.json");
    unsafe { 
        FONTPATH = Some(String::from(config["fontpath"].as_str().unwrap_or("")));
    }
    // start service
    service::apply(config["dblink"].as_str().unwrap_or(DEFAULT_DB_SERVER));
    // start handler.
    let _ = handle::main(config["port"].as_str().unwrap_or("").parse::<u16>().unwrap_or(DEFAULT_PORT));
}
