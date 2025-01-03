use criterion::{criterion_group, criterion_main, Criterion};
use raac_demo::{dal, engine};

async fn judge() {
    let ok = engine::check_talent_access(3, 2781, "/talent/view")
        .await
        .unwrap();
    assert!(ok);
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async {
        dal::init().await;
        engine::init().await;
    });
    c.bench_function("judge", |b| {
        b.to_async(&rt).iter(judge);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
