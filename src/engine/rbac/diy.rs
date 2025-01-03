use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use crate::dal;

pub static CHECKER: OnceLock<Checker> = OnceLock::new();

pub struct Checker {
    // use `Arc` to avoid cloning String everywhere.
    role_action_map: Arc<HashMap<u32, Vec<Arc<String>>>>,
}

impl Checker {
    pub async fn new() -> Self {
        let map = dal::sqlite::roles_diy::query_all_roles_with_actions()
            .await
            .unwrap();
        let map = map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(Arc::new).collect()))
            .collect();
        Self {
            role_action_map: Arc::new(map),
        }
    }

    pub fn check_atom(&self, role_id: u32, atom: &str) -> bool {
        if let Some(actions) = self.role_action_map.get(&role_id) {
            actions.iter().any(|a| **a == *atom)
        } else {
            false
        }
    }

    pub fn get_atoms(&self, role_id: u32) -> Vec<Arc<String>> {
        let actions = self.role_action_map.get(&role_id);
        if actions.is_none() {
            return vec![];
        }
        actions.unwrap().to_vec()
    }
}
