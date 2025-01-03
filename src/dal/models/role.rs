#[derive(sqlx::FromRow, Debug)]
pub struct Role {
    pub id: u32,
    pub desc: String,
}
