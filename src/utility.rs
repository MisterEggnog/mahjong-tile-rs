use std::iter::Iterator;
use std::iter::{Flatten, RepeatWith, Take};

struct LoopWith<F, I>
where
    I: Iterator,
    F: FnMut() -> I,
{
    inner: Flatten<Take<RepeatWith<F>>>,
}

impl<F, I> Iterator for LoopWith<F, I>
where
    I: Iterator,
    F: FnMut() -> I,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

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
