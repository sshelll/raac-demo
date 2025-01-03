#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: u64,
    name: String,
}
