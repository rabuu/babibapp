use actix_web::web;
use diesel::PgConnection;

use crate::error::BabibappError;
use crate::DbPool;

pub async fn blocked_access<F, T>(pool: &DbPool, f: F) -> Result<T, BabibappError>
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

    res
}
