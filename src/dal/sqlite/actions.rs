use sqlx::QueryBuilder;

use super::get_conn;
use crate::dal::models::Action;

pub async fn batch_insert_ignore_dup(models: Vec<Action>) -> sqlx::Result<u64> {
    let mut builder = QueryBuilder::new("insert into actions (id, atom, resource, desc) ");
    let query = builder
        .push_values(models, |mut b, m| {
            b.push_bind(m.id)
                .push_bind(m.atom)
                .push_bind(m.resource)
                .push_bind(m.desc);
        })
        .push(" on conflict do nothing")
        .build();
    let res = query.execute(&*get_conn()).await?;
    Ok(res.rows_affected())
}
