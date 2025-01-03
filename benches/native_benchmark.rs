use cedar_policy::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::str::FromStr;

const POLICY_SRC: &str = r#"
permit(
  principal is User,
  action in [Action::"/talent/view"],
  resource is Talent
)
when {
    (principal has atoms && principal.atoms.contains("/talent/view_hidden")) ||
    (resource.hide == 0 && principal has atoms && principal.atoms.contains("/talent/view")) ||
    (resource has shared_users && resource.shared_users.contains(principal.user_id))
};
"#;

const ENTITIES: &str = r#"
[
  {
    "uid": {
      "type": "Talent",
      "id": "2791"
    },
    "attrs": {
      "hide": 1,
      "shared_users": [ "Alice" ]
    },
    "parents": []
  },
  {
    "uid": {
      "type": "User",
      "id": "Alice"
    },
    "attrs": {"user_id": "Alice"},
    "parents": []
  }
]
"#;

fn judge() {
    let principal = EntityUid::from_str(r#"User::"Alice""#).expect("entity parse error");
    let action = EntityUid::from_str(r#"Action::"/talent/view""#).expect("entity parse error");
    let resource = EntityUid::from_str(r#"Talent::"2791""#).expect("entity parse error");

    let context_json_val = serde_json::json!({});
    let context = Context::from_json_value(context_json_val, None).unwrap();

    let request = Request::new(principal, action, resource, context, None).unwrap();

    let policy_set = PolicySet::from_str(POLICY_SRC).expect("policy parse error");

    let entities = Entities::from_json_str(ENTITIES, None).expect("entity parse error");

    let authorizer = Authorizer::new();
    if let Decision::Deny = authorizer.is_authorized(&request, &policy_set, &entities).decision() {
        panic!("access denied");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("judge", |b| b.iter(judge));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
