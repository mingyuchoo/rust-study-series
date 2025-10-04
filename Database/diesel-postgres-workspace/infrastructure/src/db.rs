use diesel::pg::PgConnection;
use crate::{establish_connection, run_migrations_and_seed};

pub struct DbProvider;

impl DbProvider {
    pub fn establish() -> PgConnection {
        // retry up to 10 times with backoff
        let mut last_err: Option<String> = None;
        for i in 0..10 {
            match std::panic::catch_unwind(|| establish_connection()) {
                Ok(conn) => return conn,
                Err(_) => {
                    last_err = Some("panic while establishing connection".to_string());
                }
            }
            let sleep_ms = 500 * (i + 1);
            eprintln!("DB not ready yet, retrying in {}ms (attempt {}/10)...", sleep_ms, i + 1);
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms as u64));
        }
        panic!("Failed to establish DB connection: {:?}", last_err);
    }
    pub fn migrate_and_seed(conn: &mut PgConnection) {
        // run migrations with a couple retries
        for i in 0..3 {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_migrations_and_seed(conn)
            }));
            match result {
                Ok(_) => return,
                Err(_) => {
                    let sleep_ms = 500 * (i + 1);
                    eprintln!("Migration failed, retrying in {}ms (attempt {}/3)...", sleep_ms, i + 1);
                    std::thread::sleep(std::time::Duration::from_millis(sleep_ms as u64));
                }
            }
        }
        // final try (will panic inside run_migrations_and_seed with its own message)
        run_migrations_and_seed(conn)
    }
}
