use std::str::FromStr;

use cedar_policy::EntityUid;
use once_cell::sync::OnceCell;

static SYSTEM_SETTINGS: OnceCell<EntityUid> = OnceCell::new();

pub fn system_settings() -> &'static EntityUid {
    SYSTEM_SETTINGS
        .get()
        .expect("SystemSettings not initialized")
}

pub fn init() {
    SYSTEM_SETTINGS.get_or_init(|| EntityUid::from_str(r#"SystemSettings::"-""#).unwrap());
}
