#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_web::{middleware, web, App, HttpServer};
use anyhow::anyhow;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use babibapp::error::BabibappError;
use babibapp::settings::Settings;
use babibapp::DbPool;
use babibapp::{actions, db};
use env_logger::Env;

embed_migrations!();

#[actix_web::main]
async fn main() -> actix_web::Result<(), BabibappError> {
    let mut args = env::args().skip(1);
    let settings_path = args.next().ok_or(anyhow!(
        "Expected argument `settings_path`, but got nothing",
    ))?;
    let settings = Settings::from_toml(&settings_path).unwrap();

    // init logging
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    // set up database pool
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        settings.database.user,
        settings.database.password,
        settings.database.host,
        settings.database.name
    );
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool: DbPool = Pool::builder()
        .max_size(settings.database.pool_size)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    // run migrations
    db::blocked_access(&pool, |conn| {
        embedded_migrations::run(conn)?;
        Ok(()) as Result<(), BabibappError>
    })
    .await??;

    log::info!(
        "Starting http server at {}:{}",
        settings.http.bind,
        settings.http.port
    );

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(actions::config)
    })
    .bind((settings.http.bind, settings.http.port))?
    .run()
    .await?;

    Ok(())
}
