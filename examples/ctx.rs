use std::str::FromStr;

use cedar_policy::*;

const POLICY_SRC: &str = r#"
permit(
    principal in User::"alice", 
    action in [Action::"update", Action::"delete"],
    resource == Photo::"flower.jpg")
when {
    context.mfa_authenticated == true &&
    context.request_client_ip == "222.222.222.222"
};
"#;

fn main() {
    let principal = EntityUid::from_str("User::\"alice\"").expect("entity parse error");
    let action = EntityUid::from_str("Action::\"update\"").expect("entity parse error");
    let resource = EntityUid::from_str("Photo::\"flower.jpg\"").expect("entity parse error");

    let context_json_val: serde_json::value::Value = serde_json::json!({
        "mfa_authenticated": true,
        "request_client_ip": "222.222.222.222",
        "oidc_scope": "profile",
        "chores": [],
    });
    let context = Context::from_json_value(context_json_val, None).unwrap();

    let request = Request::new(principal, action, resource, context, None).unwrap();

    let policy_set = PolicySet::from_str(POLICY_SRC).expect("policy parse error");

    let entities = Entities::from_json_str("[]", None).expect("entity parse error");

    let authorizer = Authorizer::new();
    let decision = authorizer.is_authorized(&request, &policy_set, &entities);
    println!("{:?}", decision.decision());
}
