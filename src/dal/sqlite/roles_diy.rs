use std::collections::HashMap;

use super::get_conn;
use sqlx::Row;

pub async fn query_all_roles_with_actions() -> sqlx::Result<HashMap<u32, Vec<String>>> {
    let sql = r#"
        select diy_role_id, act.atom
        from diy_role_action_ref
            join actions act on diy_role_action_ref.action_id = act.id;
        "#;
    let res = sqlx::query(sql).fetch_all(&*get_conn()).await?;
    Ok(res.into_iter().fold(HashMap::new(), |mut acc, row| {
        let role_id: u32 = row.get(0);
        let atom: String = row.get(1);
        acc.entry(role_id).or_default().push(atom);
        acc
    }))
}
