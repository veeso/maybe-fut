use std::hint::black_box;
use std::path::Path;

use criterion::{Criterion, criterion_group, criterion_main};
use maybe_fut::io::Write as _;
use tokio::io::AsyncWriteExt as _;
use tokio::runtime::Runtime;

async fn is_async_context() {
    maybe_fut::is_async_context();
}

async fn tokio_create_file(path: &Path) {
    let mut f = tokio::fs::File::create(path).await.unwrap();
    f.write_all(b"Hello, world!").await.unwrap();
    f.flush().await.unwrap();
}

async fn maybe_fut_create_file(path: &Path) {
    let mut f = maybe_fut::fs::File::create(path).await.unwrap();
    f.write_all(b"Hello, world!").await.unwrap();
    f.flush().await.unwrap();
}

fn benchmark_is_async_context(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("is_async_context", |b| {
        b.to_async(&rt).iter(|| black_box(is_async_context()))
    });
}

fn benchmark_tokio_create_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let tempfile = tempfile::NamedTempFile::new().unwrap();
    let path = tempfile.path();

    c.bench_function("tokio_create_file", |b| {
        b.to_async(&rt).iter(|| black_box(tokio_create_file(path)))
    });
}

fn benchmark_maybe_fut_create_file(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let tempfile = tempfile::NamedTempFile::new().unwrap();
    let path = tempfile.path();

    c.bench_function("maybe_fut_create_file", |b| {
        b.to_async(&rt)
            .iter(|| black_box(maybe_fut_create_file(path)))
    });
}

criterion_group!(
    benches,
    benchmark_is_async_context,
    benchmark_tokio_create_file,
    benchmark_maybe_fut_create_file
);
criterion_main!(benches);
