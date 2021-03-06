extern crate openssl;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_web::{middleware, web, App, HttpServer};
use babibapp::request::RequestContext;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use babibapp::error::BabibappError;
use babibapp::settings::Settings;
use babibapp::DbPool;
use babibapp::{db, request};
use env_logger::Env;

embed_migrations!();

#[actix_web::main]
async fn main() -> actix_web::Result<(), BabibappError> {
    let mut args = env::args().skip(1);
    let settings_path = args
        .next()
        .unwrap_or("/etc/babibapp/server.toml".to_string());
    let settings = Settings::from_toml(&settings_path).expect("Loading settings file failed");

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

    let context = RequestContext {
        pool,
        settings: settings.clone(),
    };

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            .wrap(middleware::Logger::default())
            .configure(request::config)
    })
    .bind((settings.http.bind, settings.http.port))?
    .run()
    .await?;

    Ok(())
}
