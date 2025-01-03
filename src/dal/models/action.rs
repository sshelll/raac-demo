#[derive(sqlx::FromRow, Debug)]
pub struct Action {
    pub id: u32,
    pub atom: String,
    pub resource: String,
    pub desc: String
}
