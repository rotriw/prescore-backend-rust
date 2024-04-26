use diesel::pg::PgConnection;
use diesel::r2d2;

pub static mut DBPOOL: Option<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>> = None;

pub fn start(dbpath: &str) {
    unsafe {
        let manager = r2d2::ConnectionManager::<PgConnection>::new(dbpath);
        DBPOOL = Some(r2d2::Pool::builder()
            .max_size(15)
            .build(manager)
            .expect("Failed to create pool."));
    }
    
}
