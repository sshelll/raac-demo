use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, OnceLock},
};

use cedar_policy::*;
use once_cell::sync::OnceCell;

pub static CHECKER: OnceLock<Checker> = OnceLock::new();

pub static ROLE_ID_UID_MAP: OnceCell<HashMap<u32, String>> = OnceCell::new();

pub struct Checker {
    policies: Arc<PolicySet>,
    entities: Arc<Entities>,
    authorizer: Authorizer,
}

impl Checker {
    pub fn new<P: AsRef<Path>>(policy_file: P, entity_file: P, schema_file: Option<P>) -> Self {
        use crate::util::cedar::*;
        let policy_set = read_policy_set_from_file(&policy_file);
        let entities = read_entities_from_file(entity_file);
        if let Some(schema_file) = schema_file {
            let schema = read_schemas_from_file(schema_file);
            let validate_result =
                Validator::new(schema).validate(&policy_set, ValidationMode::default());
            if !ValidationResult::validation_passed(&validate_result) {
                println!("validation failed");
                for err in validate_result.validation_errors() {
                    println!("{err}");
                }
                std::process::exit(1);
            }
        }
        Self {
            policies: Arc::new(policy_set),
            entities: Arc::new(entities),
            authorizer: Authorizer::new(),
        }
    }

    pub fn is_authorized(&self, req: &Request, entities: Entities) -> Response {
        // PERF:: clone here
        let entities = (*self.entities)
            .clone()
            .add_entities(entities, None)
            .unwrap();
        self.authorizer
            .is_authorized(req, &self.policies, &entities)
    }
}
