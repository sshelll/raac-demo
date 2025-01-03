use sqlx::Result;
use sqlx::Row;

use crate::dal::sqlite::get_conn;

const USER_PRESET_ROLE_REF_TABLE_NAME: &str = "user_preset_role_ref";
const USER_DIY_ROLE_REF_TABLE_NAME: &str = "user_diy_role_ref";

pub async fn query_user_roles(user_id: u32) -> Result<(Vec<u32>, Vec<u32>)> {
    let conn = get_conn();
    let sql = format!(
        "select role_id from {} where user_id = ?",
        USER_PRESET_ROLE_REF_TABLE_NAME
    );
    let preset = sqlx::query(&sql).bind(user_id).fetch_all(&*conn);

    let conn = get_conn();
    let sql = format!(
        "select role_id from {} where user_id = ?",
        USER_DIY_ROLE_REF_TABLE_NAME
    );
    let diy = sqlx::query(&sql).bind(user_id).fetch_all(&*conn);

    let res = tokio::try_join!(preset, diy)?;

    let preset_roles = res.0.into_iter().map(|row| row.get(0)).collect();
    let diy_roles = res.1.into_iter().map(|row| row.get(0)).collect();
    Ok((preset_roles, diy_roles))
}
