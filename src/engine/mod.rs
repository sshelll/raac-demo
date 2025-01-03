#![allow(dead_code)]


use crate::util;
use cedar_policy::{Context, EntityUid};

mod abac;
mod authentity;
mod rbac;
pub mod resource;
mod talent;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    DbError(sqlx::Error),
    EntityNotFound(String),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DbError(value)
    }
}

static INIT_ONCE: tokio::sync::OnceCell<()> = tokio::sync::OnceCell::const_new();

pub async fn init() {
    INIT_ONCE.get_or_init(init0).await;
}

async fn init0() {
    rbac::init().await;
    abac::init();
    resource::init();
    authentity::init();
}

// export the inner auth primitives

// abac - check talent access
#[allow(unused_imports)]
pub use talent::check_talent_access;

// rbac - check by preset roles
pub async fn check_atom_preset(user_id: u32, atom: &str, resource: &EntityUid) -> Result<bool> {
    let user = authentity::get_user(user_id).await?;
    let action = util::cedar::atom_to_action(atom);
    let context = Context::empty();
    Ok(rbac::check_preset(&user, &action, resource, &context))
}

// rbac - check by diy roles
pub async fn check_atom_diy(user_id: u32, atom: &str) -> Result<bool> {
    let user = authentity::get_user(user_id).await?;
    Ok(rbac::check_atom(&user, atom))
}
