use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub mod auth;
pub mod db;
pub mod error;
pub mod request;
pub mod settings;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
