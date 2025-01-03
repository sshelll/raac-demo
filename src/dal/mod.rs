pub mod models;
pub mod sqlite;

#[allow(dead_code)]
pub async fn init() {
    sqlite::init().await;
}
