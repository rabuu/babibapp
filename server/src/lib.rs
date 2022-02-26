#[macro_use]
extern crate diesel;

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub mod actions;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub mod settings;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
