use std::str::FromStr;

use cedar_policy::*;

const POLICY_SRC: &str = r#"
permit (
    principal,
    action in [Action::"view", Action::"edit", Action::"delete"],
    resource == Photo::"vacationPhoto.jpg"
);
"#;

fn main() {
    let principal = EntityUid::from_str("User::\"alice\"").expect("entity parse error");
    let action = EntityUid::from_str("Action::\"view\"").expect("entity parse error");
    let resource = EntityUid::from_str("Photo::\"vacationPhoto.jpg\"").expect("entity parse error");

    let context_json_val: serde_json::value::Value = serde_json::json!({});
    let context = Context::from_json_value(context_json_val, None).unwrap();

    let request = Request::new(principal, action, resource, context, None).unwrap();

    let policy_set = PolicySet::from_str(POLICY_SRC).expect("policy parse error");

    let entities_json = r#"[]"#;
    let entities = Entities::from_json_str(entities_json, None).expect("entity parse error");

    let authorizer = Authorizer::new();
    let decision = authorizer.is_authorized(&request, &policy_set, &entities);

    println!("{:?}", decision.decision());
}
