use criterion::{criterion_group, criterion_main, Criterion};

const CYCLE: usize = 50_000;

fn crossbeam_mpmc(c: &mut Criterion) {
    c.bench_function("crossbeam_mpmc", |b| b.iter(|| channel_latency::crossbeam_mpmc(CYCLE)));
}

fn crossbeam_spsc(c: &mut Criterion) {
    c.bench_function("crossbeam_spsc", |b| b.iter(|| channel_latency::crossbeam_spsc(CYCLE)));
}

fn std_mpsc(c: &mut Criterion) {
    c.bench_function("std_mpsc", |b| b.iter(|| channel_latency::std_mpsc(CYCLE)));
}

fn sync_flume(c: &mut Criterion) {
    c.bench_function("sync_flume", |b| b.iter(|| channel_latency::sync_flume(CYCLE)));
}

fn tokio_flume(c: &mut Criterion) {
    c.bench_function("tokio_flume", |b| {
        b.iter(|| channel_latency::tokio_flume(CYCLE))
    });
}

fn tokio_mpsc(c: &mut Criterion) {
    c.bench_function("tokio_mpsc", |b| {
        b.iter(|| channel_latency::tokio_mpsc(CYCLE))
    });
}

fn async_channel(c: &mut Criterion) {
    c.bench_function("async_channel", |b| {
        b.iter(|| channel_latency::async_channel(CYCLE))
    });
}

criterion_group!(benches, async_channel, crossbeam_mpmc, crossbeam_spsc, std_mpsc, sync_flume, tokio_flume, tokio_mpsc);
criterion_main!(benches);
