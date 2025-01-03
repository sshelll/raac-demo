use sqlx::QueryBuilder;

use crate::dal::models::Role;

use super::get_conn;

pub async fn batch_insert_ignore_dup(models: Vec<Role>) -> sqlx::Result<u64> {
    let mut builder = QueryBuilder::new("insert into roles_preset (id, desc) ");
    let query = builder
        .push_values(models, |mut b, m| {
            b.push_bind(m.id).push_bind(m.desc);
        })
        .push(" on conflict do nothing")
        .build();
    let res = query.execute(&*get_conn()).await?;
    Ok(res.rows_affected())
}
