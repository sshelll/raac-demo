use raac_demo::engine;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn view_hidden_talent() {
    common::setup().await;
    engine::init().await;
    let start = std::time::Instant::now();

    // user 2 is a diy super admin
    let ok = engine::check_talent_access(2, 2781, "/talent/view")
        .await
        .unwrap();
    assert!(ok);

    // user 2 has the atom
    let ok = engine::check_atom_diy(2, "/talent/view_hidden")
        .await
        .unwrap();
    assert!(ok);

    // user 2 can access by a more strict atom
    let ok = engine::check_talent_access(2, 2781, "/talent/view_hidden")
        .await
        .unwrap();
    assert!(ok);

    // user 3 is nobody, but he has been shared
    let ok = engine::check_talent_access(3, 2781, "/talent/view")
        .await
        .unwrap();
    assert!(ok);

    // user 4 is nobody
    let not_ok = engine::check_talent_access(4, 2781, "/talent/view")
        .await
        .unwrap();
    assert!(!not_ok);

    // user 300 doesn't exist
    if let Err(engine::Error::EntityNotFound(_)) =
        engine::check_talent_access(300, 2781, "/talent/view").await
    {
    } else {
        panic!("should not be found");
    }
    println!("elapsed: {:?}", start.elapsed());
}
