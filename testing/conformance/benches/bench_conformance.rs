//#[macro_use]
extern crate criterion;

use std::env::var;
use std::iter;
use std::path::{Path, PathBuf};
use std::time::Duration;

use colored::Colorize;
use conformance_tests::driver::*;
use conformance_tests::report;
use criterion::*;
use walkdir::WalkDir;

mod bench_drivers;

use crate::bench_drivers::{
    bench_vector_file, load_vector_file, BenchVectorFileConfig, CheckStrength,
};

/// Either grabs an environment variable called VECTOR and benches that test vector using criterion, or runs all of them in sequence. Displays output for results of benchmarking.
fn bench_conformance(c: &mut Criterion) {
    pretty_env_logger::init();

    // TODO match globs to get whole folders?
    let (mut vector_results, _is_many): (Vec<PathBuf>, bool) = match var("VECTOR") {
        Ok(v) => (
            iter::once(Path::new(v.as_str()).to_path_buf()).collect(),
            false,
        ),
        Err(_) => (
            WalkDir::new("test-vectors/corpus")
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(is_runnable)
                .map(|e| e.path().to_path_buf())
                .collect(),
            true,
        ),
    };

    // TODO: this is 30 seconds per benchmark... yeesh! once we get the setup running faster (by cloning VMs more efficiently), we can probably bring this down.
    let mut group = c.benchmark_group("conformance-tests");
    group.measurement_time(Duration::new(30, 0));

    for vector_path in vector_results.drain(..) {
        let mut message_vector = match load_vector_file(vector_path.clone()) {
            Ok(Some(mv)) => mv,
            Err(e) => {
                report!(
                    "FILE PARSING FAIL/NOT BENCHED".white().on_purple(),
                    &vector_path.display().to_string(),
                    "n/a"
                );
                println!("\t|> reason: {:#}", e);
                continue;
            }
            Ok(None) => {
                report!(
                    "SKIPPING FILE DUE TO SELECTOR".on_yellow(),
                    &vector_path.display().to_string(),
                    "n/a"
                );
                continue;
            }
        };
        match bench_vector_file(
            &mut group,
            &mut message_vector,
            BenchVectorFileConfig {
                only_first_variant: false,
                check_strength: CheckStrength::default(),
                replacement_apply_messages: None,
                bench_name: vector_path.display().to_string().clone(),
            },
        ) {
            Ok(()) => report!(
                "SUCCESSFULLY BENCHED TEST FILE".on_green(),
                vector_path.display(),
                "n/a"
            ),
            Err(e) => {
                report!(
                    "FAILED TO BENCH TEST FILE".white().on_red(),
                    vector_path.display(),
                    "n/a"
                );
                println!("\t|> reason: {:#}", e);
            }
        };
    }

    group.finish();
}

criterion_group!(benches, bench_conformance);
criterion_main!(benches);
