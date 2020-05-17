#[macro_use]
extern crate bencher;
use bencher::Bencher;

use lol_lib::*;


fn get_stats(b: &mut Bencher) {
    let mut sut=LolLib::new();
    b.iter(|| sut.get_stats());
}

benchmark_group!(benches, get_stats);
benchmark_main!(benches);