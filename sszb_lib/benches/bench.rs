use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use sszb::{SszDecode, SszEncode};

pub mod beacon_block;
pub use beacon_block::SignedBeaconBlock;

pub mod beacon_state;
pub use beacon_state::BeaconState;

fn basic_types(c: &mut Criterion) {
    use milhouse::List;

    type C = typenum::U1099511627776;
    const N: u64 = 1_000_000;

    let mut group = c.benchmark_group("List");

    // basic test case
    let size = N;
    let list = List::<u64, C>::try_from_iter(0..size).unwrap();
    let list_bytes = list.to_ssz();

    group.throughput(Throughput::Bytes(list_bytes.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Milhouse", "decode"),
        &list_bytes,
        |b, bytes| b.iter(|| <List<u64, C> as SszDecode>::from_ssz_bytes(bytes).unwrap()),
    );

    group.bench_with_input(BenchmarkId::new("Milhouse", "to_ssz"), &list, |b, list| {
        b.iter(|| list.to_ssz())
    });

    group.bench_with_input(
        BenchmarkId::new("Milhouse", "ssz_write to slice"),
        &list,
        |b, list| {
            let len = list.sszb_bytes_len();
            let mut buf: Vec<u8> = vec![0u8; len];
            b.iter(|| list.ssz_write(&mut buf.as_mut_slice()))
        },
    );

    group.finish();
}

fn beacon_block(c: &mut Criterion) {
    let mut group = c.benchmark_group("SignedBeaconBlock");
    let block_bytes: Vec<u8> = std::fs::read("beacon-block.ssz").unwrap();
    let beacon_block =
        <SignedBeaconBlock as SszDecode>::from_ssz_bytes(block_bytes.as_slice()).unwrap();
    group.throughput(Throughput::Bytes(block_bytes.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Sszb", "decode"),
        block_bytes.as_slice(),
        |b, bytes| b.iter(|| <SignedBeaconBlock as SszDecode>::from_ssz_bytes(bytes).unwrap()),
    );

    group.bench_with_input(
        BenchmarkId::new("Sszb", "encode naive"),
        &beacon_block,
        |b, block| b.iter(|| block.to_ssz()),
    );

    group.bench_with_input(
        BenchmarkId::new("Sszb", "ssz_write to slice"),
        &beacon_block,
        |b, block| {
            let len = block.sszb_bytes_len();
            let mut buf: Vec<u8> = vec![0u8; len];
            b.iter(|| block.ssz_write(&mut buf.as_mut_slice()))
        },
    );

    group.finish();
}

fn beacon_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("BeaconState");
    let state_bytes: Vec<u8> = std::fs::read("beacon-state.ssz").unwrap();
    let beacon_state = <BeaconState as SszDecode>::from_ssz_bytes(state_bytes.as_slice()).unwrap();
    group.throughput(Throughput::Bytes(state_bytes.len() as u64));
    group.sample_size(10);

    group.bench_with_input(
        BenchmarkId::new("Sszb", "decode"),
        state_bytes.as_slice(),
        |b, bytes| b.iter(|| <BeaconState as SszDecode>::from_ssz_bytes(bytes).unwrap()),
    );

    group.bench_with_input(
        BenchmarkId::new("Sszb", "encode naive"),
        &beacon_state,
        |b, state| b.iter(|| state.to_ssz()),
    );

    group.bench_with_input(
        BenchmarkId::new("Sszb", "ssz_write to slice"),
        &beacon_state,
        |b, state| {
            let len = state.sszb_bytes_len();
            let mut buf: Vec<u8> = vec![0u8; len];
            b.iter(|| state.ssz_write(&mut buf.as_mut_slice()))
        },
    );

    group.finish();
}

criterion_group!(benches, basic_types, beacon_block, beacon_state);
criterion_main!(benches);
