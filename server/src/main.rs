use std::env;

use actix_web::{App, HttpServer};
use anyhow::anyhow;

use babibapp::error::BabibappError;
use babibapp::routes;
use babibapp::settings::Settings;

#[actix_web::main]
async fn main() -> actix_web::Result<(), BabibappError> {
    let mut args = env::args().skip(1);
    let settings_path = args.next().ok_or(anyhow!(
        "Expected argument `settings_path`, but got nothing",
    ))?;
    let settings = Settings::from_toml(&settings_path).unwrap();

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
