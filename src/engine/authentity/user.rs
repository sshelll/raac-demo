use std::sync::Arc;

use cedar_policy::{Entity, EntityUid};
use serde::Serialize;

use crate::engine::{rbac, Error, Result};

use super::Uid;

#[derive(Serialize, Debug)]
pub struct User<'a> {
    uid: Uid<'a>,
    attrs: UserAttrs,
    parents: Vec<Uid<'a>>,
    // diy roles, skip serialize
    #[serde(skip)]
    pub diy_role_ids: Vec<u32>,
}

#[derive(Serialize, Debug)]
pub struct UserAttrs {
    user_id: u32,
    atoms: Vec<Arc<String>>,
}

impl User<'_> {
    pub async fn new(user_id: u32) -> Result<Self> {
        use crate::dal::sqlite;

        // query user model
        let user = sqlite::user::query(user_id).await?;
        if user.is_none() {
            return Err(Error::EntityNotFound(format!("user {user_id} not found")));
        }

        // query roles
        let (preset_roles, diy_roles) = sqlite::user_role_ref::query_user_roles(user_id).await?;

        // attach preset roles as parents
        let parents = preset_roles
            .into_iter()
            .filter_map(|id| {
                Self::role_uid(id).map(|uid| Uid {
                    type_: "Role",
                    id: uid,
                })
            })
            .collect();

        let atoms = diy_roles.iter().fold(vec![], |mut acc, &role_id| {
            acc.extend(rbac::get_diy_checker().get_atoms(role_id));
            acc
        });

        Ok(Self {
            uid: Uid {
                type_: "User",
                id: user_id.to_string(),
            },
            attrs: UserAttrs { user_id, atoms },
            parents,
            diy_role_ids: diy_roles, // attach diy roles
        })
    }

    pub fn principle(&self) -> EntityUid {
        EntityUid::from(&self.uid)
    }

    pub fn entity(&self) -> Entity {
        // serialize self
        let s = serde_json::to_value(self).unwrap();
        Entity::from_json_value(s, None).unwrap()
    }

    fn role_uid(id: u32) -> Option<String> {
        rbac::get_preset_role_id_to_uid_map().get(&id).cloned()
    }
}
