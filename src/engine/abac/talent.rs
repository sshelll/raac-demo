use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, OnceLock};

use cedar_policy::*;
use serde_json::json;

use crate::engine::authentity;

pub static CHECKER: OnceLock<Checker> = OnceLock::new();

pub struct Checker {
    policies: Arc<PolicySet>,
    authorizer: Authorizer,
}

impl Checker {
    pub fn new<P: AsRef<Path>>(policy_file: P, schema_file: Option<P>) -> Self {
        use crate::util::cedar::*;
        let policy_set = read_policy_set_from_file(&policy_file);
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
            authorizer: Authorizer::new(),
        }
    }

    pub fn is_authorized(
        &self,
        talent: &authentity::Talent,
        user: &authentity::User,
        atom: &str,
    ) -> bool {
        let principal = user.principle();
        let action = EntityUid::from_str(format!(r#"Action::"{}""#, atom).as_str()).unwrap();
        let resource = talent.entity_uid();
        let ctx = Context::from_json_value(json!({}), None).unwrap();
        let request = Request::new(principal, action, resource, ctx, None).unwrap();

        let entities = Entities::from_entities([talent.entity(), user.entity()], None).unwrap();

        self.authorizer
            .is_authorized(&request, &self.policies, &entities)
            .decision()
            == Decision::Allow
    }
}
