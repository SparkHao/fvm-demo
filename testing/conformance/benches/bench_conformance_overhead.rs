extern crate criterion;
use std::env::var;
use std::path::{Path, PathBuf};
use std::time::Duration;

use conformance_tests::driver::*;
use conformance_tests::vector::ApplyMessage;
use criterion::*;
use fvm::machine::BURNT_FUNDS_ACTOR_ADDR;
use fvm_shared::econ::TokenAmount;
use fvm_shared::encoding::{Cbor, RawBytes};
use fvm_shared::message::Message;
use walkdir::WalkDir;

mod bench_drivers;
use crate::bench_drivers::{
    bench_vector_file, load_vector_file, BenchVectorFileConfig, CheckStrength,
};

/// benches only machine setup, no messages get sent. This is basically overhead of the benchmarks themselves.
fn bench_init_only(
    group: &mut BenchmarkGroup<measurement::WallTime>,
    path_to_setup: PathBuf,
) -> anyhow::Result<()> {
    // compute measurement overhead by benching running a single empty vector of zero messages
    let mut message_vector = match load_vector_file(path_to_setup)? {
        Some(mv) => Ok(mv),
        None => Err(anyhow::anyhow!(
            "chosen vector was filtered out by selector"
        )),
    }?;
    bench_vector_file(
        group,
        &mut message_vector,
        BenchVectorFileConfig {
            replacement_apply_messages: Some(vec![]),
            only_first_variant: true,
            bench_name: "bench_init_only".to_owned(),
            check_strength: CheckStrength::OnlyCheckSuccess,
        },
    )
}

/// benchmarks calling 500 simple state accesses. This benchmark computes the overhead of the message plus state access itself, doing a minimal amount of computation within the FVM.
fn bench_500_simple_state_access(
    group: &mut BenchmarkGroup<measurement::WallTime>,
    path_to_setup: PathBuf,
) -> anyhow::Result<()> {
    let five_hundred_state_accesses = (0..500)
        .map(|i| ApplyMessage {
            bytes: Message {
                version: 0,
                from: BURNT_FUNDS_ACTOR_ADDR,
                to: BURNT_FUNDS_ACTOR_ADDR,
                sequence: i,
                value: TokenAmount::from(0u8),
                method_num: 2,
                params: RawBytes::default(),
                gas_limit: 5000000000,
                gas_fee_cap: TokenAmount::from(0u8),
                gas_premium: TokenAmount::from(0u8),
            }
            .marshal_cbor()
            .unwrap(),
            epoch_offset: None,
        })
        .collect();

    let mut message_vector = match load_vector_file(path_to_setup)? {
        Some(mv) => Ok(mv),
        None => Err(anyhow::anyhow!(
            "chosen vector was filtered out by selector"
        )),
    }?;
    bench_vector_file(
        group,
        &mut message_vector,
        BenchVectorFileConfig {
            only_first_variant: true,
            check_strength: CheckStrength::OnlyCheckSuccess,
            replacement_apply_messages: Some(five_hundred_state_accesses),
            bench_name: "bench_500_simple_state_access".to_owned(),
        },
    )
}
/// runs overhead benchmarks, using the contents of the environment variable VECTOR as the starting FVM state
fn bench_conformance_overhead(c: &mut Criterion) {
    pretty_env_logger::init();

    let path_to_setup = match var("VECTOR") {
        Ok(v) => Path::new(v.as_str()).to_path_buf(),
        Err(_) => WalkDir::new("test-vectors/corpus")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(is_runnable)
            .map(|e| e.path().to_path_buf())
            .next()
            .unwrap(),
    };

    // TODO: this is 30 seconds per benchmark... yeesh! once we get the setup running faster (by cloning VMs more efficiently/fixing wasm cache), we can probably bring this down.
    let mut group = c.benchmark_group("measurement-overhead-baselines");
    group.measurement_time(Duration::new(30, 0));
    // start by getting some baselines!
    bench_init_only(&mut group, path_to_setup.clone()).unwrap();
    bench_500_simple_state_access(&mut group, path_to_setup).unwrap();
    group.finish();
}

criterion_group!(benches_overhead, bench_conformance_overhead);
criterion_main!(benches_overhead);
