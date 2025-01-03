mod diy;
mod preset;

use std::{collections::HashMap, fs};

use super::authentity;
use cedar_policy::Context;
use cedar_policy::Decision;
use cedar_policy::Entities;
use cedar_policy::EntityUid;
use cedar_policy::Request;
use log::error;

use diy::CHECKER as DIY_CHECKER;
use preset::CHECKER as PRESET_CHECKER;
use preset::ROLE_ID_UID_MAP as PRESET_ROLE_ID_UID_MAP;

pub fn get_diy_checker() -> &'static diy::Checker {
    DIY_CHECKER.get().unwrap()
}

pub fn get_preset_role_id_to_uid_map() -> &'static HashMap<u32, String> {
    PRESET_ROLE_ID_UID_MAP.get().unwrap()
}

pub async fn init() {
    use crate::util::cedar::*;
    preset::CHECKER.get_or_init(|| {
        preset::Checker::new(
            policy_path("preset.role.cedar"),
            entity_path("preset.role.json"),
            Some(schema_path("preset.role.cedar")),
        )
    });

    preset::ROLE_ID_UID_MAP.get_or_init(|| {
        let f = entity_path("preset.role.json");
        let content = fs::read_to_string(f.as_path()).expect("read file failed");
        let jv: serde_json::Value = serde_json::from_str(&content).expect("parse json failed");
        jv.as_array()
            .unwrap()
            .iter()
            .fold(HashMap::new(), |mut acc, v| {
                let db_id = v
                    .get("attrs")
                    .unwrap()
                    .get("db_id")
                    .unwrap()
                    .as_u64()
                    .unwrap();
                let uid = v
                    .get("uid")
                    .unwrap()
                    .get("id")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                acc.insert(db_id as u32, uid);
                acc
            })
    });

    let diy_checker = diy::Checker::new().await;
    diy::CHECKER.get_or_init(|| diy_checker);
}

pub fn check_preset(
    user: &authentity::User,
    action: &EntityUid,
    resource: &EntityUid,
    context: &Context,
) -> bool {
    let principal = user.principle();
    let entity = user.entity();

    match Request::new(
        principal,
        action.to_owned(),
        resource.to_owned(),
        context.to_owned(),
        None,
    ) {
        Err(e) => {
            error!("error creating request: {:?}", e);
            false
        }
        Ok(req) => {
            Decision::Allow
                == PRESET_CHECKER
                    .get()
                    .expect("preset checker not inintialized")
                    .is_authorized(&req, Entities::from_entities([entity], None).unwrap())
                    .decision()
        }
    }
}

pub fn check_atom(user: &authentity::User, atom: &str) -> bool {
    let checker = DIY_CHECKER.get().unwrap();
    user.diy_role_ids
        .iter()
        .any(|&role_id| checker.check_atom(role_id, atom))
}
