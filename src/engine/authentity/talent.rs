use cedar_policy::{Entity, EntityUid};
use serde::Serialize;

use crate::{dal, engine::Error, engine::Result};

use super::Uid;

#[derive(Serialize, Debug)]
pub struct Talent<'a> {
    uid: Uid<'a>,
    attrs: TalentAttr,
    parents: Vec<Uid<'a>>,
}

#[derive(Serialize, Debug)]
pub struct TalentAttr {
    shared_users: Option<Vec<u32>>,
    hide: u8,
}

impl Talent<'_> {
    pub async fn new(talent_id: u32) -> Result<Self> {
        let talent = dal::sqlite::talent::query_talent_join_shared_users(talent_id).await?;
        if talent.is_none() {
            return Err(Error::EntityNotFound(format!(
                "talent {talent_id} not found"
            )));
        }
        let talent = talent.unwrap();
        Ok(Self {
            uid: Uid {
                type_: "Talent",
                id: talent_id.to_string(),
            },
            attrs: TalentAttr {
                shared_users: talent.shared_users,
                hide: talent.hide,
            },
            parents: vec![],
        })
    }

    pub fn entity_uid(&self) -> EntityUid {
        EntityUid::from(&self.uid)
    }

    pub fn entity(&self) -> Entity {
        let s = serde_json::to_value(self).unwrap();
        Entity::from_json_value(s, None).unwrap()
    }
}
