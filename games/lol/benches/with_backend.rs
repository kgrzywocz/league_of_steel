#[macro_use]
extern crate bencher;
use bencher::Bencher;

use games::*;
use interfaces::GameTrait;

fn bench_pool_events(b: &mut Bencher) {
    let mut sut = LolLib::new().create_game_analyzer();
    b.iter(|| sut.pool_events());
}

benchmark_group!(benches, bench_pool_events);
benchmark_main!(benches);
