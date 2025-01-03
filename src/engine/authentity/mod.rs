use std::{str::FromStr, sync::Arc};

use cedar_policy::EntityUid;
use moka::future::Cache;
use once_cell::sync::OnceCell;
use serde::Serialize;

mod user;
pub use user::User;

mod talent;
pub use talent::Talent;

#[derive(Serialize, Debug)]
struct Uid<'a> {
    #[serde(rename = "type")]
    type_: &'a str,
    id: String,
}

#[derive(Serialize, Debug)]
struct Empty {}

impl From<&Uid<'_>> for EntityUid {
    fn from(value: &Uid<'_>) -> Self {
        EntityUid::from_str(format!(r#"{}::"{}""#, value.type_, value.id).as_str()).unwrap()
    }
}

static USER_AUTH_ENTITY_CACHE: OnceCell<Cache<u32, Arc<user::User>>> = OnceCell::new();

pub fn init() {
    USER_AUTH_ENTITY_CACHE.get_or_init(|| Cache::new(10000));
}

// query with cache
pub async fn get_user(user_id: u32) -> super::Result<Arc<user::User<'static>>> {
    unsafe {
        if let Some(user) = USER_AUTH_ENTITY_CACHE.get_unchecked().get(&user_id).await {
            return Ok(user);
        }
    }

    let user = user::User::new(user_id).await?;
    let user = Arc::new(user);

    unsafe {
        USER_AUTH_ENTITY_CACHE
            .get_unchecked()
            .insert(user_id, user.clone())
            .await;
    }

    Ok(user)
}
