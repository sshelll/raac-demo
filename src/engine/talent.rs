use cedar_policy::Context;
use log::info;

use crate::util::cedar::atom_to_action;

use super::{abac, authentity, rbac, Result};

pub async fn check_talent_access(user_id: u32, talent_id: u32, atom: &str) -> Result<bool> {
    // query user and talent auth entity
    let user = authentity::get_user(user_id).await?;

    let talent = authentity::Talent::new(talent_id).await?;

    // do the rbac check
    {
        let action = atom_to_action(atom);
        let resource = talent.entity_uid();
        let context = Context::empty();

        // Talent access control is a complex case in our system,
        // so we should only check preset rules here.
        // Having the atom doesn't mean we can access the talent resource, because it's probably
        // hidden.
        let preset_ok = rbac::check_preset(&user, &action, &resource, &context);
        if preset_ok {
            info!("rbac ok for user-{user_id} to acquire '{atom}' on talent-{talent_id}");
            return Ok(true);
        }
    }

    let ok = abac::TALENT_CHECKER
        .get()
        .unwrap()
        .is_authorized(&talent, &user, atom);
    if ok {
        info!("abac ok for user-{user_id} to acquire '{atom}' on talent-{talent_id}");
    }
    Ok(ok)
}
