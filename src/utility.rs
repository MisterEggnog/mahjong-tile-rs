use std::iter::Iterator;
use std::iter::{Flatten, RepeatWith, Take};

// Loop iterator, removed after commit
// 4ed170a4068dfce6194f9489be10e153e5326740

pub fn loop_iterator_with<I, F>(iter: F, loop_num: usize) -> Flatten<Take<RepeatWith<F>>>
where
    I: Iterator,
    F: FnMut() -> I,
{
    std::iter::repeat_with(iter).take(loop_num).flatten()
}

#[test]
fn loop_three_times() {
    let iter = loop_iterator_with(|| 1..=10, 3);
    assert_eq!(30, iter.count());
}
