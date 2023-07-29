#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(any(feature = "pg", feature = "mysql"))]
use op_mode::OpMode;
#[cfg(any(feature = "pg", feature = "mysql"))]
use std::collections::HashMap;
#[cfg(any(feature = "pg", feature = "mysql"))]
use std::sync::Arc;
#[cfg(any(feature = "pg", feature = "mysql"))]
use tokio::sync::RwLock;

#[cfg(feature = "pg")]
pub type PoolPg = sqlx::Pool<sqlx::postgres::Postgres>;

#[cfg(feature = "mysql")]
pub type PoolMySql = sqlx::Pool<sqlx::mysql::MySql>;

#[cfg(any(feature = "pg", feature = "mysql"))]
#[derive(strum::EnumDiscriminants)]
#[strum_discriminants(name(PoolDb))]
pub enum Pool {
    #[cfg(feature = "pg")]
    Pg(PoolPg),
    #[cfg(feature = "mysql")]
    MySql(PoolMySql),
}

#[cfg(any(feature = "pg", feature = "mysql"))]
pub struct PoolDbWrapper {
    pub pool: Pool,
}

#[cfg(any(feature = "pg", feature = "mysql"))]
pub type PoolDbHolder = Arc<PoolDbWrapper>;

#[macro_export]
macro_rules! as_ref {
    ($db:ident $pool_holder:expr) => {
        if let $crate::Pool::$db(ref pool) = &(*$pool_holder).pool {
            pool
        } else {
            unreachable!(
                "pool expected to be a {}, not {:?}",
                stringify!($db),
                $crate::PoolDb::from(&(*$pool_holder).pool)
            );
        }
    };
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "settings", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolDbSettings {
    pub url: String,
    pub local_url: Option<String>,
    pub connection_max_count: u32,
    pub live_for_secs: Option<u64>,
    pub retry_secs: Option<u64>,
}

#[cfg(any(feature = "pg", feature = "mysql"))]
pub async fn get(db: PoolDb, settings: PoolDbSettings, op_mode: OpMode) -> Result<PoolDbHolder> {
    let pool = &mut POOL.write().await;
    let url = if !matches!(op_mode, OpMode::Local) {
        settings.url.clone()
    } else if let Some(url) = settings.local_url {
        url
    } else {
        settings.url.clone()
    };
    let ret = pool.get(&url).cloned();
    let ret = if let Some(ret) = ret {
        warn!("will reuse POOL {:?}", url);
        ret
    } else {
        warn!("will get new POOL {:?}", url);
        let ret = Arc::new(PoolDbWrapper {
            pool: match db {
                #[cfg(feature = "pg")]
                PoolDb::Pg => Pool::Pg(
                    sqlx::postgres::PgPoolOptions::new()
                        .min_connections(0)
                        .max_connections(settings.connection_max_count)
                        .idle_timeout(settings.live_for_secs.map(std::time::Duration::from_secs))
                        .connect_lazy(&url)
                        .map_err(|err| anyhow!(err))?,
                ),
                #[cfg(feature = "mysql")]
                PoolDb::MySql => Pool::MySql(
                    sqlx::mysql::MySqlPoolOptions::new()
                        .max_connections(settings.connection_max_count)
                        .idle_timeout(settings.live_for_secs.map(std::time::Duration::from_secs))
                        .connect_lazy(&url)
                        .map_err(|err| anyhow!(err))?,
                ),
            },
        });
        pool.insert(url, ret.clone());
        ret
    };
    Ok(ret)
}

#[cfg(any(feature = "pg", feature = "mysql"))]
lazy_static::lazy_static! {
    static ref POOL: RwLock<HashMap<String, PoolDbHolder >> = RwLock::new(HashMap::new());
}
