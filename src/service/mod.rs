pub mod config;
pub mod postgres;
pub mod zhixue;

pub fn apply(dblink: &str) {
    println!("Starting service with db: {}", dblink);
    postgres::start(dblink);
    
}