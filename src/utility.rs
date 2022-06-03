use std::iter::Iterator;

pub fn loop_iterator<I: Iterator + Clone>(iter: I, loop_num: usize) -> impl Iterator {
    std::iter::repeat(iter).take(loop_num).flatten()
}

#[test]
fn loop_three_times() {
    let iter = loop_iterator(1..=10, 3);
    assert_eq!(30, iter.count());
}
