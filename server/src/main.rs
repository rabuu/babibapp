#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_web::{App, HttpServer};
use anyhow::anyhow;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use babibapp::error::BabibappError;
use babibapp::settings::Settings;
use babibapp::{db, routes};

embed_migrations!();

#[actix_web::main]
async fn main() -> actix_web::Result<(), BabibappError> {
    let mut args = env::args().skip(1);
    let settings_path = args.next().ok_or(anyhow!(
        "Expected argument `settings_path`, but got nothing",
    ))?;
    let settings = Settings::from_toml(&settings_path).unwrap();

    // set up database pool
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        settings.database.user,
        settings.database.password,
        settings.database.host,
        settings.database.name
    );
    let db_manager = ConnectionManager::<PgConnection>::new(&db_url);
    let db_pool = Pool::builder()
        .max_size(settings.database.pool_size)
        .build(db_manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    // run migrations
    db::blocked_access(&db_pool, |conn| {
        embedded_migrations::run(conn)?;
        Ok(()) as Result<(), BabibappError>
    })
    .await??;

    println!(
        "Starting http server at {}:{}",
        settings.http.bind, settings.http.port
    );

    let _ = HttpServer::new(|| App::new().configure(|cfg| routes::routes_config(cfg)))
        .bind((settings.http.bind, settings.http.port))?
        .run()
        .await?;
    Ok(())
}
