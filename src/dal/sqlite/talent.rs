use crate::{
    dal::models,
    util::{sqlx::{try_unwrap_query_result, QueryResult}},
};
use sqlx::Row;

use super::get_conn;

pub async fn query_talent_join_shared_users(talent_id: u32) -> QueryResult<models::Talent> {
    let sql = r#"
        select id, hide, user_id from talent
        left join talent_share_ref as ref on talent.id = ref.talent_id
        where id = ?
        "#;
    let res = sqlx::query(sql)
        .bind(talent_id)
        .fetch_all(&*get_conn())
        .await;
    try_unwrap_query_result!(res);
    if res.is_empty() {
        return Ok(None);
    }
    let shared_users: Vec<u32> = res
        .iter()
        .filter_map(|row| row.get::<Option<u32>, _>(2))
        .collect();
    Ok(Some(models::Talent {
        id: talent_id,
        hide: res[0].get(1),
        shared_users: if shared_users.is_empty() {
            None
        } else {
            Some(shared_users)
        },
    }))
}
