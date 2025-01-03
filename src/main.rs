mod bootstrap;
mod dal;
mod engine;
mod util;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    init_logger();
}

fn init_logger() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format_level(true)
        .format_timestamp_millis()
        .init();
}
