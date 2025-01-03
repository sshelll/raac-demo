use raac_demo::engine;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preset_super_admin() {
    common::setup().await;
    engine::init().await;
    let start = std::time::Instant::now();
    let user_id = 1;
    let handles: Vec<_> = (1..=1)
        .map(|_| tokio::spawn(engine::check_talent_access(user_id, 2781, "/talent/view")))
        .collect();
    for h in handles {
        let ok = h.await.unwrap().unwrap();
        assert!(ok);
    }
    println!("elapsed: {:?}", start.elapsed());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn diy_super_admin() {
    common::setup().await;
    engine::init().await;
    let start = std::time::Instant::now();
    let user_id = 2;
    let handles: Vec<_> = (1..=1)
        .map(|_| tokio::spawn(engine::check_talent_access(user_id, 2781, "/talent/view")))
        .collect();
    for h in handles {
        let ok = h.await.unwrap().unwrap();
        assert!(ok);
    }
    println!("elapsed: {:?}", start.elapsed());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn preset_system_admin() {
    common::setup().await;
    engine::init().await;
    let start = std::time::Instant::now();
    let user_id = 5;
    let ok = engine::check_atom_preset(
        user_id,
        "/system_setting/view",
        engine::resource::system_settings(),
    )
    .await
    .unwrap();
    assert!(ok);
    println!("elapsed: {:?}", start.elapsed());
}
