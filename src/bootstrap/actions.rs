use core::str;
use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::{
    dal::{
        self,
        models::{self},
    },
    util::quit_now,
};

#[derive(Deserialize)]
struct Action {
    id: u32,
    atom: String,
    resource: String,
    desc: String,
}

#[derive(Deserialize)]
struct ActionConfig {
    #[serde(rename = "action")]
    actions: Option<Vec<Action>>,
}

impl From<ActionConfig> for Vec<models::Action> {
    fn from(value: ActionConfig) -> Self {
        value
            .actions
            .map_or(vec![], |v| v)
            .into_iter()
            .map(|m| models::Action {
                id: m.id,
                atom: m.atom,
                resource: m.resource,
                desc: m.desc,
            })
            .collect()
    }
}

#[inline]
fn action_file_path(file: &str) -> PathBuf {
    fs::canonicalize("resource/actions/".to_string() + file).unwrap()
}

pub async fn boot() {
    let file = action_file_path("all.toml");
    let content = fs::read_to_string(file.as_path())
        .unwrap_or_else(|e| quit_now!("read {:?} error: {}", file.as_path(), e));
    let conf: ActionConfig = toml::from_str(&content)
        .unwrap_or_else(|e| quit_now!("parse {:?} error: {}", file.as_path(), e));
    dal::sqlite::actions::batch_insert_ignore_dup(conf.into())
        .await
        .unwrap();
}
