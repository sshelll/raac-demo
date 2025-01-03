use crate::dal::models::User;
use crate::util::sqlx::{map_query_result, QueryResult};

use super::get_conn;

pub async fn query(user_id: u32) -> QueryResult<User> {
    let res = sqlx::query_as("select * from user where id = ?")
        .bind(user_id)
        .fetch_one(&*get_conn())
        .await;
    map_query_result!(res)
}
