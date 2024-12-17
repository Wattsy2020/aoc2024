use std::collections::HashMap;
use std::hash::Hash;

pub trait Countable {
    type Item;

    fn counts(&mut self) -> HashMap<Self::Item, usize>;
}

impl<T, I> Countable for I
where
    T: Eq + Hash,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn counts(&mut self) -> HashMap<Self::Item, usize> {
        let mut counts = HashMap::new();
        for item in self {
            *counts.entry(item).or_default() += 1;
        }
        counts
    }
}
