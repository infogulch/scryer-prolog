#![feature(test)]
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};

use scryer_prolog::machine::parsed_results::{QueryResolution, Value};

mod run;
use run::edges;

#[library_benchmark]
fn bench_edges() {
    let result = black_box(edges(black_box(String::from(
        r#"independent_set(Sat), sat_count(Sat, Count)."#,
    ))));
    if let Ok(QueryResolution::Matches(ms)) = result {
        if let Some(m) = ms.first() {
            let count = &m.bindings["Count"];
            println!("{:?}", count);
            assert_eq!(*count, Value::from("217968"));
            return;
        }
    }
    panic!("failed to match")
}

library_benchmark_group!(
    name = bench_scryer;
    benchmarks = bench_edges
);

main!(library_benchmark_groups = bench_scryer);
