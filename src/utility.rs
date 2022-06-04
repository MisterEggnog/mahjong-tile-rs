use std::iter::Iterator;

pub fn loop_iterator<I: Iterator + Clone>(iter: I, loop_num: usize) -> impl Iterator {
    std::iter::repeat(iter).take(loop_num).flatten()
}

pub fn loop_iterator_with<I, F>(iter: F, loop_num: usize) -> impl Iterator
where
    I: Iterator,
    F: FnMut() -> I,
{
    std::iter::repeat_with(iter).take(loop_num).flatten()
}

#[test]
fn loop_three_times() {
    let iter = loop_iterator(1..=10, 3);
    assert_eq!(30, iter.count());

    let iter = loop_iterator_with(|| 1..=10, 3);
    assert_eq!(30, iter.count());
}
