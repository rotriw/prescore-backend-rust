use diesel::{pg::PgConnection, Connection};

pub static mut DBCONN: Option<PgConnection> = None;

pub fn start(dbpath: &str) {
    unsafe {
        DBCONN = Some(PgConnection::establish(&dbpath).unwrap_or_else(|_| panic!("Error connecting to {}", dbpath)));
    }
    
}
