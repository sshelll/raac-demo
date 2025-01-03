use std::str::FromStr;

use cedar_policy::*;

const POLICY_SRC: &str = r#"
permit(
  principal in Role::"vacationPhotoJudges",
  action == Action::"view",
  resource == Photo::"vacationPhoto94.jpg"
);
"#;

const ENTITIES_JSON: &str = r#"[
    {
        "uid": {
            "type": "User",
            "id": "Bob"
        },
        "attrs": {},
        "parents": [
            {
                "type": "Role",
                "id": "seniorPhotographerJudges"
            }
        ]
    },
    {
        "uid": {
            "type": "Role",
            "id": "vacationPhotoJudges"
        },
        "attrs": {},
        "parents": []
    },
    {
        "uid": {
            "type": "Role",
            "id": "seniorPhotographerJudges"
        },
        "attrs": {},
        "parents": [
            {
                "type": "Role",
                "id": "vacationPhotoJudges"
            }
        ]
    }
]"#;

fn main() {
    // [Bob] is a member of the [seniorPhotographerJudges] role
    // A [seniorPhotographerJudges] is a member of the [vacationPhotoJudges] role
    // [Bob] is allowed to [view] [vacationPhoto94.jpg]
    let principal = EntityUid::from_str("User::\"Bob\"").expect("entity parse error");
    let action = EntityUid::from_str("Action::\"view\"").expect("entity parse error");
    let resource =
        EntityUid::from_str("Photo::\"vacationPhoto94.jpg\"").expect("entity parse error");

    let context_json_val = serde_json::json!({});
    let context = Context::from_json_value(context_json_val, None).unwrap();

    let request = Request::new(principal, action, resource, context, None).unwrap();

    let policy_set = PolicySet::from_str(POLICY_SRC).expect("policy parse error");

    let entities = Entities::from_json_str(ENTITIES_JSON, None).expect("entity parse error");

    let authorizer = Authorizer::new();
    let decision = authorizer.is_authorized(&request, &policy_set, &entities);
    println!("{:?}", decision.decision());
}
