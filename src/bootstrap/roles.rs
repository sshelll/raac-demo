use std::fs;

use serde::Deserialize;

use crate::dal::{self, models::Role};

#[derive(Deserialize)]
struct Uid {
    id: String,
}

#[derive(Deserialize)]
struct Attrs {
    db_id: u32,
}

#[derive(Deserialize)]
struct Item {
    uid: Uid,
    attrs: Attrs,
}

pub async fn boot() {
    let content = fs::read_to_string(
        fs::canonicalize("resource/entities/preset.role.json")
            .unwrap()
            .as_path(),
    )
    .unwrap();

    let preset_roles = serde_json::from_str::<Vec<Item>>(&content)
        .unwrap()
        .into_iter()
        .map(|item| Role {
            id: item.attrs.db_id,
            desc: item.uid.id,
        })
        .collect();

    dal::sqlite::roles_preset::batch_insert_ignore_dup(preset_roles)
        .await
        .unwrap();
}
