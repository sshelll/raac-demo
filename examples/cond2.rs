use std::str::FromStr;

use cedar_policy::*;

const POLICY_SRC: &str = r#"
permit(
  principal, 
  action in [Action::"view", Action::"edit", Action::"delete"], 
  resource 
)
when {
  resource.owner == principal.id
};
"#;

const ENTITIES_JSON: &str = r#"
[
    {
        "uid": {
            "type": "Photo",
            "id": "VacationPhoto94.jpg"
        },
        "attrs": {
            "dept": "cosmic",
            "owner": "alice"
        },
        "parents": []
    },
    {
        "uid": {
            "type": "User",
            "id": "alice"
        },
        "attrs": {
            "id": "alice",
            "dept": "chaos"
        },
        "parents": []
    }
]
"#;

fn main() {
    let principal = EntityUid::from_str("User::\"alice\"").expect("entity parse error");
    let action = EntityUid::from_str("Action::\"view\"").expect("entity parse error");
    let resource =
        EntityUid::from_str("Photo::\"VacationPhoto94.jpg\"").expect("entity parse error");

    let context_json_val = serde_json::json!({});
    let context = Context::from_json_value(context_json_val, None).unwrap();

    let request = Request::new(principal, action, resource, context, None).unwrap();

    let policy_set = PolicySet::from_str(POLICY_SRC).expect("policy parse error");

    let entities = Entities::from_json_str(ENTITIES_JSON, None).expect("entity parse error");

    let authorizer = Authorizer::new();
    let decision = authorizer.is_authorized(&request, &policy_set, &entities);
    println!("{:?}", decision.decision());
}
