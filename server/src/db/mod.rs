use actix_web::web;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::error::BabibappError;

pub async fn blocked_access<F, T>(
    pool: &Pool<ConnectionManager<PgConnection>>,
    f: F,
) -> Result<T, BabibappError>
where
    F: FnOnce(&PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    let pool = pool.clone();
    let res = web::block(move || {
        let conn = pool.get()?;
        let res = (f)(&conn);
        Ok(res) as Result<T, BabibappError>
    })
    .await?;

    Ok(res)
}
