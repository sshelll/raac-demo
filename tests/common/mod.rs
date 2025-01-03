use tokio::sync::OnceCell;

use raac_demo::{bootstrap, dal};

static INIT: OnceCell<()> = OnceCell::const_new();

pub async fn setup() {
    INIT.get_or_init(init0).await;
}

async fn init0() {
    init_logger();
    dal::init().await;
    bootstrap::boot().await;
}

fn init_logger() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format_level(true)
        .format_timestamp_millis()
        .init();
}

#[allow(unused_macros)]
macro_rules! entity_uid {
    ($s: expr) => {{
        use std::str::FromStr;
        cedar_policy::EntityUid::from_str($s).expect("entity parse error")
    }};
}

#[allow(unused_imports)]
pub(crate) use entity_uid;
