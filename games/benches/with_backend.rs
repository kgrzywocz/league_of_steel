#[macro_use]
extern crate bencher;
use bencher::Bencher;

use lol_lib::*;

fn bench_pool_events(b: &mut Bencher) {
    let mut sut = LolLib::new();
    b.iter(|| sut.pool_events());
}

benchmark_group!(benches, bench_pool_events);
benchmark_main!(benches);
