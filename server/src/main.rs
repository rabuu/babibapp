use std::env;

use anyhow::anyhow;

use babibapp::config::Config;

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);

    let config_path = args
        .next()
        .ok_or(anyhow!("Expected argument `config_path`, but got nothing"))?;

    let config = Config::from_toml(&config_path).unwrap();
    println!("{:#?}", config);

    Ok(())
}
