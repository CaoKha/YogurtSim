#![feature(test)]

extern crate test;
extern crate yogurt_sim;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = yogurt_sim::Universe::new(128, 128);

    b.iter(|| {
        universe.tick();
    });
}
