use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager, CustomizeConnection};
use diesel::RunQueryDsl;
use diesel::sql_query;
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct ConnectionOptions;

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            sql_query("PRAGMA journal_mode=WAL").execute(conn)?;
            sql_query("PRAGMA synchronous=NORMAL").execute(conn)?;
            Ok(())
        })().map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mercury.db".to_string());
    
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder()
        .connection_customizer(Box::new(ConnectionOptions))
        .build(manager)
        .expect("Failed to create pool")
}