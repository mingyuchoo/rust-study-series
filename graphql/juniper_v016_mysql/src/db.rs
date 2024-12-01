use mysql::Pool;

pub type DBPool = mysql::Pool;

pub fn get_db_pool() -> DBPool {
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::new(db_url).expect("Failed to create DB Pool")
}
