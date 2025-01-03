#[derive(sqlx::FromRow, Debug)]
pub struct Talent {
    pub id: u32,
    pub hide: u8,
    pub shared_users: Option<Vec<u32>>
}
