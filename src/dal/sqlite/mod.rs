#![allow(dead_code)]

use std::sync::Arc;

use sqlx::{Pool, Sqlite};
use tokio::sync::OnceCell;

use crate::util::quit_now;

pub mod actions;
pub mod roles_diy;
pub mod roles_preset;
pub mod talent;
pub mod user;
pub mod user_role_ref;

const DB: &str = "raac.db";

type ConnPool = Pool<Sqlite>;

static CONN_POOL: OnceCell<Arc<ConnPool>> = OnceCell::const_new();

fn get_conn() -> Arc<ConnPool> {
    let conn = CONN_POOL.get().unwrap();
    conn.clone()
}

pub async fn init() {
    CONN_POOL.get_or_init(init_conn).await;
}

async fn init_conn() -> Arc<Pool<Sqlite>> {
    Arc::new(
        sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(DB)
            .await
            .unwrap_or_else(|e| quit_now!("connect to db failed: {}", e)),
    )
}
